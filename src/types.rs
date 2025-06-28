//! Shared types for the corrode course system.
//!
//! These types are used by both the CLI and server for consistent data exchange.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A participant's name with validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(String);

impl Name {
    /// Maximum allowed length for a participant name.
    pub const MAX_LENGTH: usize = 100;
    
    /// Get the inner string value.
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    /// Create a new token from a ULID string.
    #[must_use]
    pub fn new(ulid: String) -> Self {
        Self(ulid)
    }
    
    /// Get the inner ULID string.
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

/// Request to register a new participant.
#[derive(Serialize)]
pub struct RegistrationRequest {
    pub name: Name,
}

/// Response from participant registration.
#[derive(Deserialize)]
pub struct RegistrationResponse {
    pub ulid: String,
}

/// Request to submit an exercise solution.
#[derive(Serialize)]
pub struct SubmissionRequest {
    pub ulid: String,
    pub exercise_name: String,
    pub source_code: String,
    pub tests_passed: bool,
    pub clippy_passed: bool,
    pub fmt_passed: bool,
}

/// Status of a single exercise.
#[derive(Deserialize)]
pub struct ExerciseStatus {
    pub name: String,
    pub completed: bool,
    pub perfected: bool, // Better word than "polished"
}

/// Response containing participant progress.
#[derive(Deserialize)]
pub struct ProgressResponse {
    pub exercises: Vec<ExerciseStatus>,
}