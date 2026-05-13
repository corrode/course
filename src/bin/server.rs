use cargo_course::exercises::{self, Exercise, RenderItem, RenderKind, Step};
use cargo_course::types::*;

use anyhow::Result;
use askama::Template;
use axum::{
    Router, debug_handler,
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{delete, get, post},
};
use dotenvy::dotenv;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{Row, Sqlite, SqlitePool, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};
use std::env;
use std::sync::Arc;
use tower_http::services::ServeDir;
use ulid::Ulid;

/// Application state shared across all routes
#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    admin_token: String,
    exercises: Arc<Vec<Exercise>>,
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

/// Template for the standalone playground (scratchpad) page.
#[derive(Template)]
#[template(path = "playground.html")]
struct PlaygroundTemplate {
    /// Initial source shown before the user has typed anything.
    starter: String,
}

/// Template for the cheatsheet page. Just renders pre-built HTML
/// produced from `docs/cheatsheet.md` at startup.
#[derive(Template)]
#[template(path = "cheatsheet.html")]
struct CheatsheetTemplate {
    html: String,
}

/// Template for an exercise page
#[derive(Template)]
#[template(path = "exercise.html")]
struct ExerciseTemplate {
    exercise: Exercise,
    /// `Some` when the page is rendered with participant context.
    ulid: Option<String>,
    /// Rollup status for the chapter as a whole (all code steps).
    /// All `false` on the public route.
    current_status: UiExerciseStatus,
    /// One entry per exercise in the catalog, ordered by `number`.
    /// Used to render the bottom "chapter list" navigation.
    dots: Vec<ProgressDot>,
    /// Ordered render plan: prose blocks and code sections in display
    /// order. Each `Code` carries the per-step status and the database
    /// key (`<chapter>/<step_key>` or just `<chapter>` for legacy).
    items: Vec<RenderItem>,
}

/// One row in the bottom chapter list.
#[derive(Clone)]
struct ProgressDot {
    slug: String,
    number: u8,
    title: String,
    /// Has at least one submission (regardless of pass/fail).
    attempted: bool,
    /// `tests_passed` on at least one submission.
    completed: bool,
    /// Tests + fmt + clippy all green on the same submission.
    perfected: bool,
    current: bool,
    is_quiz: bool,
}

/// Per-exercise progress used by the chapter list and current-status badge.
#[derive(Clone, Default)]
struct UiExerciseStatus {
    attempted: bool,
    completed: bool,
    perfected: bool,
}

/// Template for participant dashboard
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    participant_name: String,
    ulid: String,
    exercises: Vec<ExerciseProgress>,
    /// Slug of the first chapter the participant hasn't completed yet,
    /// or the first chapter overall if they're brand new / fully done.
    /// Used by the "Start" call-to-action.
    next_slug: String,
    /// Display label for the CTA ("Start with chapter 1" or
    /// "Resume chapter 5" depending on whether anything has been
    /// completed yet).
    next_label: String,
}

/// Template for admin dashboard
#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate {
    participants: Vec<ParticipantSummary>,
    recent_submissions: Vec<SubmissionSummary>,
    stats: AdminStats,
    exercises: Vec<String>,
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
    /// Display number, 1-based (chapter 0 on disk -> `1.` in the TOC).
    number: u8,
    name: String,
    completed: bool,
    perfected: bool,
    title: String,
    description: String,
    submissions: Vec<ExerciseSubmission>,
    is_quiz: bool,
}

