use cargo_course::types::{
    Name, ProgressResponse, RegistrationRequest, RegistrationResponse, SubmissionRequest, Token,
};

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

const DEFAULT_SERVER_URL: &str = "https://course.corrode.dev";
const TOKEN_FILE: &str = ".corrode/token";

/// Get the server URL from environment variable or use default
fn get_server_url() -> String {
    env::var("CORRODE_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string())
}

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(about = "Submit Rust exercises to corrode course server")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Submit Rust exercises to corrode course server
    Course(CourseArgs),
}

#[derive(Parser)]
struct CourseArgs {
    #[command(subcommand)]
    command: CourseCommands,
}

#[derive(Subcommand)]
enum CourseCommands {
    /// Initialize the course repository and register participant
    Init {
        /// Use an existing token instead of registering a new participant
        #[arg(short, long)]
        token: Option<String>,
    },
    /// Submit an exercise solution
    Submit {
        /// Path to the exercise file (e.g., `examples/01_strings.rs`)
        file: Option<String>,
        /// Run fmt and clippy for a pedantic submission to earn a star
        #[arg(long)]
        pedantic: bool,
        /// Submit all exercises that pass tests
        #[arg(long)]
        all: bool,
    },
    /// Show progress and available exercises
    Status,
    /// Open the course dashboard in the browser
    Open,
    /// Print the current token to stdout
    Token,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Course(args) => match args.command {
            CourseCommands::Init { token } => handle_init(token).await,
            CourseCommands::Submit {
                file,
                pedantic,
                all,
            } => handle_submit(file.as_deref(), pedantic, all).await,
            CourseCommands::Status => handle_status().await,
            CourseCommands::Open => handle_open(),
            CourseCommands::Token => handle_token(),
        },
    }
}

fn handle_token() -> Result<()> {
    let Ok(token) = read_token() else {
        return Err(anyhow!(
            "No token found. Run 'cargo course init' to register."
        ));
    };
    println!("{}", token.as_str());
    Ok(())
}

fn handle_open() -> Result<()> {
    // Read token from file
    let token = read_token()?;

    // Construct the dashboard URL
    let url = format!("{}/dashboard/{}", get_server_url(), token.as_str());

    // Open the URL in the default browser
    if open::that(&url).is_err() {
        return Err(anyhow!("Failed to open browser. Please visit: {url}"));
    }

    println!("🌐 Opening course dashboard: {url}");
    Ok(())
}

