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
    /// Optional cohort label, populated when the participant signed up
    /// via `/signup/{group_slug}`. `None` for public signups. See
    /// migration `007_add_team_token.sql`.
    #[allow(dead_code)]
    team_token: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl DbParticipant {
    /// Returns the participant's parsed [`TeamToken`], if any. Rows
    /// that pre-date the type, or that somehow contain a value the
    /// validator rejects, surface as `None` rather than panicking;
    /// the worst case is the participant shows up in the
    /// "Unassigned" bucket and an admin can re-assign them.
    #[allow(dead_code)]
    fn parsed_team_token(&self) -> Option<TeamToken> {
        self.team_token
            .as_deref()
            .and_then(|s| TeamToken::try_from(s).ok())
    }
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

/// Template for the standalone playground (scratchpad) page.
#[derive(Template)]
#[template(path = "playground.html")]
struct PlaygroundTemplate {
    /// Initial source shown before the user has typed anything.
    starter: String,
}

/// Template for the cheatsheet page. Just renders pre-built HTML
/// produced from `static/cheatsheet.md` at startup.
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
    /// The chapter that comes after the current one in the catalog,
    /// or `None` when this is the last chapter. Used for the
    /// "Next chapter" call-to-action at the bottom of the page.
    next_dot: Option<ProgressDot>,
    /// Number of completable chapters the participant has finished.
    /// Quizzes and notes-only chapters don't count toward either total.
    progress_done: usize,
    /// Number of completable chapters in the course (denominator).
    progress_total: usize,
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
    /// `false` for notes-only chapters (the appendix). Used to skip
    /// them when building the "next chapter" CTA and progress totals.
    has_exercises: bool,
}

/// Per-exercise progress used by the chapter list and current-status badge.
#[derive(Clone, Default)]
struct UiExerciseStatus {
    attempted: bool,
    completed: bool,
    perfected: bool,
}

/// Template for participant dashboard.
///
/// Rendered in two modes:
/// - **Anonymous** (`/`): `participant_name` and `ulid` are `None`.
///   All chapters render with `completed = perfected = false`. Links
///   from the TOC point at `/exercise/{slug}` (no participant prefix).
/// - **Participant** (`/dashboard/{ulid}`): both fields are `Some`.
///   Completion marks reflect real submissions and links carry the ULID.
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    participant_name: Option<String>,
    ulid: Option<String>,
    /// One entry per chapter, in display order. Renders the bottom
    /// table of contents on the homepage via the shared
    /// `templates/partials/chapter_list.html` partial — the same one
    /// each exercise page uses for its "All exercises" nav, so the
    /// two stay visually identical. `current` is always `false`
    /// here because the homepage isn't any one chapter.
    dots: Vec<ProgressDot>,
    /// Slug of the first chapter the participant hasn't completed yet,
    /// or the first chapter overall if they're brand new / fully done.
    /// Used by the "Start" call-to-action.
    next_slug: String,
    /// Display label for the CTA ("Start with chapter 1" or
    /// "Resume chapter 5" depending on whether anything has been
    /// completed yet).
    next_label: String,
    /// Number / title of the chapter the participant should pick up at
    /// next. Used by the "where you left off" prose for authenticated
    /// participants. Empty / 0 when there is no next chapter (brand-new
    /// course with no exercises, or course fully completed).
    next_chapter_number: u8,
    next_chapter_title: String,
    /// Number of completable chapters the participant has finished.
    /// Always 0 in anonymous mode. Quizzes and notes-only chapters
    /// don't count.
    progress_done: usize,
    /// Number of completable chapters in the course (denominator).
    progress_total: usize,
    /// Optional one-shot reason code shown as a toast on first render
    /// of the anonymous dashboard. `Some("unknown-token")` after a
    /// missing-ULID redirect; `None` everywhere else.
    reason: Option<String>,
    /// Cohort label for the signed-in participant, if any. Drives the
    /// "View your team" link on the dashboard. `None` for anonymous
    /// viewers and for participants who signed up via the public form.
    team_token: Option<TeamToken>,
}

/// Template for the slim signup form.
///
/// `group_slug` is `Some` when reached via `/signup/{group_slug}`; it
/// renders an inline banner above the form and a hidden `team_token`
/// input so the cohort label is round-tripped back to `/register`.
#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate {
    group_slug: Option<String>,
}

/// Template for admin dashboard
#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate {
    participant_groups: Vec<ParticipantGroup>,
    recent_submissions: Vec<SubmissionSummary>,
    stats: AdminStats,
    exercises: Vec<String>,
    /// Echoed back into every form action / link so the admin token
    /// stays attached as the operator clicks around.
    admin_token: String,
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
    /// Notes-only chapters (appendix material) have no code steps and
    /// can't be "completed". The dashboard skips them when choosing
    /// the next chapter to resume.
    has_exercises: bool,
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
#[derive(Serialize, Clone)]
struct ParticipantSummary {
    id: String,
    name: String,
    completed_count: i64,
    total_exercises: i64,
    last_activity: Option<chrono::DateTime<chrono::Utc>>,
    /// Cohort label written by `/signup/{group_slug}`, or `None` for
    /// participants who came through the public `/signup` form. Used
    /// to bucket the admin participants table.
    team_token: Option<TeamToken>,
}