/// Individual submission for an exercise
#[derive(Serialize, Clone)]
struct ExerciseSubmission {
    id: String,
    source_code: String,
    tests_passed: bool,
    clippy_passed: bool,
    fmt_passed: bool,
    submitted_at: chrono::DateTime<chrono::Utc>,
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
        let db_path = current_dir.join("course.db");
        info!("📁 Database will be created at: {}", db_path.display());
        format!("sqlite:{}", db_path.display())
    });
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Set up database
    info!("Checking if database exists: {}", database_url);
    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
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

    // Scan exercises once at startup.
    let exercises = exercises::load(std::path::Path::new("examples")).map_err(|e| {
        error!("Failed to scan exercises: {e:#}");
        e
    })?;
    info!("Loaded {} exercises", exercises.len());

    let app_state = AppState {
        pool,
        admin_token: admin_token.clone(),
        exercises,
    };

    // Build API routes
    let api_routes = Router::new()
        .route("/register", post(api_register))
        .route("/submit", post(api_submit))
        .route("/status/{ulid}", get(api_status))
        .route("/run", post(api_run))
        .route("/format", post(api_format))
        .with_state(app_state.clone());

    // Build main routes
    let app = Router::new()
        .route("/", get(landing_page))
        .route("/register", post(web_register))
        .route("/dashboard/{ulid}", get(participant_dashboard))
        .route("/exercise/{slug}", get(public_exercise_page))
        .route("/exercise/{ulid}/{slug}", get(participant_exercise_page))
        .route("/playground", get(playground_page))
        .route("/cheatsheet", get(cheatsheet_page))
        .route("/cheatsheet/fragment", get(cheatsheet_fragment))
        .route("/health", get(health_check))
        .route("/admin", get(admin_dashboard))
        .route(
            "/admin/remove-participant/{ulid}",
            delete(admin_remove_participant),
        )
        .nest("/api", api_routes)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await?;
    info!("🚀 Server listening on 0.0.0.0:{port} (open http://localhost:{port} locally)");
    info!("📊 Admin dashboard path: /admin?token={admin_token}");
    info!("🗃️  Database: {database_url}");

    axum::serve(listener, app).await?;
    Ok(())
}

/// Liveness + readiness probe.
///
/// Returns 200 if the process is up *and* the database accepts a
/// trivial query, 503 otherwise. Cheap enough to hit every few seconds
/// from uptime monitors.
async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(e) => {
            error!("health check failed: {e}");
            (StatusCode::SERVICE_UNAVAILABLE, "db unavailable")
        }
    }
}

/// Landing page handler
async fn landing_page() -> impl IntoResponse {
    let template = LandingTemplate;
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("Error rendering template".to_string()),
    }
}

/// Standalone Rust scratchpad. Code is persisted client-side in
/// `localStorage`; this handler only ships the starter snippet.
async fn playground_page() -> impl IntoResponse {
    const STARTER: &str = r#"//! Playground scratchpad.
//!
//! Anything you type here is saved to your browser's local storage and
//! survives reloads. Press "Run" to compile and execute on
//! play.rust-lang.org.
//!
//! Tip: delete this comment block once you don't need the reminder.

fn main() {
    println!("Hello from the playground!");
}
"#;
    let template = PlaygroundTemplate {
        starter: STARTER.to_string(),
    };
    match template.render() {
        Ok(html) => Html(html),
        Err(e) => {
            error!("playground template render failed: {e}");
            Html("Error rendering template".to_string())
        }
    }
}

/// Renders `docs/cheatsheet.md` as a standalone reference page.
async fn cheatsheet_page() -> impl IntoResponse {
    let md = match std::fs::read_to_string("docs/cheatsheet.md") {
        Ok(s) => s,
        Err(e) => {
            warn!("cheatsheet markdown missing: {e}");
            "# Cheatsheet\n\n_Cheatsheet not found._".to_string()
        }
    };
    let html = exercises::render_markdown(&md);
    let template = CheatsheetTemplate { html };
    match template.render() {
        Ok(html) => Html(html),
        Err(e) => {
            error!("cheatsheet template render failed: {e}");
            Html("Error rendering template".to_string())
        }
    }
}

