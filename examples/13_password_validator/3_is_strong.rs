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