/// Initialize the course repository and register participant if needed.
async fn handle_init(token_arg: Option<String>) -> Result<()> {
    // If a token was provided as argument, use it
    if let Some(token_str) = token_arg {
        let token = Token::from_str(&token_str)?;
        save_token(&token)?;
        println!("✅ Token saved successfully: {token}");
        println!("💡 Submit exercises with: cargo course submit <file>");
        println!("💡 For pedantic submissions (earn stars): cargo course submit <file> --pedantic");
        println!("💡 Open dashboard with: cargo course open");
        return Ok(());
    }

    // Check for existing token
    if let Ok(existing_token) = read_token() {
        println!("✅ You're already registered with token: {existing_token}");
        println!("💡 Use --token <TOKEN> to replace with a different token");
        return Ok(());
    }

    // Start registration flow
    println!("🚀 Welcome to the corrode Rust Course!");
    print!("How should I call you? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let name = Name::try_from(input)?;
    let token_str = register_with_server(&name).await?;
    let token = Token::new(token_str);
    save_token(&token)?;

    println!("✅ Registered successfully! Token: {token}");

    // Give instructions on how to use the CLI
    println!("💡 Submit exercises with: cargo course submit <file>");
    println!("💡 For pedantic submissions (earn stars): cargo course submit <file> --pedantic");

    Ok(())
}

/// Submit an exercise solution to the server.
async fn handle_submit(file: Option<&str>, pedantic: bool, all: bool) -> Result<()> {
    if all {
        return handle_submit_all(pedantic).await;
    }

    let file = file.ok_or_else(|| anyhow!("File path is required when not using --all"))?;
    // 1. Read source code from file
    let source_code =
        fs::read_to_string(file).map_err(|_| anyhow!("Failed to read file: {file}"))?;

    // 2. Extract chapter + optional step from the path. For legacy
    //    single-step chapters the step is `None`, and `exercise_name`
    //    is just the chapter slug. For multi-step (`<chapter>/<n>_<slug>.rs`)
    //    the exercise key is `<chapter>/<n>_<slug>` and the test filter
    //    is `_<n>_<slug>::`.
    let target = extract_submission_target(file)?;
    let exercise_name = target.exercise_key();

    // 3. Run tests scoped to the right chapter (and step, if any).
    let (tests_passed, test_output) =
        run_cargo_test(&target.chapter, target.test_filter.as_deref())?;

    // 4. If pedantic flag, also run fmt + clippy
    let (fmt_passed, clippy_passed) = if pedantic {
        (run_cargo_fmt()?, run_cargo_clippy()?)
    } else {
        (false, false)
    };

    // 5. Read token from file
    let token = read_token()?;

    // 6. Submit to server
    submit_to_server(SubmissionRequest {
        ulid: token.as_str().to_string(),
        exercise_name: exercise_name.clone(),
        source_code,
        tests_passed,
        clippy_passed,
        fmt_passed,
    })
    .await?;

    // 7. Show result
    if tests_passed {
        if pedantic && fmt_passed && clippy_passed {
            println!("⭐ Exercise {exercise_name} perfected! You earned a star!");
        } else {
            println!("✅ Exercise {exercise_name} completed!");
            if !pedantic {
                println!("💡 Try submitting with --pedantic to earn a star and perfect your code!");
            }
        }
    } else {
        println!("❌ Tests failed for {exercise_name}");
        if !test_output.is_empty() {
            println!("\n🔍 Test output for troubleshooting:");
            println!("{test_output}");
        }
    }

    Ok(())
}

/// Submit all exercises that pass tests.
async fn handle_submit_all(pedantic: bool) -> Result<()> {
    println!("🔍 Scanning for exercises...");

    // Read token from file
    let token = read_token()?;

    // Get list of all exercise files
    let exercise_files = find_exercise_files()?;
    if exercise_files.is_empty() {
        println!("❌ No exercise files found in examples/ directory");
        return Ok(());
    }

    println!("📋 Found {} exercise files", exercise_files.len());
    println!("🚀 Testing exercises in parallel...");

    // Process exercises in parallel using bounded concurrency
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(4)); // Limit to 4 concurrent operations
    let token = std::sync::Arc::new(token);

    let tasks: Vec<_> = exercise_files
        .into_iter()
        .map(|file_path| {
            let semaphore = semaphore.clone();
            let token = token.clone();
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                process_single_exercise(file_path, pedantic, &token).await
            })
        })
        .collect();

    // Wait for all tasks to complete and collect results
    let mut successful_submissions = 0;
    let mut failed_exercises = Vec::new();

    for task in tasks {
        match task.await {
            Ok(result) => match result {
                Ok(exercise_name) => {
                    successful_submissions += 1;
                    println!("✅ {exercise_name} submitted successfully");
                }
                Err(exercise_name) => {
                    failed_exercises.push(exercise_name);
                }
            },
            Err(_) => {
                // Task panicked; this shouldn't happen in normal operation
                failed_exercises.push("unknown".to_string());
            }
        }
    }

    // Summary
    println!("\n📊 Submission Summary:");
    println!("✅ Successfully submitted: {successful_submissions}");
    if !failed_exercises.is_empty() {
        println!("❌ Failed exercises: {}", failed_exercises.len());
        println!("   {}", failed_exercises.join(", "));
    }

    if successful_submissions > 0 {
        println!("\n🎉 Use 'cargo course status' to see your updated progress!");
    }

    Ok(())
}

