#[derive(Debug, Clone, PartialEq, Eq)]
enum PasswordStrength {
    Weak,
    Medium,
    Strong,
}

impl PasswordStrength {
    /// Classifies any u8: 0..=29 is Weak, 30..=69 is Medium, 70..=255 is
    /// Strong.
    const fn from_score(score: u8) -> Self {
        match score {
            0..=29 => Self::Weak,
            30..=69 => Self::Medium,
            _ => Self::Strong,
        }
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
        self.strength == PasswordStrength::Strong
    }
}

fn has_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}

fn has_lowercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_lowercase())
}

fn has_digit(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_digit())
}

fn has_special(password: &str) -> bool {
    password.chars().any(|c| "!@#$%^&*".contains(c))
}

struct PasswordValidator {}

impl PasswordValidator {
    /// Applies a toy scoring scheme, not a real-world password security
    /// assessment.
    ///
    /// Count Unicode scalar values without trimming or normalization. Lengths
    /// of at least 8, 12, and 16 earn 20, 10, and 10 cumulative points. Each
    /// ASCII character class earns 15 points. Report only failed base rules, in
    /// order: length >= 8, uppercase, lowercase, digit, special (`!@#$%^&*`).
    /// Never store the input in the report.
    fn validate(password: &str) -> PasswordReport {
        todo!("Return a password report using the specified scoring and feedback rules")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_report(password: &str, score: u8, feedback: &[&str], strength: PasswordStrength) {
        let report = PasswordValidator::validate(password);
        assert_eq!(report.score, score);
        assert_eq!(report.feedback, feedback);
        assert_eq!(report.strength, strength);
        assert_eq!(report.strength, PasswordStrength::from_score(report.score));
        assert_eq!(report.is_strong(), strength == PasswordStrength::Strong);
    }

    #[test]
    fn scores_exact_length_boundaries_with_all_classes_held_constant() {
        for (length, score) in [(7, 60), (8, 80), (11, 80), (12, 90), (15, 90), (16, 100)] {
            let password = format!("Aa1!{}", "?".repeat(length - 4));
            let feedback: &[&str] = if length < 8 {
                &["Use at least 8 characters."]
            } else {
                &[]
            };
            let strength = if length < 8 {
                PasswordStrength::Medium
            } else {
                PasswordStrength::Strong
            };
            assert_report(&password, score, feedback, strength);
        }
    }

    #[test]
    fn reports_only_missing_base_length() {
        assert_report(
            "Aa1!",
            60,
            &["Use at least 8 characters."],
            PasswordStrength::Medium,
        );
    }

    #[test]
    fn reports_only_missing_uppercase() {
        assert_report(
            "aa1!????",
            65,
            &["Add an uppercase ASCII letter."],
            PasswordStrength::Medium,
        );
    }

    #[test]
    fn reports_only_missing_lowercase() {
        assert_report(
            "AA1!????",
            65,
            &["Add a lowercase ASCII letter."],
            PasswordStrength::Medium,
        );
    }

    #[test]
    fn reports_only_missing_digit() {
        assert_report(
            "Aaa!????",
            65,
            &["Add an ASCII digit."],
            PasswordStrength::Medium,
        );
    }

    #[test]
    fn reports_only_missing_special() {
        assert_report(
            "Aa11?_-.",
            65,
            &["Add one of !@#$%^&*."],
            PasswordStrength::Medium,
        );
    }

    #[test]
    fn empty_input_has_zero_score_and_all_five_messages_in_order() {
        assert_report(
            "",
            0,
            &[
                "Use at least 8 characters.",
                "Add an uppercase ASCII letter.",
                "Add a lowercase ASCII letter.",
                "Add an ASCII digit.",
                "Add one of !@#$%^&*.",
            ],
            PasswordStrength::Weak,
        );
    }

    #[test]
    fn multiple_missing_rules_keep_base_rule_order() {
        assert_report(
            "a",
            15,
            &[
                "Use at least 8 characters.",
                "Add an uppercase ASCII letter.",
                "Add an ASCII digit.",
                "Add one of !@#$%^&*.",
            ],
            PasswordStrength::Weak,
        );
    }

    #[test]
    fn counts_unicode_scalars_not_bytes_at_every_length_boundary() {
        for (length, score) in [(7, 60), (8, 80), (11, 80), (12, 90), (15, 90), (16, 100)] {
            let password = format!("Aa1!{}", "é".repeat(length - 4));
            let feedback: &[&str] = if length < 8 {
                &["Use at least 8 characters."]
            } else {
                &[]
            };
            let strength = if length < 8 {
                PasswordStrength::Medium
            } else {
                PasswordStrength::Strong
            };
            assert_report(&password, score, feedback, strength);
        }
    }

    #[test]
    fn unicode_letters_and_arabic_digits_contribute_only_to_length() {
        for (length, score, strength) in [
            (8, 20, PasswordStrength::Weak),
            (12, 30, PasswordStrength::Medium),
            (16, 40, PasswordStrength::Medium),
        ] {
            let password: String = "Éé١".chars().cycle().take(length).collect();
            assert_report(
                &password,
                score,
                &[
                    "Add an uppercase ASCII letter.",
                    "Add a lowercase ASCII letter.",
                    "Add an ASCII digit.",
                    "Add one of !@#$%^&*.",
                ],
                strength,
            );
        }
    }

    #[test]
    fn every_allowed_special_symbol_earns_points() {
        for special in "!@#$%^&*".chars() {
            let password = format!("Aa1{special}????");
            assert_report(&password, 80, &[], PasswordStrength::Strong);
        }
    }

    #[test]
    fn whitespace_is_not_trimmed() {
        assert_report(" Aa1! \t\n", 80, &[], PasswordStrength::Strong);
    }

    #[test]
    fn combining_marks_are_separate_scalars_without_normalization() {
        assert_report("Aa1!e\u{301}??", 80, &[], PasswordStrength::Strong);
    }

    #[test]
    fn long_inputs_do_not_truncate_the_count_or_repeat_class_points() {
        for length in [255, 256, 257, 300, 512] {
            let password = format!("Aa1!{}", "é".repeat(length - 4));
            assert_report(&password, 100, &[], PasswordStrength::Strong);
        }
        assert_report(&"Aa1!".repeat(100), 100, &[], PasswordStrength::Strong);
    }

    #[test]
    fn borrowed_input_remains_unchanged_and_reusable() {
        let password = String::from("Aa1!????");
        let report = PasswordValidator::validate(&password);
        assert_eq!(password, "Aa1!????");
        assert_eq!(report.score, 80);
        assert_report(&password, 80, &[], PasswordStrength::Strong);
    }
}