/// Returns the rendered cheatsheet markdown as a bare HTML fragment.
/// Used by the in-page modal so we don't have to ship the entire
/// document with every page load.
async fn cheatsheet_fragment() -> impl IntoResponse {
    let md = match std::fs::read_to_string("docs/cheatsheet.md") {
        Ok(s) => s,
        Err(e) => {
            warn!("cheatsheet markdown missing: {e}");
            "# Cheatsheet\n\n_Cheatsheet not found._".to_string()
        }
    };
    Html(exercises::render_markdown(&md))
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
    let participant_result =
        sqlx::query_as("SELECT id, name, created_at FROM participants WHERE id = ?")
            .bind(&ulid)
            .fetch_one(&state.pool)
            .await;

    let participant: DbParticipant = match participant_result {
        Ok(p) => p,
        Err(_) => return (StatusCode::NOT_FOUND, "Participant not found").into_response(),
    };

    // Get exercise progress
    let exercises = match get_exercise_progress(&state.pool, &ulid, &state.exercises).await {
        Ok(exercises) => exercises,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get exercise progress",
            )
                .into_response();
        }
    };

    // CTA: jump to the first unfinished, non-quiz chapter. If everything
    // is done, point back to chapter 1 as a graceful default.
    let first_unfinished = exercises
        .iter()
        .find(|e| !e.completed && !e.is_quiz)
        .or_else(|| exercises.first());
    let any_completed = exercises.iter().any(|e| e.completed);
    let (next_slug, next_label) = match first_unfinished {
        Some(e) if any_completed => (e.name.clone(), format!("Resume chapter {}", e.number)),
        Some(e) => (e.name.clone(), format!("Start with chapter {}", e.number)),
        None => (String::new(), "Start the course".to_string()),
    };

    let template = DashboardTemplate {
        participant_name: participant.name,
        ulid: ulid.clone(),
        exercises,
        next_slug,
        next_label,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to render template",
        )
            .into_response(),
    }
}

/// Public exercise page (no participant context).
async fn public_exercise_page(
    AxumPath(slug): AxumPath<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    render_exercise_page(&state, &slug, None).await
}

/// Exercise page with participant context.
async fn participant_exercise_page(
    AxumPath((ulid, slug)): AxumPath<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Verify the participant exists; if not, fall back to the public page so
    // the URL still resolves to something useful.
    let exists = sqlx::query("SELECT 1 FROM participants WHERE id = ?")
        .bind(&ulid)
        .fetch_optional(&state.pool)
        .await
        .ok()
        .flatten()
        .is_some();

    let ulid = if exists { Some(ulid) } else { None };
    render_exercise_page(&state, &slug, ulid).await
}

/// Minimal HTML escape for arbitrary text we splice into a template.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