/// One row of the admin participants view: a team bucket and the
/// members in it. `team_token` is `None` for the synthetic
/// "Unassigned" bucket that gathers every participant without a
/// cohort label.
#[derive(Serialize, Clone)]
struct ParticipantGroup {
    team_token: Option<TeamToken>,
    members: Vec<ParticipantSummary>,
}

/// Submission summary for admin view
#[derive(Serialize, Clone)]
struct SubmissionSummary {
    participant_name: String,
    exercise_name: String,
    /// Human-readable version of `exercise_name`. The DB value is
    /// `<chapter_file_stem>` for legacy single-step chapters and
    /// `<chapter_file_stem>/<step_key>` for multi-step (e.g.
    /// `01_strings_and_chars/4_shout`). For display we strip the
    /// numeric ordering prefixes and turn underscores into spaces so
    /// readers see "Strings and chars · Shout" instead of the raw key.
    exercise_label: String,
    tests_passed: bool,
    perfected: bool,
    submitted_at: chrono::DateTime<chrono::Utc>,
    source_code: String,
}

/// Strip the `NN_` ordering prefix and turn underscores into spaces.
/// `01_strings_and_chars` -> `Strings and chars`,
/// `4_shout` -> `Shout`.
fn prettify_slug_segment(seg: &str) -> String {
    let trimmed = seg
        .trim_start_matches(|c: char| c.is_ascii_digit())
        .trim_start_matches('_');
    let body = if trimmed.is_empty() { seg } else { trimmed };
    let spaced = body.replace('_', " ");
    let mut chars = spaced.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Format `exercise_name` (DB key) into something a reader can scan.
/// Multi-step keys (`<chapter>/<step>`) become `Chapter · Step`.
fn prettify_exercise_name(name: &str) -> String {
    let parts: Vec<String> = name.split('/').map(prettify_slug_segment).collect();
    parts.join(" · ")
}

/// Per-team page (`/admin/team/{slug}`, `/admin/team-unassigned`,
/// `/dashboard/{ulid}/team`). Renders a single cohort's roster plus
/// a feed of every member's submissions, with the same chrome and
/// the same code-editor look as the rest of the course site.
///
/// Two viewer modes share this template:
///
/// - **Admin** (`is_admin = true`): `admin_token` is `Some` so links
///   back to `/admin` carry the token, member rows show ULIDs and a
///   "View dashboard" link, and the "Unassigned" bucket is reachable.
/// - **Participant** (`is_admin = false`): no admin chrome, the
///   member matching `viewer_ulid` is highlighted as `is_self`, and
///   `back_href` returns the user to their own dashboard.
#[derive(Template)]
#[template(path = "team.html")]
struct TeamPageTemplate {
    /// Display label shown in the page heading. The cohort slug for
    /// a real team, or the literal string "Unassigned" for the
    /// no-cohort bucket.
    team_label: String,
    /// `true` for the synthetic Unassigned bucket. The template uses
    /// this to swap the eyebrow text and skip a couple of admin
    /// affordances that only make sense for real cohorts.
    is_unassigned: bool,
    /// `true` when rendered for an admin operator. Drives whether
    /// admin-only chrome (ULIDs, dashboard links, the back-to-admin
    /// link) is rendered.
    is_admin: bool,
    /// Admin token to thread back into `/admin` and `/dashboard/{ulid}`
    /// links so the operator stays authenticated as they click
    /// around. `None` in participant mode. Threaded into the inline
    /// "move to team" form action so the form posts back with the
    /// admin token still attached.
    admin_token: Option<String>,
    /// Roster: one row per member of this team, ordered by most
    /// recent activity first.
    members: Vec<TeamMemberView>,
    /// Chronological feed of every team member's most recent
    /// submissions, capped to keep the page from blowing up for a
    /// busy cohort.
    submissions: Vec<SubmissionSummary>,
    /// `true` when the submissions list is the cap (see
    /// [`TEAM_SUBMISSIONS_LIMIT`]) rather than the full history.
    submissions_truncated: bool,
    /// Distinct exercise names that show up in this team's submissions
    /// feed. Used to populate the per-team "Filter" dropdown so the
    /// reader can narrow the list without leaving the page.
    exercises: Vec<String>,
    /// URL for the "back" link at the top of the page.
    back_href: String,
    /// Label for the "back" link.
    back_label: String,
}

/// One member row on the team page.
#[derive(Serialize, Clone)]
struct TeamMemberView {
    id: String,
    name: String,
    completed_count: i64,
    total_exercises: i64,
    last_activity: Option<chrono::DateTime<chrono::Utc>>,
    /// `true` when this row is the participant currently viewing the
    /// page. Only ever set in participant mode; the template adds a
    /// small "You" badge so the user can spot themselves quickly.
    is_self: bool,
}

/// Cap on the number of submissions rendered on a single team page.
/// Keeps the page snappy for very active cohorts; the admin can dig
/// into individual dashboards for the full history.
const TEAM_SUBMISSIONS_LIMIT: i64 = 60;

/// Query parameters for admin access
#[derive(Deserialize)]
struct AdminQuery {
    token: String,
}

/// Optional query parameters on the anonymous dashboard at `/`.
///
/// `reason` carries a short machine-readable code that the template
/// turns into a toast. Only `unknown-token` is recognized today (set
/// when `/dashboard/{ulid}` redirects here because the ULID has no
/// matching participant). Anything else renders as no toast.
#[derive(Deserialize, Default)]
struct DashboardQuery {
    #[serde(default)]
    reason: Option<String>,
}

/// Form data for web registration.
///
/// `team_token` is populated from the URL path on `/signup/{group_slug}`
/// via a hidden input; it's never typed by the user. Public signups at
/// `/signup` leave it empty / `None`.
///
/// `next` is set by the inline signup card embedded in `exercise.html`
/// (see `signup_on_pass` directive). It carries the URL the visitor
/// was about to visit so we can redirect there with their fresh ULID
/// spliced in, instead of bouncing through the dashboard.
#[derive(Deserialize)]
struct WebRegistrationForm {
    name: String,
    #[serde(default)]
    team_token: Option<String>,
    #[serde(default)]
    next: Option<String>,
}

/// Resolves the optional `next` form field from `web_register` to a
/// safe redirect target. Returns `None` if the value is missing,
/// malformed, or doesn't match the narrow structure we expect (an
/// anonymous exercise URL: `/exercise/<slug>`). The slug whitelist is
/// intentionally tight to keep this from becoming an open redirect.
fn resolve_register_next(next: Option<&str>, ulid: &str) -> Option<String> {
    let raw = next?.trim();
    let slug = raw.strip_prefix("/exercise/")?;
    if slug.is_empty()
        || !slug
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return None;
    }
    Some(format!("/exercise/{ulid}/{slug}"))
}

