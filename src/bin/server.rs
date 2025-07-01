use cargo_course::types::*;

use anyhow::Result;
use askama::Template;
use axum::{
    debug_handler, extract::{Path as AxumPath, Query, State}, http::StatusCode, response::{Html, IntoResponse, Json}, routing::{get, post}, Router
};
use dotenvy::dotenv;
use log::{info, warn, error};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, SqlitePool, Row, Sqlite};
use std::env;
use tower_http::services::ServeDir;
use ulid::Ulid;

/// Application state shared across all routes
#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    admin_token: String,
}

/// Database model for participants
#[derive(sqlx::FromRow)]
struct DbParticipant {
    id: String,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Database model for submissions
#[derive(sqlx::FromRow)]
struct DbSubmission {
    id: String,
    participant_id: String,
    exercise_name: String,
    source_code: String,
    tests_passed: bool,
    clippy_passed: bool,
    fmt_passed: bool,
    submitted_at: chrono::DateTime<chrono::Utc>,
}

/// Template for the landing page
#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate;

/// Template for participant dashboard
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    participant_name: String,
    exercises: Vec<ExerciseProgress>,
    stats: UserStats,
}

/// User dashboard statistics
#[derive(Serialize)]
struct UserStats {
    completed_count: i64,
    perfected_count: i64,
    total_submissions: i64,
}

/// Template for admin dashboard
#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate {
    participants: Vec<ParticipantSummary>,
    recent_submissions: Vec<SubmissionSummary>,
    stats: AdminStats,
}

/// Admin dashboard statistics
#[derive(Serialize)]
struct AdminStats {
    total_participants: i64,
    total_submissions: i64,
    total_perfected: i64,
}

/// Exercise progress for dashboard display
#[derive(Serialize, Clone)]
struct ExerciseProgress {
    name: String,
    completed: bool,
    perfected: bool,
    title: String,
    description: String,
}

/// Participant summary for admin view
#[derive(Serialize)]
struct ParticipantSummary {
    id: String,
    name: String,
    completed_count: i64,
    total_exercises: i64,
    last_activity: Option<chrono::DateTime<chrono::Utc>>,
}

/// Submission summary for admin view
#[derive(Serialize)]
struct SubmissionSummary {
    participant_name: String,
    exercise_name: String,
    tests_passed: bool,
    perfected: bool,
    submitted_at: chrono::DateTime<chrono::Utc>,
    source_code: String,
}

/// Query parameters for admin access
#[derive(Deserialize)]
struct AdminQuery {
    token: String,
}

/// Form data for web registration
#[derive(Deserialize)]
struct WebRegistrationForm {
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with info level by default
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    
    // Load environment variables
    dotenv().ok();
    
    info!("Starting corrode course server...");

    let admin_token =
        env::var("CORRODE_ADMIN_TOKEN").expect("CORRODE_ADMIN_TOKEN must be set in .env file");
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Create the database in the current working directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let db_path = current_dir.join("playground.db");
        info!("📁 Database will be created at: {}", db_path.display());
        format!("sqlite:{}", db_path.display())
    });
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Set up database
    info!("Checking if database exists: {}", database_url);
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        info!("Creating database {}", database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => {
                println!("✅ Database created successfully");
                info!("Database created successfully");
            }
            Err(error) => {
                error!("Error creating database: {}", error);
                panic!("❌ Error creating database: {}", error);
            }
        }
    } else {
        info!("Database already exists");
    }

    info!("Connecting to database...");
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .map_err(|e| {
            eprintln!("❌ Failed to connect to database: {}", e);
            eprintln!("   Database URL: {}", database_url);
            error!("Failed to connect to database: {}", e);
            error!("Database URL: {}", database_url);
            e
        })?;
    
    info!("Database connection established");

    // Run migrations
    let migrations = std::path::Path::new("./migrations");
    let migrator = sqlx::migrate::Migrator::new(migrations).await?;
    migrator.run(&pool).await?;
    info!("Database migrations completed");

    let app_state = AppState { pool, admin_token: admin_token.clone() };

    // Build API routes
    let api_routes = Router::new()
        .route("/register", post(api_register))
        .route("/submit", post(api_submit))
        .route("/status/{ulid}", get(api_status))
        .with_state(app_state.clone());

    // Build main routes
    let app = Router::new()
        .route("/", get(landing_page))
        .route("/register", post(web_register))
        .route("/dashboard/{ulid}", get(participant_dashboard))
        .route("/admin", get(admin_dashboard))
        .nest("/api", api_routes)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await?;
    info!("🚀 Server running on http://localhost:{port}");
    info!("📊 Admin dashboard: http://localhost:{port}/admin?token={admin_token}");
    info!("🗃️  Database: {database_url}");

    axum::serve(listener, app).await?;
    Ok(())
}


