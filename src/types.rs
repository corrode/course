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

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A validated team / cohort token.
///
/// Stored on `participants.team_token`. Populated server-side from
/// the `/signup/{group_slug}` URL (the user never types it). Used
/// to bucket the admin participants table into one group per cohort
/// and to scope the per-team submissions view.
///
/// A `TeamToken` is always a non-empty slug of up to 64 characters
/// drawn from `[A-Za-z0-9_-]`. Anything else (HTML, path separators,
/// non-ASCII letters, oversized blobs) is rejected at the boundary
/// so the column never holds a value the templates can't safely
/// render or the URL router can't safely route on.
///
/// # Examples
///
/// ```
/// use cargo_course::types::TeamToken;
///
/// let token = TeamToken::try_from("veo-x9k2").unwrap();
/// assert_eq!(token.as_str(), "veo-x9k2");
///
/// // Whitespace is trimmed.
/// let token = TeamToken::try_from("  veo ").unwrap();
/// assert_eq!(token.as_str(), "veo");
///
/// // Spaces inside the slug, non-ASCII, oversized values are rejected.
/// assert!(TeamToken::try_from("team one").is_err());
/// assert!(TeamToken::try_from("équipe").is_err());
/// assert!(TeamToken::try_from("").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamToken(String);

/// Reasons a string can fail to parse into a [`TeamToken`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamTokenError {
    /// Empty string or only whitespace.
    Empty,
    /// More than [`TeamToken::MAX_LENGTH`] characters after trimming.
    TooLong,
    /// Contains a character outside `[A-Za-z0-9_-]`.
    InvalidChar,
}

impl fmt::Display for TeamTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "team token cannot be empty"),
            Self::TooLong => write!(
                f,
                "team token must be {} characters or fewer",
                TeamToken::MAX_LENGTH
            ),
            Self::InvalidChar => write!(
                f,
                "team token may only contain letters, digits, underscore, or hyphen"
            ),
        }
    }
}

impl std::error::Error for TeamTokenError {}

impl TeamToken {
    /// Maximum allowed length for a team token, after trimming.
    ///
    /// 64 chars is generous enough for any cohort label an instructor
    /// would actually pick, and tight enough that the value fits
    /// comfortably in the admin table and any URL we route on.
    pub const MAX_LENGTH: usize = 64;

    /// Returns the validated token as a string slice.
    ///
    /// The returned string is guaranteed to be non-empty, no longer
    /// than [`Self::MAX_LENGTH`] characters, and made up entirely of
    /// `[A-Za-z0-9_-]`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parses a free-text form input into an optional team token.
    ///
    /// Blank / whitespace-only inputs become `Ok(None)`: the
    /// participant is moved into the synthetic "Unassigned" bucket.
    /// Any other input has to be a valid slug per [`TryFrom<&str>`].
    ///
    /// This is the right entry point for the admin's inline
    /// re-assignment form and the `team_token` hidden input on the
    /// signup page, where blank means "no cohort" rather than "this
    /// is bad input".
    ///
    /// # Examples
    ///
    /// ```
    /// use cargo_course::types::TeamToken;
    ///
    /// assert_eq!(TeamToken::parse_form_input("").unwrap(), None);
    /// assert_eq!(TeamToken::parse_form_input("   ").unwrap(), None);
    /// assert_eq!(
    ///     TeamToken::parse_form_input("veo").unwrap().unwrap().as_str(),
    ///     "veo"
    /// );
    /// assert!(TeamToken::parse_form_input("<script>").is_err());
    /// ```
    pub fn parse_form_input(raw: &str) -> std::result::Result<Option<Self>, TeamTokenError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        Self::try_from(trimmed).map(Some)
    }
}

impl TryFrom<&str> for TeamToken {
    type Error = TeamTokenError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(TeamTokenError::Empty);
        }
        if trimmed.len() > Self::MAX_LENGTH {
            return Err(TeamTokenError::TooLong);
        }
        if !trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(TeamTokenError::InvalidChar);
        }
        Ok(Self(trimmed.to_string()))
    }
}

impl TryFrom<String> for TeamToken {
    type Error = TeamTokenError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl FromStr for TeamToken {
    type Err = TeamTokenError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl fmt::Display for TeamToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for TeamToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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
            return Err(anyhow!(
                "Name too long (max {} characters)",
                Self::MAX_LENGTH
            ));
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
#[derive(Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// The participant's chosen name (already validated)
    pub name: Name,
}

/// Response payload from successful participant registration.
///
/// The server generates a unique ULID token that the CLI
/// stores locally for subsequent requests.
#[derive(Serialize, Deserialize)]
pub struct RegistrationResponse {
    /// The generated ULID token for this participant
    pub ulid: String,
}

/// Request payload for submitting an exercise solution.
///
/// This contains the participant's code and the results of local
/// testing and linting checks performed by the CLI.
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct ProgressResponse {
    /// Status for each exercise in the course
    pub exercises: Vec<ExerciseStatus>,
}