async fn render_exercise_page(
    state: &AppState,
    slug: &str,
    ulid: Option<String>,
) -> axum::response::Response {
    // Look up by slug or by file_stem so both `/exercise/strings_and_chars`
    // and `/exercise/02_strings_and_chars` resolve.
    let Some(idx) = state
        .exercises
        .iter()
        .position(|e| e.slug == slug || e.file_stem == slug)
    else {
        return (StatusCode::NOT_FOUND, "Exercise not found").into_response();
    };
    let exercise = state.exercises[idx].clone();

    // Per-chapter rollup status for the header badge and chapter list.
    let chapter_progress: std::collections::HashMap<String, UiExerciseStatus> = match &ulid {
        Some(u) => match get_exercise_progress(&state.pool, u, &state.exercises).await {
            Ok(rows) => rows
                .into_iter()
                .map(|r| {
                    (
                        r.name,
                        UiExerciseStatus {
                            attempted: !r.submissions.is_empty(),
                            completed: r.completed,
                            perfected: r.perfected,
                        },
                    )
                })
                .collect(),
            Err(e) => {
                warn!("Failed to load progress for header dots: {e}");
                std::collections::HashMap::new()
            }
        },
        None => std::collections::HashMap::new(),
    };

    // Per-step status (`<chapter>/<step_key>`). For legacy single-step
    // chapters the step status is just the chapter status.
    let step_progress: std::collections::HashMap<String, UiExerciseStatus> = match &ulid {
        Some(u) => load_step_progress(&state.pool, u)
            .await
            .unwrap_or_else(|e| {
                warn!("Failed to load per-step progress: {e}");
                std::collections::HashMap::new()
            }),
        None => std::collections::HashMap::new(),
    };

    let current_status = chapter_progress
        .get(&exercise.file_stem)
        .cloned()
        .unwrap_or_default();

    let dots: Vec<ProgressDot> = state
        .exercises
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let s = chapter_progress
                .get(&e.file_stem)
                .cloned()
                .unwrap_or_default();
            ProgressDot {
                slug: e.slug.clone(),
                number: e.number,
                title: e.title.clone(),
                attempted: s.attempted,
                completed: s.completed,
                perfected: s.perfected,
                current: i == idx,
                is_quiz: e.is_quiz(),
            }
        })
        .collect();

    // Build the ordered render plan from the chapter's steps.
    let total_code_steps = exercise.code_steps().len();
    let mut current_code_index: usize = 0;
    let mut seen_first_note = false;
    let mut prev_was_note = false;
    let mut items: Vec<RenderItem> = Vec::with_capacity(exercise.steps.len());
    for step in &exercise.steps {
        match step {
            Step::Prose(note) => {
                // The first note's title is also the chapter title (rendered
                // as the page <h1>), so we only show the title on subsequent
                // notes. Otherwise the heading would appear twice on chapter
                // index pages.
                let html = if seen_first_note {
                    format!(
                        "<h2 class=\"note-title\">{}</h2>\n{}",
                        html_escape(&note.title),
                        note.html
                    )
                } else {
                    note.html.clone()
                };
                seen_first_note = true;
                prev_was_note = true;
                items.push(RenderItem {
                    kind: RenderKind::Prose { html },
                });
            }
            Step::Code(code) => {
                current_code_index += 1;
                let step_key = code.key();
                let exercise_key = if step_key.is_empty() {
                    exercise.file_stem.clone()
                } else {
                    format!("{}/{}", exercise.file_stem, step_key)
                };
                let dom_id = if step_key.is_empty() {
                    exercise.file_stem.clone()
                } else {
                    format!("{}__{}", exercise.file_stem, step_key)
                };
                let eyebrow = if total_code_steps > 1 {
                    format!("Exercise {current_code_index} of {total_code_steps}")
                } else {
                    "Exercise".to_string()
                };
                let filename = if step_key.is_empty() {
                    "main.rs".to_string()
                } else {
                    format!("{step_key}.rs")
                };
                let github_dev_url = format!(
                    "https://github.dev/corrode/course/blob/main/examples/{}/{}",
                    exercise.file_stem, filename
                );
                let status = step_progress
                    .get(&exercise_key)
                    .cloned()
                    .unwrap_or_default();
                items.push(RenderItem {
                    kind: RenderKind::Code {
                        dom_id,
                        exercise_key,
                        eyebrow,
                        title: code.title.clone(),
                        show_title: !prev_was_note,
                        starter_code: code.starter_code.clone(),
                        attempted: status.attempted,
                        completed: status.completed,
                        perfected: status.perfected,
                        github_dev_url,
                        hints_html: code.hints_html.clone(),
                    },
                });
                prev_was_note = false;
            }
        }
    }

    let template = ExerciseTemplate {
        exercise,
        ulid,
        current_status,
        dots,
        items,
    };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            error!("Failed to render exercise template: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response()
        }
    }
}

