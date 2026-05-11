//! # Warm-up: `is_strong`
//!
//! Welcome to the open-ended chapter. The whole exercise revolves around a
//! `PasswordReport` value: a structured verdict about a password, with a
//! numeric score, some human-readable feedback, and a coarse strength label.
//!
//! We start small. Before tackling the actual scoring, get a feel for the
//! data by implementing the one-line `is_strong` method on `PasswordReport`.
//! By convention in this exercise, "strong" means the score is at least
//! `70`.
//!
//! This step also introduces the shared `PasswordStrength` enum and
//! `PasswordReport` struct that every later step will reuse (each step
//! re-declares them so it can stand on its own).

#[derive(Debug, Clone)]
enum PasswordStrength {
    Weak,
    Medium,
    Strong,
}

#[derive(Debug, Clone)]
struct PasswordReport {
    input: String,
    score: u8, // 0-100
    feedback: Vec<String>,
    strength: PasswordStrength,
}

impl PasswordReport {
    /// Checks if the password is considered strong.
    fn is_strong(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report_with_score(score: u8) -> PasswordReport {
        PasswordReport {
            input: "example".to_string(),
            score,
            feedback: Vec::new(),
            strength: PasswordStrength::Weak,
        }
    }

    #[test]
    fn test_is_strong_high_score() {
        assert!(report_with_score(70).is_strong());
        assert!(report_with_score(90).is_strong());
    }

    #[test]
    fn test_is_strong_low_score() {
        assert!(!report_with_score(0).is_strong());
        assert!(!report_with_score(69).is_strong());
    }
}
