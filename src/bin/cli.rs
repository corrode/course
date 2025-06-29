use cargo_course::types::*;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

const SERVER_URL: &str = "http://localhost:3000";
const TOKEN_FILE: &str = ".corrode/token";

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
    Init,
    /// Submit an exercise solution
    Submit {
        /// Path to the exercise file (e.g., examples/01_strings.rs)
        file: String,
        /// Run fmt and clippy for a pedantic submission to earn a star
        #[arg(long)]
        pedantic: bool,
    },
    /// Show progress and available exercises
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Course(args) => match args.command {
            CourseCommands::Init => handle_init().await,
            CourseCommands::Submit { file, pedantic } => handle_submit(&file, pedantic).await,
            CourseCommands::Status => handle_status().await,
        },
    }
}

/// Initialize the course repository and register participant if needed.
async fn handle_init() -> Result<()> {
    // Check for existing token
    if let Ok(existing_token) = read_token() {
        println!("✅ You're already registered with token: {existing_token}");
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
async fn handle_submit(file: &str, pedantic: bool) -> Result<()> {
    // 1. Read source code from file
    let source_code = fs::read_to_string(file)
        .map_err(|_| anyhow!("Failed to read file: {file}"))?;

    // 2. Extract exercise name from filename
    let exercise_name = extract_exercise_name(file)?;

    // 3. Run tests
    let (tests_passed, test_output) = run_cargo_test(&exercise_name).await?;

    // 4. If pedantic flag, also run fmt + clippy
    let (fmt_passed, clippy_passed) = if pedantic {
        (run_cargo_fmt().await?, run_cargo_clippy().await?)
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

/// Extract exercise name from file path.
/// 
/// Examples:
/// - `examples/01_strings.rs` → `01_strings`
/// - `01_strings.rs` → `01_strings`
fn extract_exercise_name(file_path: &str) -> Result<String> {
    let path = Path::new(file_path);
    let filename = path
        .file_name()
        .ok_or_else(|| anyhow!("Invalid file path"))?
        .to_str()
        .ok_or_else(|| anyhow!("Invalid filename"))?;
    
    // Remove .rs extension
    if let Some(name) = filename.strip_suffix(".rs") {
        Ok(name.to_string())
    } else {
        Err(anyhow!("File must have .rs extension"))
    }
}

/// Run cargo test for an exercise and return success status and output.
async fn run_cargo_test(exercise_name: &str) -> Result<(bool, String)> {
    let output = Command::new("cargo")
        .args(["test", "--example", exercise_name])
        .output()?;
    
    let success = output.status.success();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    Ok((success, stderr.to_string()))
}

/// Run cargo fmt --check and return success status.
async fn run_cargo_fmt() -> Result<bool> {
    let output = Command::new("cargo")
        .args(["fmt", "--check"])
        .output()?;
    
    Ok(output.status.success())
}

/// Run cargo clippy with warnings as errors and return success status.
async fn run_cargo_clippy() -> Result<bool> {
    let output = Command::new("cargo")
        .args(["clippy", "--", "-D", "warnings"])
        .output()?;
    
    Ok(output.status.success())
}


/// Register a new participant with the server.
async fn register_with_server(name: &Name) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{SERVER_URL}/api/register"))
        .json(&RegistrationRequest {
            name: name.clone(),
        })
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow!(
                    "❌ Cannot connect to the corrode course server at {SERVER_URL}\n\n\
                     This usually means:\n\
                     • The course server is not running\n\
                     • You're working offline\n\n\
                     💡 For offline practice, use manual testing instead:\n\
                     cargo test --example 00_hello_rust"
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
        .post(&format!("{SERVER_URL}/api/submit"))
        .json(&submission)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow!(
                    "❌ Cannot connect to the corrode course server at {SERVER_URL}\n\n\
                     💡 Your solution was tested locally but couldn't be submitted.\n\
                     For offline practice, continue using: cargo test --example <exercise_name>"
                )
            } else {
                anyhow!("Network error: {e}")
            }
        })?;

    if !response.status().is_success() {
        return Err(anyhow!("Submission failed: {}", response.status()));
    }

    Ok(())
}

/// Fetch participant progress from the server.
async fn fetch_progress(token: &Token) -> Result<ProgressResponse> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{SERVER_URL}/api/status/{}", token.as_str()))
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow!(
                    "❌ Cannot connect to the corrode course server at {SERVER_URL}\n\n\
                     💡 Server is not available to show your progress.\n\
                     Continue practicing with: cargo test --example <exercise_name>"
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