/// Load per-step submission status for a participant.
///
/// Keys are the full `submissions.exercise_name` value (`<chapter>` for
/// legacy single-step chapters or `<chapter>/<step_key>` for multi-step).
async fn load_step_progress(
    pool: &SqlitePool,
    ulid: &str,
) -> Result<std::collections::HashMap<String, UiExerciseStatus>> {
    let rows: Vec<DbSubmission> = sqlx::query_as(
        "SELECT * FROM submissions WHERE participant_id = ? ORDER BY exercise_name, submitted_at DESC",
    )
    .bind(ulid)
    .fetch_all(pool)
    .await?;

    let mut by_key: std::collections::HashMap<String, UiExerciseStatus> =
        std::collections::HashMap::new();
    for row in rows {
        let entry = by_key.entry(row.exercise_name.clone()).or_default();
        entry.attempted = true;
        if row.tests_passed {
            entry.completed = true;
        }
        if row.tests_passed && row.fmt_passed && row.clippy_passed {
            entry.perfected = true;
        }
    }
    Ok(by_key)
}
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
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch participants",
            )
                .into_response();
        }
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
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch submissions",
            )
                .into_response();
        }
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
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get admin statistics",
            )
                .into_response();
        }
    };

    // Get unique exercises from submissions
    let exercise_rows_result =
        sqlx::query("SELECT DISTINCT exercise_name FROM submissions ORDER BY exercise_name")
            .fetch_all(&state.pool)
            .await;

    let exercises = match exercise_rows_result {
        Ok(rows) => rows
            .iter()
            .map(|row| row.get::<String, _>("exercise_name"))
            .collect(),
        Err(_) => Vec::new(), // Return empty list if query fails
    };

    let template = AdminTemplate {
        participants,
        recent_submissions,
        stats,
        exercises,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to render template",
        )
            .into_response(),
    }
}