/// Buckets a flat list of participants into one `ParticipantGroup`
/// per distinct `team_token`. Participants whose `team_token` is
/// `None` are gathered into a final "Unassigned" bucket. Assumes the
/// input is already sorted by `team_token` so a single linear pass
/// produces stable output.
fn group_participants_by_team(participants: Vec<ParticipantSummary>) -> Vec<ParticipantGroup> {
    let mut groups: Vec<ParticipantGroup> = Vec::new();
    for p in participants {
        match groups.last_mut() {
            Some(g) if g.team_token == p.team_token => g.members.push(p),
            _ => groups.push(ParticipantGroup {
                team_token: p.team_token.clone(),
                members: vec![p],
            }),
        }
    }
    groups
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
        .route("/", get(anonymous_dashboard))
        .route("/signup", get(signup_page))
        .route("/signup/{group_slug}", get(signup_page_with_group))
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
        .route(
            "/admin/participants/{ulid}/team-token",
            post(admin_set_team_token),
        )
        .route("/admin/team/{slug}", get(admin_team_page))
        .route("/admin/team-unassigned", get(admin_team_unassigned_page))
        .route("/dashboard/{ulid}/team", get(participant_team_page))
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

/// Build chapter-list rows for the homepage from per-exercise
/// progress. Mirrors the dot construction in `render_exercise_page`
/// so both pages feed the same `partials/chapter_list.html` partial.
/// `current` is always `false` on the homepage.
fn dots_from_exercises(exercises: &[ExerciseProgress]) -> Vec<ProgressDot> {
    exercises
        .iter()
        .map(|e| ProgressDot {
            slug: e.name.clone(),
            number: e.number,
            title: e.title.clone(),
            attempted: !e.submissions.is_empty(),
            completed: e.completed,
            perfected: e.perfected,
            current: false,
            is_quiz: e.is_quiz,
            has_exercises: e.has_exercises,
        })
        .collect()
}

/// Anonymous dashboard at `/`.
///
/// Renders the same `dashboard.html` template the participant view
/// uses, but with no ULID and no name. All chapters render with
/// `completed = perfected = false`; the TOC links to `/exercise/{slug}`
/// (the public exercise route). The CTA invites the visitor to start
/// chapter 1 without registering.
///
/// Optionally accepts a `?reason=...` query param surfaced as a one-shot
/// toast (e.g. when a participant landed on a missing-ULID dashboard).
async fn anonymous_dashboard(
    State(state): State<AppState>,
    Query(query): Query<DashboardQuery>,
) -> impl IntoResponse {
    let exercises = match get_exercise_progress(&state.pool, None, &state.exercises).await {
        Ok(exercises) => exercises,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build catalog").into_response();
        }
    };

    let first = exercises
        .iter()
        .find(|e| !e.is_quiz && e.has_exercises)
        .or_else(|| exercises.first());
    let (next_slug, next_label, next_chapter_number, next_chapter_title) = match first {
        Some(e) => (
            e.name.clone(),
            format!("Start with chapter {}", e.number),
            e.number,
            e.title.clone(),
        ),
        None => (
            String::new(),
            "Start the course".to_string(),
            0,
            String::new(),
        ),
    };
    let progress_total = exercises
        .iter()
        .filter(|e| !e.is_quiz && e.has_exercises)
        .count();

    let template = DashboardTemplate {
        participant_name: None,
        ulid: None,
        dots: dots_from_exercises(&exercises),
        next_slug,
        next_label,
        next_chapter_number,
        next_chapter_title,
        progress_done: 0,
        progress_total,
        reason: query.reason,
        team_token: None,
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

/// Public signup form at `/signup` (no cohort).
async fn signup_page() -> impl IntoResponse {
    render_signup(None)
}

/// Workshop signup form at `/signup/{group_slug}`.
///
/// The slug is captured server-side and surfaced both as a banner and
/// as a hidden `team_token` input on the form so it round-trips back
/// to `/register` without the user typing anything.
async fn signup_page_with_group(AxumPath(group_slug): AxumPath<String>) -> impl IntoResponse {
    // Defensive trim. Empty slugs degrade to the public form rather
    // than rendering a banner that says "Signing up with **(blank)**".
    let trimmed = group_slug.trim();
    let group = if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    };
    render_signup(group)
}

fn render_signup(group_slug: Option<String>) -> axum::response::Response {
    let template = SignupTemplate { group_slug };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to render template",
        )
            .into_response(),
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

/// Renders `static/cheatsheet.md` as a standalone reference page.
async fn cheatsheet_page() -> impl IntoResponse {
    let md = match std::fs::read_to_string("static/cheatsheet.md") {
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
    let md = match std::fs::read_to_string("static/cheatsheet.md") {
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

    // Treat blank / whitespace-only team tokens as "no cohort" and
    // reject any other malformed value. The hidden input on the
    // workshop signup page only ever sends a known-good slug, so a
    // bad value here means a hand-crafted POST and a 400 is fine.
    let team_token = form
        .team_token
        .as_deref()
        .map_or(Ok(None), TeamToken::parse_form_input)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let ulid = Ulid::new().to_string();

    sqlx::query("INSERT INTO participants (id, name, team_token) VALUES (?, ?, ?)")
        .bind(&ulid)
        .bind(name.as_str())
        .bind(team_token.as_ref().map(TeamToken::as_str))
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If the signup came from the inline card on an exercise page,
    // hop straight back to the exercise (now with the new ULID in the
    // URL) so the user lands on the next chapter they were already
    // looking at, not the dashboard.
    let target = resolve_register_next(form.next.as_deref(), &ulid)
        .unwrap_or_else(|| format!("/dashboard/{ulid}"));
    Ok(axum::response::Redirect::to(&target))
}

/// Participant dashboard handler
async fn participant_dashboard(
    AxumPath(ulid): AxumPath<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Get participant info
    let participant_result =
        sqlx::query_as("SELECT id, name, team_token, created_at FROM participants WHERE id = ?")
            .bind(&ulid)
            .fetch_one(&state.pool)
            .await;

    let participant: DbParticipant = match participant_result {
        Ok(p) => p,
        Err(_) => {
            // Unknown ULID. Old behavior was a hard 404, which is
            // unfriendly because the dashboard URL is the bookmark and
            // people will mistype it (or follow a stale link from
            // before we migrated databases). Send them home with a
            // soft toast instead so they have somewhere to go.
            return axum::response::Redirect::to("/?reason=unknown-token").into_response();
        }
    };

    // Get exercise progress
    let exercises = match get_exercise_progress(&state.pool, Some(&ulid), &state.exercises).await {
        Ok(exercises) => exercises,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get exercise progress",
            )
                .into_response();
        }
    };

    // CTA: jump to the first unfinished, non-quiz chapter that actually
    // has exercises. If everything is done, point back to chapter 1 as
    // a graceful default.
    let first_unfinished = exercises
        .iter()
        .find(|e| !e.completed && !e.is_quiz && e.has_exercises)
        .or_else(|| exercises.first());
    let any_completed = exercises.iter().any(|e| e.completed);
    let (next_slug, next_label, next_chapter_number, next_chapter_title) = match first_unfinished {
        Some(e) if any_completed => (
            e.name.clone(),
            format!("Resume chapter {}", e.number),
            e.number,
            e.title.clone(),
        ),
        Some(e) => (
            e.name.clone(),
            format!("Start with chapter {}", e.number),
            e.number,
            e.title.clone(),
        ),
        None => (
            String::new(),
            "Start the course".to_string(),
            0,
            String::new(),
        ),
    };
    let progress_total = exercises
        .iter()
        .filter(|e| !e.is_quiz && e.has_exercises)
        .count();
    let progress_done = exercises
        .iter()
        .filter(|e| !e.is_quiz && e.has_exercises && e.completed)
        .count();

    let team_token = participant.parsed_team_token();
    let template = DashboardTemplate {
        participant_name: Some(participant.name),
        ulid: Some(ulid.clone()),
        dots: dots_from_exercises(&exercises),
        next_slug,
        next_label,
        next_chapter_number,
        next_chapter_title,
        progress_done,
        progress_total,
        reason: None,
        team_token,
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
        Some(u) => match get_exercise_progress(&state.pool, Some(u), &state.exercises).await {
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
                has_exercises: !e.code_steps().is_empty(),
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
            Step::Quiz(quiz) => {
                items.push(RenderItem {
                    kind: RenderKind::Quiz {
                        html: quiz.render_html(),
                    },
                });
                prev_was_note = false;
            }
        }
    }

    // Next chapter for the bottom CTA: the first dot that comes after
    // the current one. We don't skip quizzes or appendices here so the
    // CTA always points to whatever the picker would show next.
    let next_dot = dots.get(idx + 1).cloned();

    // Progress: how many completable chapters has the participant
    // finished? Quizzes and notes-only chapters never "complete", so
    // they're excluded from both numerator and denominator.
    let progress_total = dots
        .iter()
        .filter(|d| !d.is_quiz && d.has_exercises)
        .count();
    let progress_done = dots
        .iter()
        .filter(|d| !d.is_quiz && d.has_exercises && d.completed)
        .count();

    let template = ExerciseTemplate {
        exercise,
        ulid,
        current_status,
        dots,
        items,
        next_dot,
        progress_done,
        progress_total,
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
            p.team_token,
            COUNT(s.id) as completed_count,
            15 as total_exercises,
            MAX(s.submitted_at) as last_activity
        FROM participants p
        LEFT JOIN submissions s ON p.id = s.participant_id AND s.tests_passed = 1
        GROUP BY p.id, p.name, p.team_token
        ORDER BY
            CASE WHEN p.team_token IS NULL THEN 1 ELSE 0 END,
            p.team_token COLLATE NOCASE,
            last_activity DESC NULLS LAST
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
        let raw_token: Option<String> = row.get("team_token");
        // Rows with a column value the validator rejects degrade to
        // `None` (the participant lands in the "Unassigned" bucket).
        // That can't happen via our own write paths, but it keeps a
        // hand-edited DB row from taking the page down.
        let team_token = raw_token
            .as_deref()
            .and_then(|s| TeamToken::try_from(s).ok());
        participants.push(ParticipantSummary {
            id: row.get("id"),
            name: row.get("name"),
            completed_count: row.get("completed_count"),
            total_exercises: row.get("total_exercises"),
            last_activity: row.get("last_activity"),
            team_token,
        });
    }

    // Bucket the flat list into team groups. The query already sorted
    // by team_token (with NULLs last) so a single linear pass is
    // enough to produce one bucket per distinct value.
    let participant_groups = group_participants_by_team(participants);

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
        let exercise_name: String = row.get("exercise_name");
        let exercise_label = prettify_exercise_name(&exercise_name);
        recent_submissions.push(SubmissionSummary {
            participant_name: row.get("participant_name"),
            exercise_name,
            exercise_label,
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
        participant_groups,
        recent_submissions,
        stats,
        exercises,
        admin_token: state.admin_token.clone(),
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

/// Form data for `POST /admin/participants/{ulid}/team-token`.
#[derive(Deserialize)]
struct TeamTokenForm {
    team_token: String,
}

/// Move a participant into a different team bucket (or out of any).
///
/// Submitted by the inline form on the admin participants table. The
/// `team_token` field is normalised via `normalize_team_token`:
/// blank values clear the column (sending the participant to the
/// "Unassigned" bucket), anything else has to be a short slug.
///
/// Always redirects back to `/admin?token=...` on success so the
/// admin sees the regrouped table immediately.
async fn admin_set_team_token(
    AxumPath(participant_id): AxumPath<String>,
    Query(query): Query<AdminQuery>,
    State(state): State<AppState>,
    axum::Form(form): axum::Form<TeamTokenForm>,
) -> impl IntoResponse {
    if query.token != state.admin_token {
        return (StatusCode::FORBIDDEN, "Invalid admin token").into_response();
    }

    let new_token = match TeamToken::parse_form_input(&form.team_token) {
        Ok(value) => value,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid team_token: {err}"),
            )
                .into_response();
        }
    };

    let result = sqlx::query("UPDATE participants SET team_token = ? WHERE id = ?")
        .bind(new_token.as_ref().map(TeamToken::as_str))
        .bind(&participant_id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() == 0 => {
            (StatusCode::NOT_FOUND, "Participant not found").into_response()
        }
        Ok(_) => {
            info!(
                "Admin moved participant {} to team {:?}",
                participant_id, new_token
            );
            // The admin token is already taken from the request URL,
            // which means the operator's browser was OK with it as-is;
            // echo it straight back, the same way the rest of the
            // admin UI does (see /admin/remove-participant).
            axum::response::Redirect::to(&format!("/admin?token={}", state.admin_token))
                .into_response()
        }
        Err(err) => {
            error!(
                "Failed to update team_token for {}: {}",
                participant_id, err
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

/// Builds the per-team page (`team.html`) for a given cohort label.
///
/// `team_token = None` selects the synthetic Unassigned bucket
/// (every participant whose `participants.team_token` column is
/// NULL). `viewer_ulid` highlights the matching member row in
/// participant mode and is ignored in admin mode. Returns the
/// rendered HTML response, or a 500 if any of the supporting queries
/// fail.
async fn render_team_page(
    state: &AppState,
    team_token: Option<&TeamToken>,
    is_admin: bool,
    admin_token: Option<String>,
    viewer_ulid: Option<&str>,
    back_href: String,
    back_label: String,
) -> axum::response::Response {
    let total_exercises: i64 = state.exercises.len() as i64;

    // Roster: every participant in this team, with a count of their
    // passing submissions and the timestamp of their most recent one.
    // We branch on the SQL because SQLite has no portable way to bind
    // an Option<&str> against `IS NULL` in a single statement.
    let roster_query = match team_token {
        Some(_) => {
            r#"
            SELECT p.id, p.name,
                   COUNT(s.id) AS completed_count,
                   MAX(s.submitted_at) AS last_activity
            FROM participants p
            LEFT JOIN submissions s
                ON p.id = s.participant_id AND s.tests_passed = 1
            WHERE p.team_token = ?
            GROUP BY p.id, p.name
            ORDER BY last_activity DESC NULLS LAST, p.name COLLATE NOCASE
            "#
        }
        None => {
            r#"
            SELECT p.id, p.name,
                   COUNT(s.id) AS completed_count,
                   MAX(s.submitted_at) AS last_activity
            FROM participants p
            LEFT JOIN submissions s
                ON p.id = s.participant_id AND s.tests_passed = 1
            WHERE p.team_token IS NULL
            GROUP BY p.id, p.name
            ORDER BY last_activity DESC NULLS LAST, p.name COLLATE NOCASE
            "#
        }
    };
    let roster_q = sqlx::query(roster_query);
    let roster_q = match team_token {
        Some(t) => roster_q.bind(t.as_str()),
        None => roster_q,
    };
    let roster_rows = match roster_q.fetch_all(&state.pool).await {
        Ok(rows) => rows,
        Err(err) => {
            error!("team page roster query failed: {err}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let mut members = Vec::with_capacity(roster_rows.len());
    for row in &roster_rows {
        let id: String = row.get("id");
        let is_self = viewer_ulid.is_some_and(|u| u == id.as_str());
        members.push(TeamMemberView {
            id,
            name: row.get("name"),
            completed_count: row.get("completed_count"),
            total_exercises,
            last_activity: row.get("last_activity"),
            is_self,
        });
    }

    // Submissions feed: every member's latest activity, capped so a
    // very busy cohort doesn't render thousands of editor instances.
    let member_ids: Vec<String> = roster_rows
        .iter()
        .map(|r| r.get::<String, _>("id"))
        .collect();

    let (submissions, submissions_truncated) = if member_ids.is_empty() {
        (Vec::new(), false)
    } else {
        // Hand-rolled IN list. Member IDs are ULIDs we just read out
        // of the same DB so there's no untrusted input to escape;
        // sqlx's prepared-statement placeholders go in via `bind`.
        let placeholders = std::iter::repeat_n("?", member_ids.len())
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            r#"
            SELECT p.name AS participant_name,
                   s.exercise_name,
                   s.tests_passed,
                   s.fmt_passed,
                   s.clippy_passed,
                   s.submitted_at,
                   s.source_code
            FROM submissions s
            JOIN participants p ON s.participant_id = p.id
            WHERE s.participant_id IN ({placeholders})
            ORDER BY s.submitted_at DESC
            LIMIT ?
            "#
        );
        let mut q = sqlx::query(&sql);
        for id in &member_ids {
            q = q.bind(id);
        }
        // +1 so we can detect truncation without a second query.
        q = q.bind(TEAM_SUBMISSIONS_LIMIT + 1);
        let rows = match q.fetch_all(&state.pool).await {
            Ok(rows) => rows,
            Err(err) => {
                error!("team page submissions query failed: {err}");
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
            }
        };
        let truncated = rows.len() as i64 > TEAM_SUBMISSIONS_LIMIT;
        let mut subs = Vec::with_capacity(rows.len().min(TEAM_SUBMISSIONS_LIMIT as usize));
        for row in rows.into_iter().take(TEAM_SUBMISSIONS_LIMIT as usize) {
            let fmt_passed: bool = row.get("fmt_passed");
            let clippy_passed: bool = row.get("clippy_passed");
            let exercise_name: String = row.get("exercise_name");
            let exercise_label = prettify_exercise_name(&exercise_name);
            subs.push(SubmissionSummary {
                participant_name: row.get("participant_name"),
                exercise_name,
                exercise_label,
                tests_passed: row.get("tests_passed"),
                perfected: fmt_passed && clippy_passed,
                submitted_at: row.get("submitted_at"),
                source_code: row.get("source_code"),
            });
        }
        (subs, truncated)
    };

    let (team_label, is_unassigned) = match team_token {
        Some(t) => (t.as_str().to_string(), false),
        None => ("Unassigned".to_string(), true),
    };

    // Distinct exercise names from the submissions we're about to
    // render. Sorted for a stable, alphabetic dropdown.
    let mut exercises: Vec<String> = submissions
        .iter()
        .map(|s| s.exercise_name.clone())
        .collect();
    exercises.sort();
    exercises.dedup();

    let template = TeamPageTemplate {
        team_label,
        is_unassigned,
        is_admin,
        admin_token,
        members,
        submissions,
        submissions_truncated,
        exercises,
        back_href,
        back_label,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            error!("team page template render failed: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response()
        }
    }
}

/// Admin: per-team page for a real cohort slug.
async fn admin_team_page(
    AxumPath(slug): AxumPath<String>,
    Query(query): Query<AdminQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if query.token != state.admin_token {
        return (StatusCode::FORBIDDEN, "Invalid admin token").into_response();
    }
    let token = match TeamToken::try_from(slug.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid team slug").into_response();
        }
    };
    let admin_token = state.admin_token.clone();
    let back_href = format!("/admin?token={admin_token}");
    render_team_page(
        &state,
        Some(&token),
        true,
        Some(admin_token),
        None,
        back_href,
        "Back to admin".to_string(),
    )
    .await
}

/// Admin: per-team page for the synthetic Unassigned bucket.
async fn admin_team_unassigned_page(
    Query(query): Query<AdminQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if query.token != state.admin_token {
        return (StatusCode::FORBIDDEN, "Invalid admin token").into_response();
    }
    let admin_token = state.admin_token.clone();
    let back_href = format!("/admin?token={admin_token}");
    render_team_page(
        &state,
        None,
        true,
        Some(admin_token),
        None,
        back_href,
        "Back to admin".to_string(),
    )
    .await
}

/// Participant: read-only view of their own cohort's submissions.
///
/// Returns 404 if the ULID is unknown; redirects to the dashboard
/// with a one-shot toast if the participant has no team_token (no
/// cohort means there's nothing meaningful to show on this page).
async fn participant_team_page(
    AxumPath(ulid): AxumPath<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let participant: DbParticipant = match sqlx::query_as(
        "SELECT id, name, team_token, created_at FROM participants WHERE id = ?",
    )
    .bind(&ulid)
    .fetch_one(&state.pool)
    .await
    {
        Ok(p) => p,
        Err(_) => {
            return axum::response::Redirect::to("/?reason=unknown-token").into_response();
        }
    };

    let token = match participant.parsed_team_token() {
        Some(t) => t,
        None => {
            // No cohort to show; bounce back to the personal dashboard.
            return axum::response::Redirect::to(&format!("/dashboard/{ulid}")).into_response();
        }
    };

    let back_href = format!("/dashboard/{ulid}");
    render_team_page(
        &state,
        Some(&token),
        false,
        None,
        Some(&ulid),
        back_href,
        "Back to your dashboard".to_string(),
    )
    .await
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
                "Duplicate submission detected for participant='{}', exercise='{}'; skipping",
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

    match get_exercise_progress(&state.pool, Some(&ulid), &state.exercises).await {
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

/// Response type mirroring the Playground `/execute` endpoint, plus a
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

/// Request body for `/api/format`. Same request as `/api/run` minus the
/// fields the formatter doesn't care about.
#[derive(Deserialize)]
struct FormatRequest {
    code: String,
    #[serde(default)]
    slug: Option<String>,
}

/// Response returned to the browser. `success = false` means the
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

/// Build the per-chapter progress vector used by the dashboard.
///
/// Pass `Some(ulid)` for an authenticated participant view; submissions
/// belonging to that participant are aggregated into per-chapter
/// `completed` / `perfected` flags. Pass `None` for the anonymous
/// dashboard at `/`; the database is skipped entirely and every
/// chapter comes back with both flags `false`.
async fn get_exercise_progress<'a>(
    pool: &'a SqlitePool,
    ulid: Option<&'a str>,
    catalog: &'a [Exercise],
) -> Result<Vec<ExerciseProgress>> {
    // Anonymous mode: skip the SQL round-trip entirely. The loop below
    // still has to run to enumerate the catalog, but with no rows to
    // match against every chapter falls into the "not yet attempted"
    // branch.
    let all_submissions: Vec<DbSubmission> = match ulid {
        Some(ulid) => sqlx::query_as(
            "SELECT * FROM submissions WHERE participant_id = ? ORDER BY exercise_name, submitted_at DESC",
        )
        .bind(ulid)
        .fetch_all(pool)
        .await?,
        None => Vec::new(),
    };

    let mut exercises = Vec::with_capacity(catalog.len());
    for ex in catalog {
        // A chapter's submissions live under `<chapter_file_stem>` (legacy
        // single-step) or `<chapter_file_stem>/<step_key>` (multi-step).
        // Match both formats so chapter-level progress aggregates correctly.
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
            has_exercises: !code_steps.is_empty(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_register_next_accepts_simple_exercise_url() {
        assert_eq!(
            resolve_register_next(Some("/exercise/01_numbers"), "ULID"),
            Some("/exercise/ULID/01_numbers".to_string()),
        );
    }

    #[test]
    fn resolve_register_next_trims_whitespace() {
        assert_eq!(
            resolve_register_next(Some("  /exercise/foo-bar  "), "U"),
            Some("/exercise/U/foo-bar".to_string()),
        );
    }

    #[test]
    fn resolve_register_next_rejects_open_redirects() {
        // Anything that doesn't begin with the literal `/exercise/`
        // prefix, or contains characters outside the slug allow-list,
        // gets dropped so we can't be tricked into bouncing the user
        // off-site after registration.
        let bad = [
            None,
            Some(""),
            Some("/dashboard/U"),
            Some("https://evil.com/exercise/x"),
            Some("//evil.com/exercise/x"),
            Some("/exercise/"),
            Some("/exercise/foo/bar"),
            Some("/exercise/foo bar"),
            Some("/exercise/foo?next=evil"),
            Some("/exercise/foo#frag"),
        ];
        for input in bad {
            assert_eq!(
                resolve_register_next(input, "U"),
                None,
                "expected None for {input:?}"
            );
        }
    }

    #[test]
    fn team_token_form_input_treats_blank_as_none() {
        assert_eq!(TeamToken::parse_form_input(""), Ok(None));
        assert_eq!(TeamToken::parse_form_input("   "), Ok(None));
        assert_eq!(TeamToken::parse_form_input("\t\n "), Ok(None));
    }

    #[test]
    fn team_token_form_input_accepts_slugs() {
        let parsed = TeamToken::parse_form_input("veo").unwrap().unwrap();
        assert_eq!(parsed.as_str(), "veo");
        let parsed = TeamToken::parse_form_input("  veo-x9k2 ").unwrap().unwrap();
        assert_eq!(parsed.as_str(), "veo-x9k2");
        let parsed = TeamToken::parse_form_input("team_2025").unwrap().unwrap();
        assert_eq!(parsed.as_str(), "team_2025");
    }

    #[test]
    fn team_token_rejects_garbage() {
        // Anything outside [A-Za-z0-9_-], plus anything longer than
        // 64 chars, gets rejected so an admin can't smuggle HTML,
        // path separators, or a giant blob into the column.
        assert!(TeamToken::parse_form_input("team one").is_err());
        assert!(TeamToken::parse_form_input("team/one").is_err());
        assert!(TeamToken::parse_form_input("<script>").is_err());
        assert!(TeamToken::parse_form_input("équipe").is_err());
        assert!(TeamToken::parse_form_input(&"x".repeat(65)).is_err());
    }

    fn make_summary(name: &str, team: Option<&str>) -> ParticipantSummary {
        ParticipantSummary {
            id: format!("id-{name}"),
            name: name.to_string(),
            completed_count: 0,
            total_exercises: 15,
            last_activity: None,
            team_token: team.map(|s| TeamToken::try_from(s).expect("valid team token in test")),
        }
    }

    #[test]
    fn group_participants_buckets_by_team_token() {
        // Pre-sorted as the SQL query produces: alphabetical teams
        // first, NULLs last.
        let participants = vec![
            make_summary("alice", Some("alpha")),
            make_summary("bob", Some("alpha")),
            make_summary("carol", Some("beta")),
            make_summary("dave", None),
            make_summary("eve", None),
        ];
        let groups = group_participants_by_team(participants);
        assert_eq!(groups.len(), 3);
        assert_eq!(
            groups[0].team_token.as_ref().map(TeamToken::as_str),
            Some("alpha")
        );
        assert_eq!(groups[0].members.len(), 2);
        assert_eq!(
            groups[1].team_token.as_ref().map(TeamToken::as_str),
            Some("beta")
        );
        assert_eq!(groups[1].members.len(), 1);
        assert_eq!(groups[2].team_token, None);
        assert_eq!(groups[2].members.len(), 2);
    }

    #[test]
    fn group_participants_handles_empty_input() {
        assert!(group_participants_by_team(Vec::new()).is_empty());
    }
}
