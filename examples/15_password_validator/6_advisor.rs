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
