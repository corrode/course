mod types;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use types::*;

const SERVER_URL: &str = "http://localhost:3000";
const TOKEN_FILE: &str = ".corrode/token";

#[derive(Parser)]
#[command(name = "cargo-course")]
#[command(about = "Submit Rust exercises to corrode course server")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the course repository and register participant
    Init,
    /// Submit an exercise solution
    Submit {
        /// Path to the exercise file (e.g., examples/01_strings.rs)
        file: String,
        /// Run fmt and clippy for a perfect submission
        #[arg(long)]
        perfect: bool,
    },
    /// Show progress and available exercises
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => handle_init().await,
        Commands::Submit { file, perfect } => handle_submit(&file, perfect).await,
        Commands::Status => handle_status().await,
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
    
    // Detect how the binary was called to give appropriate instructions
    let binary_name = get_binary_name();
    match binary_name.as_str() {
        "cargo-course" => {
            println!("💡 Tip: Install with `cargo install --path .` to use `cargo course submit <file>`");
            println!("For now, you can submit exercises with: {binary_name} submit <file>");
        }
        _ => {
            println!("You can now submit exercises with: {binary_name} submit <file>");
        }
    }

    Ok(())
}

/// Submit an exercise solution to the server.
async fn handle_submit(file: &str, perfect: bool) -> Result<()> {
    // 1. Read source code from file
    let source_code = fs::read_to_string(file)
        .map_err(|_| anyhow!("Failed to read file: {file}"))?;

    // 2. Extract exercise name from filename
    let exercise_name = extract_exercise_name(file)?;

    // 3. Run tests
    let (tests_passed, test_output) = run_cargo_test(&exercise_name).await?;

    // 4. If perfect flag, also run fmt + clippy
    let (fmt_passed, clippy_passed) = if perfect {
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
        if perfect && fmt_passed && clippy_passed {
            println!("⭐ Exercise {exercise_name} perfected!");
        } else {
            println!("✅ Exercise {exercise_name} completed!");
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

/// Get the name of the binary that was used to call this program.
fn get_binary_name() -> String {
    env::current_exe()
        .ok()
        .and_then(|path| path.file_name().map(|name| name.to_string_lossy().to_string()))
        .unwrap_or_else(|| "cargo-course".to_string())
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
        .await?;

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
        .await?;

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
        .await?;

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
