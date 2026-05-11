//! # Turning feedback into suggestions
//!
//! The validator (next step) produces a `PasswordReport` with a list of
//! short complaints in `feedback`, like `"too short"` or `"missing digit"`.
//! This step turns those complaints into actionable, human-readable
//! suggestions.
//!
//! Decide on your own format. A reasonable approach is to scan each
//! feedback string for keywords ("short", "uppercase", "digit", ...) and
//! emit a matching suggestion ("Add at least 4 more characters", "Mix in
//! an uppercase letter like A-Z", ...).
//!
//! The shared types are duplicated here so this step compiles on its own.

#[derive(Debug, Clone)]
enum PasswordStrength {
    Weak,
    Medium,
    Strong,
}

#[derive(Debug, Clone)]
struct PasswordReport {
    input: String,
    score: u8,
    feedback: Vec<String>,
    strength: PasswordStrength,
}

struct PasswordAdvisor {}

impl PasswordAdvisor {
    /// Returns a list of human-readable suggestions for improving a
    /// password, derived from the report's `feedback`. If the report has
    /// no complaints, it's fine to return an empty `Vec`.
    fn suggest_improvements(report: &PasswordReport) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weak_report() -> PasswordReport {
        PasswordReport {
            input: "weak".to_string(),
            score: 15,
            feedback: vec![
                "too short".to_string(),
                "missing uppercase".to_string(),
                "missing digit".to_string(),
            ],
            strength: PasswordStrength::Weak,
        }
    }

    #[test]
    fn test_suggest_improvements_nonempty() {
        let suggestions = PasswordAdvisor::suggest_improvements(&weak_report());
        assert!(!suggestions.is_empty());
    }
}
