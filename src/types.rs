//! Shared types for the corrode course system.
//!
//! This module provides the core data types used for communication between
//! the corrode course CLI tool and server. These types handle participant
//! registration, exercise submission, and progress tracking.
//!
//! The types are designed to be:
//! - Validated at construction time to prevent invalid states
//! - Serializable for JSON API communication
//! - Ergonomic to use with appropriate conversion traits

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A validated participant name.
///
/// Participant names must be non-empty and no longer than 100 characters.
/// Whitespace is automatically trimmed during construction.
///
/// # Examples
///
/// ```
/// use cargo_course::types::Name;
/// 
/// let name = Name::try_from("Alice".to_string()).unwrap();
/// assert_eq!(name.as_str(), "Alice");
/// 
/// // Whitespace is trimmed
/// let name = Name::try_from("  Bob  ".to_string()).unwrap();
/// assert_eq!(name.as_str(), "Bob");
/// 
/// // Empty names are rejected
/// assert!(Name::try_from("".to_string()).is_err());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(String);

impl Name {
    /// Maximum allowed length for a participant name.
    ///
    /// This limit helps prevent abuse and ensures names fit comfortably
    /// in database columns and UI displays.
    pub const MAX_LENGTH: usize = 100;
    
    /// Returns the validated name as a string slice.
    ///
    /// The returned string is guaranteed to be non-empty and no longer
    /// than [`Self::MAX_LENGTH`] characters.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Name {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        let trimmed = value.trim();
        
        if trimmed.is_empty() {
            return Err(anyhow!("Name cannot be empty"));
        }
        
        if trimmed.len() > Self::MAX_LENGTH {
            return Err(anyhow!("Name too long (max {} characters)", Self::MAX_LENGTH));
        }
        
        Ok(Self(trimmed.to_string()))
    }
}

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(s.to_string())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A ULID token for participant identification.
///
/// Tokens are used to uniquely identify participants in the course system.
/// They are generated server-side as ULIDs and stored locally by the CLI.
///
/// # Examples
///
/// ```
/// use cargo_course::types::Token;
/// use std::str::FromStr;
///
/// // Tokens are typically created from server responses
/// let token = Token::new("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string());
/// assert_eq!(token.as_str(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
///
/// // Can be parsed from strings (e.g., reading from file)
/// let token = Token::from_str("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    /// Creates a new token from a ULID string.
    ///
    /// This constructor does not validate the ULID format, as tokens
    /// are typically received from trusted sources (the server).
    #[must_use]
    pub fn new(ulid: String) -> Self {
        Self(ulid)
    }
    
    /// Returns the token as a string slice.
    ///
    /// This is primarily used for serialization and API requests.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Token cannot be empty"));
        }
        Ok(Self(trimmed.to_string()))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Request payload for registering a new participant.
///
/// This is sent by the CLI during the `cargo course init` flow
/// when a participant enters their name.
#[derive(Serialize)]
pub struct RegistrationRequest {
    /// The participant's chosen name (already validated)
    pub name: Name,
}

/// Response payload from successful participant registration.
///
/// The server generates a unique ULID token that the CLI
/// stores locally for subsequent requests.
#[derive(Deserialize)]
pub struct RegistrationResponse {
    /// The generated ULID token for this participant
    pub ulid: String,
}

/// Request payload for submitting an exercise solution.
///
/// This contains the participant's code and the results of local
/// testing and linting checks performed by the CLI.
#[derive(Serialize)]
pub struct SubmissionRequest {
    /// The participant's ULID token for identification
    pub ulid: String,
    /// Name of the exercise (e.g., "01_strings")
    pub exercise_name: String,
    /// The complete source code submitted by the participant
    pub source_code: String,
    /// Whether the exercise tests passed locally
    pub tests_passed: bool,
    /// Whether `cargo clippy` passed (for pedantic submissions)
    pub clippy_passed: bool,
    /// Whether `cargo fmt --check` passed (for pedantic submissions)
    pub fmt_passed: bool,
}

/// Status information for a single exercise.
///
/// This represents a participant's progress on one exercise,
/// including whether they've completed it and achieved perfection.
#[derive(Deserialize)]
pub struct ExerciseStatus {
    /// The exercise name (e.g., "01_strings")
    pub name: String,
    /// Whether the participant has submitted a passing solution
    pub completed: bool,
    /// Whether the solution passed all pedantic checks (fmt + clippy)
    pub perfected: bool,
}

/// Response payload containing a participant's overall progress.
///
/// This is returned by the status API endpoint and contains
/// information about all exercises in the course.
#[derive(Deserialize)]
pub struct ProgressResponse {
    /// Status for each exercise in the course
    pub exercises: Vec<ExerciseStatus>,
}