/// Process a single exercise: test, validate, and submit if successful.
/// Returns `Ok(exercise_name)` on success, `Err(exercise_name)` on failure.
async fn process_single_exercise(
    file_path: String,
    pedantic: bool,
    token: &Token,
) -> Result<String, String> {
    let Ok(target) = extract_submission_target(&file_path) else {
        println!("⚠️  Skipping {file_path}: invalid filename format");
        return Err(file_path);
    };
    let exercise_name = target.exercise_key();

    print!("🧪 Testing {exercise_name}... ");
    std::io::stdout().flush().unwrap();

    // Run tests for this exercise
    let Ok((tests_passed, _test_output)) =
        run_cargo_test(&target.chapter, target.test_filter.as_deref())
    else {
        println!("❌ Error running tests");
        return Err(exercise_name);
    };

    if !tests_passed {
        println!("❌ Tests failed");
        return Err(exercise_name);
    }

    // Tests passed, now read source code
    let Ok(source_code) = fs::read_to_string(&file_path) else {
        println!("❌ Failed to read file");
        return Err(exercise_name);
    };

    // If pedantic flag, also run fmt + clippy (these are global checks)
    let (fmt_passed, clippy_passed) = if pedantic {
        if let (Ok(fmt), Ok(clippy)) = (run_cargo_fmt(), run_cargo_clippy()) {
            (fmt, clippy)
        } else {
            println!("❌ Error running pedantic checks");
            return Err(exercise_name);
        }
    } else {
        (false, false)
    };

    // Submit to server
    let submission_result = submit_to_server(SubmissionRequest {
        ulid: token.as_str().to_string(),
        exercise_name: exercise_name.clone(),
        source_code,
        tests_passed,
        clippy_passed,
        fmt_passed,
    })
    .await;

    if submission_result.is_ok() {
        if pedantic && fmt_passed && clippy_passed {
            print!("⭐ Perfected! ");
        } else {
            print!("✅ Submitted! ");
        }
        Ok(exercise_name)
    } else {
        println!("❌ Upload failed");
        Err(exercise_name)
    }
}

/// Find all exercise files in the examples directory.
///
/// For legacy single-step chapters, returns the chapter's `main.rs`.
/// For multi-step chapters (those with `<n>_<slug>.rs` step files
/// alongside the generated `main.rs`), returns one entry per step file
/// instead so `--all` submits each step individually.
fn find_exercise_files() -> Result<Vec<String>> {
    let examples_dir = Path::new("examples");
    if !examples_dir.exists() {
        return Err(anyhow!("examples/ directory not found"));
    }

    let mut exercise_files = Vec::new();

    for entry in fs::read_dir(examples_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if dir_name.starts_with('_') || dir_name.starts_with('.') {
            continue;
        }

        // Collect sibling `<n>_<slug>.rs` step files first.
        let mut step_files: Vec<String> = Vec::new();
        if let Ok(rd) = fs::read_dir(&path) {
            for sub in rd.flatten() {
                let p = sub.path();
                if p.extension().and_then(|s| s.to_str()) != Some("rs") {
                    continue;
                }
                let Some(name) = p.file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                if name == "main.rs" {
                    continue;
                }
                // Only count files starting with `<n>_`.
                let Some(stem) = p.file_stem().and_then(|s| s.to_str()) else {
                    continue;
                };
                let Some((num, _)) = stem.split_once('_') else {
                    continue;
                };
                if num.parse::<u32>().is_err() {
                    continue;
                }
                if let Some(s) = p.to_str() {
                    step_files.push(s.to_string());
                }
            }
        }

        if step_files.is_empty() {
            let main_rs = path.join("main.rs");
            if main_rs.exists()
                && let Some(p) = main_rs.to_str()
            {
                exercise_files.push(p.to_string());
            }
        } else {
            step_files.sort();
            exercise_files.extend(step_files);
        }
    }

    // Sort files for consistent ordering
    exercise_files.sort();
    Ok(exercise_files)
}