/// Admin participant removal endpoint
async fn admin_remove_participant(
    AxumPath(participant_id): AxumPath<String>,
    Query(query): Query<AdminQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Verify admin token
    if query.token != state.admin_token {
        return (StatusCode::FORBIDDEN, "Invalid admin token").into_response();
    }

    info!("Admin removing participant: {}", participant_id);

    // Start a transaction to ensure atomicity
    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!("Failed to start transaction: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // First, delete all submissions for this participant
    if let Err(err) = sqlx::query("DELETE FROM submissions WHERE participant_id = ?")
        .bind(&participant_id)
        .execute(&mut *tx)
        .await
    {
        error!(
            "Failed to delete submissions for participant {}: {}",
            participant_id, err
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete submissions",
        )
            .into_response();
    }

    // Then, delete the participant
    match sqlx::query("DELETE FROM participants WHERE id = ?")
        .bind(&participant_id)
        .execute(&mut *tx)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return (StatusCode::NOT_FOUND, "Participant not found").into_response();
            }
        }
        Err(err) => {
            error!("Failed to delete participant {}: {}", participant_id, err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete participant",
            )
                .into_response();
        }
    }

    // Commit the transaction
    if let Err(err) = tx.commit().await {
        error!("Failed to commit transaction: {}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    info!("Successfully removed participant: {}", participant_id);
    (StatusCode::OK, "Participant removed successfully").into_response()
}

/// API registration endpoint
#[debug_handler]
async fn api_register(
    State(state): State<AppState>,
    Json(request): Json<RegistrationRequest>,
) -> Result<Json<RegistrationResponse>, StatusCode> {
    let ulid = Ulid::new().to_string();

    info!(
        "New participant registration: name='{}', ulid='{}'",
        request.name.as_str(),
        ulid
    );

    match sqlx::query("INSERT INTO participants (id, name) VALUES (?, ?)")
        .bind(&ulid)
        .bind(request.name.as_str())
        .execute(&state.pool)
        .await
    {
        Ok(_) => {
            info!("Participant registered successfully: {}", ulid);
            Ok(Json(RegistrationResponse { ulid }))
        }
        Err(e) => {
            error!("Failed to register participant: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// API submission endpoint
#[debug_handler]
async fn api_submit(
    State(state): State<AppState>,
    Json(request): Json<SubmissionRequest>,
) -> Result<StatusCode, StatusCode> {
    info!(
        "Submission attempt: participant_id='{}', exercise='{}'",
        request.ulid, request.exercise_name
    );

    // First, check if the participant exists
    let participant_exists = sqlx::query("SELECT 1 FROM participants WHERE id = ?")
        .bind(&request.ulid)
        .fetch_optional(&state.pool)
        .await;

    match participant_exists {
        Ok(Some(_)) => {
            // Participant exists, proceed with submission
            info!("Valid participant found for submission: {}", request.ulid);
        }
        Ok(None) => {
            warn!(
                "Submission attempt with invalid participant ID: {}",
                request.ulid
            );
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            error!("Database error while checking participant: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Calculate content hash for deduplication
    let content_hash =
        calculate_submission_hash(&request.ulid, &request.exercise_name, &request.source_code);

    // Check if identical submission already exists
    let existing_submission = sqlx::query(
        "SELECT id FROM submissions WHERE participant_id = ? AND exercise_name = ? AND content_hash = ?"
    )
    .bind(&request.ulid)
    .bind(&request.exercise_name)
    .bind(&content_hash)
    .fetch_optional(&state.pool)
    .await;

    match existing_submission {
        Ok(Some(_)) => {
            info!(
                "Duplicate submission detected for participant='{}', exercise='{}' - skipping",
                request.ulid, request.exercise_name
            );
            return Ok(StatusCode::OK); // Return success but don't store duplicate
        }
        Ok(None) => {
            // No duplicate found, proceed with insertion
        }
        Err(e) => {
            error!(
                "Database error while checking for duplicate submission: {}",
                e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Insert new submission with hash
    match sqlx::query(
        r#"
        INSERT INTO submissions (id, participant_id, exercise_name, source_code, tests_passed, clippy_passed, fmt_passed, content_hash)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(Ulid::new().to_string())
    .bind(&request.ulid)
    .bind(&request.exercise_name)
    .bind(&request.source_code)
    .bind(request.tests_passed)
    .bind(request.clippy_passed)
    .bind(request.fmt_passed)
    .bind(&content_hash)
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

    match get_exercise_progress(&state.pool, &ulid, &state.exercises).await {
        Ok(exercises) => {
            let completed_count = exercises.iter().filter(|e| e.completed).count();
            let perfected_count = exercises.iter().filter(|e| e.perfected).count();
            info!(
                "Status response: participant='{}', completed={}, perfected={}",
                ulid, completed_count, perfected_count
            );

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
        }
    }
}

/// Request body for `/api/run`. We accept any source code; the slug is
/// optional and only used for logging.
#[derive(Deserialize)]
struct RunRequest {
    code: String,
    #[serde(default)]
    slug: Option<String>,
}

/// Response shape mirroring the Playground `/execute` endpoint, plus a
/// parsed list of test results extracted from stdout.
#[derive(Serialize)]
struct RunResponse {
    success: bool,
    stdout: String,
    stderr: String,
    test_results: Vec<TestResult>,
}

#[derive(Serialize)]
struct TestResult {
    name: String,
    passed: bool,
}

/// Proxy handler: forwards the editor's source to play.rust-lang.org and
/// returns the JSON. We intentionally keep this thin: the upstream
/// already runs untrusted code in a sandbox and enforces its own rate
/// limits, so we just pass status codes back through.
async fn api_run(Json(req): Json<RunRequest>) -> Result<Json<RunResponse>, StatusCode> {
    let slug = req.slug.as_deref().unwrap_or("<unknown>");
    info!("/api/run: forwarding {} bytes for {slug}", req.code.len());

    let body = serde_json::json!({
        "channel": "stable",
        "mode": "debug",
        "edition": "2024",
        "crateType": "bin",
        "tests": true,
        "backtrace": false,
        "code": req.code,
    });

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|e| {
            error!("reqwest client build failed: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let resp = client
        .post("https://play.rust-lang.org/execute")
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            error!("Playground request failed: {e}");
            StatusCode::BAD_GATEWAY
        })?;

    let status = resp.status();
    if !status.is_success() {
        warn!("Playground returned non-2xx: {status}");
        // Forward 429 so the browser can show a 'rate limited' hint;
        // collapse anything else to 502.
        let mapped = if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            StatusCode::TOO_MANY_REQUESTS
        } else {
            StatusCode::BAD_GATEWAY
        };
        return Err(mapped);
    }

    #[derive(Deserialize)]
    struct PlaygroundResp {
        success: bool,
        stdout: String,
        stderr: String,
    }

    let parsed: PlaygroundResp = resp.json().await.map_err(|e| {
        error!("Failed to parse Playground response: {e}");
        StatusCode::BAD_GATEWAY
    })?;

    let test_results = parse_test_results(&parsed.stdout);
    info!(
        "/api/run: {} test result(s) parsed (success={})",
        test_results.len(),
        parsed.success
    );

    Ok(Json(RunResponse {
        success: parsed.success,
        stdout: parsed.stdout,
        stderr: parsed.stderr,
        test_results,
    }))
}

/// Request body for `/api/format`. Same shape as `/api/run` minus the
/// fields the formatter doesn't care about.
#[derive(Deserialize)]
struct FormatRequest {
    code: String,
    #[serde(default)]
    slug: Option<String>,
}

/// Response shape returned to the browser. `success = false` means the
/// formatter rejected the input (almost always a parse error); in that
/// case `stderr` carries rustfmt's complaint and `code` is the
/// (unchanged) original input.
#[derive(Serialize)]
struct FormatResponse {
    success: bool,
    code: String,
    stderr: String,
}

/// Proxy handler for play.rust-lang.org's `/format` endpoint. Takes the
/// editor's source, runs it through `rustfmt` upstream, and returns the
/// reformatted code so the client can replace its buffer.
async fn api_format(Json(req): Json<FormatRequest>) -> Result<Json<FormatResponse>, StatusCode> {
    let slug = req.slug.as_deref().unwrap_or("<unknown>");
    info!(
        "/api/format: forwarding {} bytes for {slug}",
        req.code.len()
    );

    let body = serde_json::json!({
        "channel": "stable",
        "edition": "2024",
        "code": req.code,
    });

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| {
            error!("reqwest client build failed: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let resp = client
        .post("https://play.rust-lang.org/format")
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            error!("Playground format request failed: {e}");
            StatusCode::BAD_GATEWAY
        })?;

    let status = resp.status();
    if !status.is_success() {
        warn!("Playground /format returned non-2xx: {status}");
        let mapped = if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            StatusCode::TOO_MANY_REQUESTS
        } else {
            StatusCode::BAD_GATEWAY
        };
        return Err(mapped);
    }

    #[derive(Deserialize)]
    struct PlaygroundFormatResp {
        success: bool,
        code: String,
        stderr: String,
    }

    let parsed: PlaygroundFormatResp = resp.json().await.map_err(|e| {
        error!("Failed to parse Playground /format response: {e}");
        StatusCode::BAD_GATEWAY
    })?;

    info!("/api/format: success={}", parsed.success);
    Ok(Json(FormatResponse {
        success: parsed.success,
        code: parsed.code,
        stderr: parsed.stderr,
    }))
}

/// Parse `test some::name ... ok` / `... FAILED` lines from cargo test
/// output. Anything we don't recognise is ignored, which is fine: the
/// raw stdout is forwarded too, so the UI can still show it.
fn parse_test_results(stdout: &str) -> Vec<TestResult> {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let line = line.trim_start();
        let Some(rest) = line.strip_prefix("test ") else {
            continue;
        };
        let Some((name, status)) = rest.rsplit_once(" ... ") else {
            continue;
        };
        // The harness also emits per-suite summaries like
        // "test result: ok. 4 passed; 0 failed". Skip those.
        if name.starts_with("result:") {
            continue;
        }
        let passed = match status.trim() {
            "ok" => true,
            "FAILED" => false,
            // "ignored", "bench", etc.: skip.
            _ => continue,
        };
        out.push(TestResult {
            name: name.trim().to_string(),
            passed,
        });
    }
    out
}

/// Get admin statistics
async fn get_admin_stats(pool: &SqlitePool) -> Result<AdminStats> {
    // Get total participants
    let total_participants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM participants")
        .fetch_one(pool)
        .await?;

    // Get total submissions across all participants
    let total_submissions: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM submissions WHERE tests_passed = 1")
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
async fn get_exercise_progress<'a>(
    pool: &'a SqlitePool,
    ulid: &'a str,
    catalog: &'a [Exercise],
) -> Result<Vec<ExerciseProgress>> {
    // Get all submissions (both successful and failed)
    let all_submissions: Vec<DbSubmission> = sqlx::query_as(
        "SELECT * FROM submissions WHERE participant_id = ? ORDER BY exercise_name, submitted_at DESC",
    )
    .bind(ulid)
    .fetch_all(pool)
    .await?;

    let mut exercises = Vec::with_capacity(catalog.len());
    for ex in catalog {
        // A chapter's submissions live under `<chapter_file_stem>` (legacy
        // single-step) or `<chapter_file_stem>/<step_key>` (multi-step).
        // Match both shapes so chapter-level progress aggregates correctly.
        let chapter_prefix = format!("{}/", ex.file_stem);
        let exercise_submissions: Vec<ExerciseSubmission> = all_submissions
            .iter()
            .filter(|s| {
                s.exercise_name == ex.file_stem || s.exercise_name.starts_with(&chapter_prefix)
            })
            .map(|s| ExerciseSubmission {
                id: s.id.clone(),
                source_code: s.source_code.clone(),
                tests_passed: s.tests_passed,
                clippy_passed: s.clippy_passed,
                fmt_passed: s.fmt_passed,
                submitted_at: s.submitted_at,
            })
            .collect();

        let is_quiz = ex.is_quiz();

        // Chapter is "completed" when every code step has at least one
        // passing submission. "Perfected" requires tests + fmt + clippy
        // green on the same submission for every step.
        let code_steps = ex.code_steps();
        let (completed, perfected) = if is_quiz || code_steps.is_empty() {
            (false, false)
        } else {
            let mut all_done = true;
            let mut all_perfect = true;
            for step in &code_steps {
                let step_key = step.key();
                let key = if step_key.is_empty() {
                    ex.file_stem.clone()
                } else {
                    format!("{}/{}", ex.file_stem, step_key)
                };
                let step_done = all_submissions
                    .iter()
                    .any(|s| s.exercise_name == key && s.tests_passed);
                let step_perfect = all_submissions.iter().any(|s| {
                    s.exercise_name == key && s.tests_passed && s.fmt_passed && s.clippy_passed
                });
                if !step_done {
                    all_done = false;
                }
                if !step_perfect {
                    all_perfect = false;
                }
            }
            (all_done, all_perfect)
        };

        exercises.push(ExerciseProgress {
            number: ex.number + 1,
            name: ex.file_stem.clone(),
            completed,
            perfected,
            title: ex.title.clone(),
            description: String::new(),
            submissions: exercise_submissions,
            is_quiz,
        });
    }

    Ok(exercises)
}

/// Calculate a hash for submission content to detect duplicates
fn calculate_submission_hash(
    participant_id: &str,
    exercise_name: &str,
    source_code: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(participant_id.as_bytes());
    hasher.update(b":");
    hasher.update(exercise_name.as_bytes());
    hasher.update(b":");
    hasher.update(source_code.as_bytes());
    format!("{:x}", hasher.finalize())
}
