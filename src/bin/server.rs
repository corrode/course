use cargo_course::types::*;

use anyhow::Result;
use askama::Template;
use axum::{
    debug_handler, extract::{Path as AxumPath, Query, State}, http::StatusCode, response::{Html, IntoResponse, Json}, routing::{get, post}, Router
};
use dotenvy::dotenv;
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
}

/// Template for admin dashboard
#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate {
    participants: Vec<ParticipantSummary>,
    recent_submissions: Vec<SubmissionSummary>,
}

/// Exercise progress for dashboard display
#[derive(Serialize, Clone)]
struct ExerciseProgress {
    name: String,
    completed: bool,
    perfected: bool,
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
    // Load environment variables
    dotenv().ok();

    let admin_token =
        env::var("CORRODE_ADMIN_TOKEN").expect("CORRODE_ADMIN_TOKEN must be set in .env file");
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Create the database in the current working directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let db_path = current_dir.join("playground.db");
        println!("📁 Database will be created at: {}", db_path.display());
        format!("sqlite:{}", db_path.display())
    });
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Set up database
    println!("🔍 Checking if database exists: {}", database_url);
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        println!("📦 Creating database {}", database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("✅ Database created successfully"),
            Err(error) => panic!("❌ Error creating database: {}", error),
        }
    } else {
        println!("✅ Database already exists");
    }

    println!("🔌 Connecting to database...");
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .map_err(|e| {
            eprintln!("❌ Failed to connect to database: {}", e);
            eprintln!("   Database URL: {}", database_url);
            e
        })?;
    
    println!("✅ Database connection established");

    // Run migrations
    let migrations = std::path::Path::new("./migrations");
    let migrator = sqlx::migrate::Migrator::new(migrations).await?;
    migrator.run(&pool).await?;
    println!("✅ Database migrations completed");

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
    println!("🚀 Server running on http://localhost:{port}");
    println!("📊 Admin dashboard: http://localhost:{port}/admin?token={admin_token}");
    println!("🗃️  Database: {}", database_url);

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

    let template = DashboardTemplate {
        participant_name: participant.name,
        exercises,
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

    let template = AdminTemplate {
        participants,
        recent_submissions,
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

    match sqlx::query("INSERT INTO participants (id, name) VALUES (?, ?)")
        .bind(&ulid)
        .bind(request.name.as_str())
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(RegistrationResponse { ulid })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// API submission endpoint
#[debug_handler]
async fn api_submit(
    State(state): State<AppState>,
    Json(request): Json<SubmissionRequest>,
) -> Result<StatusCode, StatusCode> {
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
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// API status endpoint
#[debug_handler]
async fn api_status(
    AxumPath(ulid): AxumPath<String>,
    State(state): State<AppState>,
) -> Result<Json<ProgressResponse>, StatusCode> {
    match get_exercise_progress(&state.pool, &ulid).await {
        Ok(exercises) => {
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
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get exercise progress for a participant
async fn get_exercise_progress<'a>(pool: &'a SqlitePool, ulid: &'a str) -> Result<Vec<ExerciseProgress>> {
    // Hard-coded exercise list for now - TODO: scan examples/ directory
    let all_exercises = vec![
        "00_hello_rust",
        "01_integer_handling",
        "02_enums_and_matching",
        "03_vectors_basics",
        "04_hashmaps",
        "05_tuples",
        "06_option_handling",
        "07_result_handling",
        "08_ownership_basics",
        "09_structs_and_methods",
        "10_iterator_patterns",
        "11_word_counter",
        "12_env_parser",
        "13_csv_parser",
        "14_password_validator",
    ];

    // Get completed submissions
    let submissions: Vec<DbSubmission> =
        sqlx::query_as("SELECT * FROM submissions WHERE participant_id = ? AND tests_passed = 1")
            .bind(ulid)
            .fetch_all(pool)
            .await?;

    let mut exercises = Vec::new();
    for exercise_name in all_exercises {
        let submission = submissions
            .iter()
            .find(|s| s.exercise_name == exercise_name);

        exercises.push(ExerciseProgress {
            name: exercise_name.to_string(),
            completed: submission.is_some(),
            perfected: submission.map_or(false, |s| s.fmt_passed && s.clippy_passed),
        });
    }

    Ok(exercises)
}