/// Show participant progress and available exercises.
async fn handle_status() -> Result<()> {
    let token = read_token()?;
    let progress = fetch_progress(&token).await?;

    println!("📚 corrode Rust Course Progress");
    for exercise in &progress.exercises {
        let status_icon = if exercise.perfected {
            "⭐"
        } else if exercise.completed {
            "✅"
        } else {
            "⏳"
        };
        println!("{status_icon} {}", exercise.name);
    }

    let completed = progress.exercises.iter().filter(|e| e.completed).count();
    let total = progress.exercises.len();
    println!("\nProgress: {completed}/{total} exercises");

    Ok(())
}

/// A submission target derived from a file or chapter path.
///
/// For legacy single-step chapters this is just a chapter name; for
/// multi-step chapters it also carries a `cargo test` filter so we
/// only run the relevant step's tests.
struct SubmissionTarget {
    /// Chapter directory name (the `--example <chapter>` argument).
    chapter: String,
    /// `Some("<n>_<slug>")` for a step file; `None` for legacy chapters.
    step_key: Option<String>,
    /// `Some("_<n>_<slug>::")` filter passed to `cargo test`; `None` to
    /// run every test in the chapter.
    test_filter: Option<String>,
}

impl SubmissionTarget {
    /// Full `submissions.exercise_name` value: `<chapter>` for legacy,
    /// `<chapter>/<step_key>` for multi-step.
    fn exercise_key(&self) -> String {
        self.step_key.as_ref().map_or_else(
            || self.chapter.clone(),
            |k| format!("{}/{}", self.chapter, k),
        )
    }
}

/// Derive the submission target from a path the user passed to
/// `cargo course submit`.
///
/// Recognised inputs:
///
/// * `examples/<chapter>/main.rs`       (legacy single-step chapter)
/// * `examples/<chapter>/<n>_<slug>.rs` (multi-step file: step `n_slug`)
/// * `examples/<chapter>/` (or bare)    (legacy single-step chapter)
/// * `<chapter>` (bare slug)            (legacy single-step chapter)
/// * `examples/<chapter>.rs`            (legacy flat layout)
fn extract_submission_target(file_path: &str) -> Result<SubmissionTarget> {
    let path = Path::new(file_path);
    let filename = path.file_name().and_then(|n| n.to_str());

    // `<chapter>/main.rs`: legacy aggregator file.
    if filename == Some("main.rs") {
        let chapter = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("main.rs has no parent chapter directory"))?
            .to_string();
        return Ok(SubmissionTarget {
            chapter,
            step_key: None,
            test_filter: None,
        });
    }

    // `<chapter>/<n>_<slug>.rs`: multi-step file.
    if let Some(name) = filename
        && let Some(stem) = name.strip_suffix(".rs")
        && stem != "main"
    {
        if let Some(parent_name) = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
        {
            // Verify the stem starts with `<n>_` so we don't
            // mis-classify a bare `examples/foo.rs`.
            if let Some((num, _rest)) = stem.split_once('_')
                && num.parse::<u32>().is_ok()
            {
                return Ok(SubmissionTarget {
                    chapter: parent_name.to_string(),
                    step_key: Some(stem.to_string()),
                    test_filter: Some(format!("_{stem}::")),
                });
            }
        }
        // Bare `examples/<name>.rs`: legacy flat layout.
        return Ok(SubmissionTarget {
            chapter: stem.to_string(),
            step_key: None,
            test_filter: None,
        });
    }

    // Bare directory or chapter slug.
    let last = path
        .file_name()
        .ok_or_else(|| anyhow!("Invalid file path"))?
        .to_str()
        .ok_or_else(|| anyhow!("Invalid filename"))?;
    Ok(SubmissionTarget {
        chapter: last.to_string(),
        step_key: None,
        test_filter: None,
    })
}