/// Landing page handler
async fn landing_page() -> impl IntoResponse {
    let template = LandingTemplate;
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("Error rendering template".to_string()),
    }
}

/// Web registration handler
async fn web_register(
    State(state): State<AppState>,
    axum::Form(form): axum::Form<WebRegistrationForm>,
) -> Result<axum::response::Redirect, StatusCode> {
    let name = Name::try_from(form.name).map_err(|_| StatusCode::BAD_REQUEST)?;

    let ulid = Ulid::new().to_string();

    sqlx::query("INSERT INTO participants (id, name) VALUES (?, ?)")
        .bind(&ulid)
        .bind(name.as_str())
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(axum::response::Redirect::to(&format!("/dashboard/{ulid}")))
}

/// Participant dashboard handler
async fn participant_dashboard(
    AxumPath(ulid): AxumPath<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Get participant info
    let participant_result = sqlx::query_as("SELECT id, name, created_at FROM participants WHERE id = ?")
        .bind(&ulid)
        .fetch_one(&state.pool)
        .await;

    let participant: DbParticipant = match participant_result {
        Ok(p) => p,
        Err(_) => return (StatusCode::NOT_FOUND, "Participant not found").into_response(),
    };

    // Get exercise progress
    let exercises = match get_exercise_progress(&state.pool, &ulid).await {
        Ok(exercises) => exercises,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get exercise progress").into_response(),
    };

    // Get user statistics
    let stats = match get_user_stats(&state.pool, &ulid).await {
        Ok(stats) => stats,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user statistics").into_response(),
    };

    let template = DashboardTemplate {
        participant_name: participant.name,
        exercises,
        stats,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render template").into_response(),
    }
}

/// Admin dashboard handler
async fn admin_dashboard(
    Query(query): Query<AdminQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Verify admin token
    if query.token != state.admin_token {
        return (StatusCode::FORBIDDEN, "Invalid admin token").into_response();
    }

    // Get participant summaries
    let participant_rows_result = sqlx::query(
        r#"
        SELECT 
            p.id,
            p.name,
            COUNT(s.id) as completed_count,
            15 as total_exercises,
            MAX(s.submitted_at) as last_activity
        FROM participants p
        LEFT JOIN submissions s ON p.id = s.participant_id AND s.tests_passed = 1
        GROUP BY p.id, p.name
        ORDER BY last_activity DESC NULLS LAST
        "#,
    )
    .fetch_all(&state.pool)
    .await;

    let participant_rows = match participant_rows_result {
        Ok(rows) => rows,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch participants").into_response(),
    };

    let mut participants = Vec::new();
    for row in participant_rows {
        participants.push(ParticipantSummary {
            id: row.get("id"),
            name: row.get("name"),
            completed_count: row.get("completed_count"),
            total_exercises: row.get("total_exercises"),
            last_activity: row.get("last_activity"),
        });
    }

    // Get recent submissions
    let submission_rows_result = sqlx::query(
        r#"
        SELECT 
            p.name as participant_name,
            s.exercise_name,
            s.tests_passed,
            s.fmt_passed,
            s.clippy_passed,
            s.submitted_at,
            s.source_code
        FROM submissions s
        JOIN participants p ON s.participant_id = p.id
        ORDER BY s.submitted_at DESC
        LIMIT 20
        "#,
    )
    .fetch_all(&state.pool)
    .await;

    let submission_rows = match submission_rows_result {
        Ok(rows) => rows,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch submissions").into_response(),
    };

    let mut recent_submissions = Vec::new();
    for row in submission_rows {
        let fmt_passed: bool = row.get("fmt_passed");
        let clippy_passed: bool = row.get("clippy_passed");
        recent_submissions.push(SubmissionSummary {
            participant_name: row.get("participant_name"),
            exercise_name: row.get("exercise_name"),
            tests_passed: row.get("tests_passed"),
            perfected: fmt_passed && clippy_passed,
            submitted_at: row.get("submitted_at"),
            source_code: row.get("source_code"),
        });
    }

    // Get admin statistics with proper SQL queries
    let stats = match get_admin_stats(&state.pool).await {
        Ok(stats) => stats,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get admin statistics").into_response(),
    };

    let template = AdminTemplate {
        participants,
        recent_submissions,
        stats,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render template").into_response(),
    }
}

