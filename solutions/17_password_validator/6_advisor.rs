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
        let mut suggestions = Vec::new();
        for complaint in &report.feedback {
            let complaint = complaint.to_lowercase();
            if complaint.contains("short")
                || complaint.contains("length")
                || complaint.contains("character")
            {
                suggestions.push("Make it longer: aim for at least 12 characters.".to_string());
            } else if complaint.contains("uppercase") {
                suggestions.push("Mix in an uppercase letter (A-Z).".to_string());
            } else if complaint.contains("lowercase") {
                suggestions.push("Mix in a lowercase letter (a-z).".to_string());
            } else if complaint.contains("digit") {
                suggestions.push("Add a digit (0-9).".to_string());
            } else if complaint.contains("special") {
                suggestions.push("Add a special character like !@#$%^&*.".to_string());
            }
        }
        suggestions
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