/// Run cargo test for an exercise and return success status and output.
///
/// When `filter` is `Some`, it's passed as a `cargo test` test-name filter
/// (e.g. `_2_fallback::`) so only one step's tests run.
fn run_cargo_test(chapter: &str, filter: Option<&str>) -> Result<(bool, String)> {
    let mut cmd = Command::new("cargo");
    cmd.arg("test").arg("--example").arg(chapter);
    if let Some(f) = filter {
        cmd.arg("--").arg(f);
    }
    let output = cmd.output()?;

    let success = output.status.success();
    let stderr = String::from_utf8_lossy(&output.stderr);

    Ok((success, stderr.to_string()))
}

/// Run cargo fmt --check and return success status.
fn run_cargo_fmt() -> Result<bool> {
    let output = Command::new("cargo").args(["fmt", "--check"]).output()?;

    Ok(output.status.success())
}

/// Run cargo clippy with warnings as errors and return success status.
fn run_cargo_clippy() -> Result<bool> {
    let output = Command::new("cargo")
        .args(["clippy", "--", "-D", "warnings"])
        .output()?;

    Ok(output.status.success())
}

/// Register a new participant with the server.
async fn register_with_server(name: &Name) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/register", get_server_url()))
        .json(&RegistrationRequest { name: name.clone() })
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow!(
                    "❌ Cannot connect to the corrode course server at {}\n\n\
                     This usually means:\n\
                     • The course server is not running\n\
                     • You're working offline\n\n\
                     💡 For offline practice, use manual testing instead:\n\
                     cargo test --example 00_integers",
                    get_server_url()
                )
            } else {
                anyhow!("Network error: {e}")
            }
        })?;

    if !response.status().is_success() {
        return Err(anyhow!("Registration failed: {}", response.status()));
    }

    let reg_response: RegistrationResponse = response.json().await?;
    Ok(reg_response.ulid)
}

/// Submit an exercise solution to the server.
async fn submit_to_server(submission: SubmissionRequest) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/submit", get_server_url()))
        .json(&submission)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow!(
                    "❌ Cannot connect to the corrode course server at {}\n\n\
                     💡 Your solution was tested locally but couldn't be submitted.\n\
                     For offline practice, continue using: cargo test --example <exercise_name>",
                    get_server_url()
                )
            } else {
                anyhow!("Network error: {e}")
            }
        })?;

    if !response.status().is_success() {
        let error_msg = match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                "Invalid token. Please run 'cargo course init' to register or check your token."
                    .to_string()
            }
            reqwest::StatusCode::BAD_REQUEST => {
                "Invalid submission data. Please check your exercise file and try again."
                    .to_string()
            }
            reqwest::StatusCode::SERVICE_UNAVAILABLE => {
                format!(
                    "Server at {} is up but reported 503 Service Unavailable. \
                     The course server might be restarting or out of capacity. \
                     Try again in a moment.",
                    get_server_url()
                )
            }
            status => {
                format!("Submission failed: {status}")
            }
        };
        return Err(anyhow!(error_msg));
    }

    Ok(())
}

/// Fetch participant progress from the server.
async fn fetch_progress(token: &Token) -> Result<ProgressResponse> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "{}/api/status/{}",
            get_server_url(),
            token.as_str()
        ))
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow!(
                    "❌ Cannot connect to the corrode course server at {}\n\n\
                     💡 Server is not available to show your progress.\n\
                     Continue practicing with: cargo test --example <exercise_name>",
                    get_server_url()
                )
            } else {
                anyhow!("Network error: {e}")
            }
        })?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch progress: {}", response.status()));
    }

    let progress: ProgressResponse = response.json().await?;
    Ok(progress)
}

/// Read the participant token from the local file.
fn read_token() -> Result<Token> {
    let token_str = fs::read_to_string(TOKEN_FILE)
        .map_err(|_| anyhow!("No token found. Run 'cargo course init' to register."))?;
    Token::from_str(&token_str)
}

/// Save the participant token to the local file.
fn save_token(token: &Token) -> Result<()> {
    // Create .corrode directory if it doesn't exist
    if let Some(parent) = Path::new(TOKEN_FILE).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(TOKEN_FILE, token.as_str())?;
    Ok(())
}
