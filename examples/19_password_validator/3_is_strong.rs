#[derive(Debug, Clone, PartialEq, Eq)]
enum PasswordStrength {
    Weak,
    Medium,
    Strong,
}

impl PasswordStrength {
    /// Classifies any u8: 0..=29 is Weak, 30..=69 is Medium, 70..=255 is Strong.
    const fn from_score(score: u8) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct PasswordReport {
    score: u8,
    feedback: Vec<String>,
    strength: PasswordStrength,
}

impl PasswordReport {
    /// Checks the stored strength label; does not recompute it from the score.
    fn is_strong(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_weak_boundaries() {
        assert_eq!(PasswordStrength::from_score(0), PasswordStrength::Weak);
        assert_eq!(PasswordStrength::from_score(29), PasswordStrength::Weak);
    }

    #[test]
    fn classifies_medium_boundaries() {
        assert_eq!(PasswordStrength::from_score(30), PasswordStrength::Medium);
        assert_eq!(PasswordStrength::from_score(69), PasswordStrength::Medium);
    }

    #[test]
    fn classifies_strong_through_the_full_u8_range() {
        for score in [70, 100, 255] {
            assert_eq!(
                PasswordStrength::from_score(score),
                PasswordStrength::Strong
            );
        }
    }

    #[test]
    fn weak_report_is_not_strong() {
        let report = PasswordReport {
            score: 29,
            feedback: vec!["Add an uppercase ASCII letter.".to_string()],
            strength: PasswordStrength::Weak,
        };
        assert!(!report.is_strong());
    }

    #[test]
    fn medium_report_is_not_strong() {
        let report = PasswordReport {
            score: 69,
            feedback: Vec::new(),
            strength: PasswordStrength::Medium,
        };
        assert!(!report.is_strong());
    }

    #[test]
    fn strong_report_is_strong() {
        let report = PasswordReport {
            score: 70,
            feedback: Vec::new(),
            strength: PasswordStrength::Strong,
        };
        assert!(report.is_strong());
    }

    #[test]
    fn stored_label_is_authoritative_even_if_a_report_is_manually_inconsistent() {
        // This method reads the label, not the score. Only the validator guarantees
        // consistency; a manually constructed report need not have that invariant.
        for (score, strength, expected) in [
            (100, PasswordStrength::Weak, false),
            (100, PasswordStrength::Medium, false),
            (0, PasswordStrength::Strong, true),
        ] {
            let report = PasswordReport {
                score,
                feedback: Vec::new(),
                strength,
            };
            assert_eq!(report.is_strong(), expected);
        }
    }
}