/// API registration endpoint
#[debug_handler]
async fn api_register(
    State(state): State<AppState>,
    Json(request): Json<RegistrationRequest>,
) -> Result<Json<RegistrationResponse>, StatusCode> {
    let ulid = Ulid::new().to_string();
    
    info!("New participant registration: name='{}', ulid='{}'", request.name.as_str(), ulid);

    match sqlx::query("INSERT INTO participants (id, name) VALUES (?, ?)")
        .bind(&ulid)
        .bind(request.name.as_str())
        .execute(&state.pool)
        .await
    {
        Ok(_) => {
            info!("Participant registered successfully: {}", ulid);
            Ok(Json(RegistrationResponse { ulid }))
        },
        Err(e) => {
            error!("Failed to register participant: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// API submission endpoint
#[debug_handler]
async fn api_submit(
    State(state): State<AppState>,
    Json(request): Json<SubmissionRequest>,
) -> Result<StatusCode, StatusCode> {
    info!("Submission attempt: participant_id='{}', exercise='{}'", request.ulid, request.exercise_name);
    
    // First, check if the participant exists
    let participant_exists = sqlx::query("SELECT 1 FROM participants WHERE id = ?")
        .bind(&request.ulid)
        .fetch_optional(&state.pool)
        .await;
        
    match participant_exists {
        Ok(Some(_)) => {
            // Participant exists, proceed with submission
            info!("Valid participant found for submission: {}", request.ulid);
        },
        Ok(None) => {
            warn!("Submission attempt with invalid participant ID: {}", request.ulid);
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(e) => {
            error!("Database error while checking participant: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    // Upsert submission (replace if exists)
    match sqlx::query(
        r#"
        INSERT INTO submissions (id, participant_id, exercise_name, source_code, tests_passed, clippy_passed, fmt_passed)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(participant_id, exercise_name) DO UPDATE SET
            id = excluded.id,
            source_code = excluded.source_code,
            tests_passed = excluded.tests_passed,
            clippy_passed = excluded.clippy_passed,
            fmt_passed = excluded.fmt_passed,
            submitted_at = CURRENT_TIMESTAMP
        "#
    )
    .bind(Ulid::new().to_string())
    .bind(&request.ulid)
    .bind(&request.exercise_name)
    .bind(&request.source_code)
    .bind(request.tests_passed)
    .bind(request.clippy_passed)
    .bind(request.fmt_passed)
    .execute(&state.pool)
    .await 
    {
        Ok(_) => {
            info!("Submission successful: participant='{}', exercise='{}', tests_passed={}", 
                  request.ulid, request.exercise_name, request.tests_passed);
            Ok(StatusCode::OK)
        },
        Err(e) => {
            error!("Failed to save submission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// API status endpoint
#[debug_handler]
async fn api_status(
    AxumPath(ulid): AxumPath<String>,
    State(state): State<AppState>,
) -> Result<Json<ProgressResponse>, StatusCode> {
    info!("Status request for participant: {}", ulid);
    
    match get_exercise_progress(&state.pool, &ulid).await {
        Ok(exercises) => {
            let completed_count = exercises.iter().filter(|e| e.completed).count();
            let perfected_count = exercises.iter().filter(|e| e.perfected).count();
            info!("Status response: participant='{}', completed={}, perfected={}", 
                  ulid, completed_count, perfected_count);
            
            let exercise_statuses = exercises
                .into_iter()
                .map(|e| ExerciseStatus {
                    name: e.name,
                    completed: e.completed,
                    perfected: e.perfected,
                })
                .collect();

            Ok(Json(ProgressResponse {
                exercises: exercise_statuses,
            }))
        }
        Err(e) => {
            error!("Failed to get exercise progress for {}: {}", ulid, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

/// Get user statistics for a specific participant
async fn get_user_stats(pool: &SqlitePool, participant_id: &str) -> Result<UserStats> {
    // Get completed submissions count
    let completed_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM submissions WHERE participant_id = ? AND tests_passed = 1"
    )
    .bind(participant_id)
    .fetch_one(pool)
    .await?;

    // Get perfected submissions count  
    let perfected_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM submissions WHERE participant_id = ? AND tests_passed = 1 AND fmt_passed = 1 AND clippy_passed = 1"
    )
    .bind(participant_id)
    .fetch_one(pool)
    .await?;

    // Get total submissions count (including failed ones)
    let total_submissions: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM submissions WHERE participant_id = ?"
    )
    .bind(participant_id)
    .fetch_one(pool)
    .await?;

    Ok(UserStats {
        completed_count,
        perfected_count,
        total_submissions,
    })
}

/// Get admin statistics
async fn get_admin_stats(pool: &SqlitePool) -> Result<AdminStats> {
    // Get total participants
    let total_participants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM participants")
        .fetch_one(pool)
        .await?;

    // Get total submissions across all participants
    let total_submissions: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM submissions WHERE tests_passed = 1"
    )
    .fetch_one(pool)
    .await?;

    // Get total perfected submissions (successful + fmt + clippy)
    let total_perfected: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM submissions WHERE tests_passed = 1 AND fmt_passed = 1 AND clippy_passed = 1"
    )
    .fetch_one(pool)
    .await?;


    Ok(AdminStats {
        total_participants,
        total_submissions,
        total_perfected,
    })
}

/// Get exercise progress for a participant
async fn get_exercise_progress<'a>(pool: &'a SqlitePool, ulid: &'a str) -> Result<Vec<ExerciseProgress>> {
    // Scan the examples directory for exercise files
    let examples_dir = std::path::Path::new("examples");
    if !examples_dir.exists() {
        return Err(anyhow::anyhow!("Examples directory not found"));
    }

    let mut exercise_files = Vec::new();
    for entry in std::fs::read_dir(examples_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.ends_with(".rs") && !file_name.starts_with("_") {
                exercise_files.push(file_name.to_string());
            }
        }
    }
    
    // Sort files for consistent ordering
    exercise_files.sort();

    // Get completed submissions
    let submissions: Vec<DbSubmission> =
        sqlx::query_as("SELECT * FROM submissions WHERE participant_id = ? AND tests_passed = 1")
            .bind(ulid)
            .fetch_all(pool)
            .await?;

    let mut exercises = Vec::new();
    for file_name in exercise_files {
        let exercise_name = file_name.strip_suffix(".rs").unwrap_or(&file_name);
        
        // Parse the exercise documentation
        let (title, description) = parse_exercise_docs(&format!("examples/{}", file_name))
            .unwrap_or_else(|_| (exercise_name.to_string(), String::new()));
        
        let submission = submissions
            .iter()
            .find(|s| s.exercise_name == exercise_name);

        exercises.push(ExerciseProgress {
            name: exercise_name.to_string(),
            completed: submission.is_some(),
            perfected: submission.map_or(false, |s| s.fmt_passed && s.clippy_passed),
            title,
            description,
        });
    }

    Ok(exercises)
}

/// Parse exercise documentation from Rust file doc comments
fn parse_exercise_docs(file_path: &str) -> Result<(String, String)> {
    let content = std::fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    let mut title = String::new();
    let mut description_lines = Vec::new();
    let mut in_doc_comment = false;
    let mut found_title = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        if trimmed.starts_with("//!") {
            in_doc_comment = true;
            let doc_content = trimmed.strip_prefix("//!").unwrap_or("").trim();
            
            if !found_title && doc_content.starts_with("# ") {
                // Extract title from # heading
                title = doc_content.strip_prefix("# ").unwrap_or(doc_content).to_string();
                found_title = true;
            } else if found_title && !doc_content.is_empty() {
                // Collect description lines after title
                description_lines.push(doc_content.to_string());
            }
        } else if in_doc_comment && !trimmed.starts_with("//") {
            // End of doc comment block
            break;
        }
    }
    
    // Join description lines and clean up
    let description = description_lines
        .join(" ")
        .trim()
        .to_string();
    
    // Fallback title if none found
    if title.is_empty() {
        title = file_path.strip_prefix("examples/")
            .unwrap_or(file_path)
            .strip_suffix(".rs")
            .unwrap_or(file_path)
            .to_string();
    }
    
    Ok((title, description))
}
