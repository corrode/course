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

impl PasswordReport {
    /// Checks if the password is considered strong (score >= 70).
    fn is_strong(&self) -> bool {
        self.score >= 70
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
    /// Validates `password` and returns a detailed report.
    ///
    /// See the module docs for the suggested scoring scheme. Once the base
    /// rules pass the tests, get creative: detect common passwords, repeated
    /// runs (`aaa`, `111`), keyboard patterns (`qwerty`, `123456`), or add
    /// a bonus for variety. None of those are required by the tests.
    fn validate(password: &str) -> PasswordReport {
        let length = password.chars().count();
        let mut score: u32 = 0;
        let mut feedback = Vec::new();

        // Length: longer is better, in tiers.
        if length >= 16 {
            score += 25;
        } else if length >= 12 {
            score += 20;
        } else if length >= 8 {
            score += 10;
        } else {
            feedback.push("Password is too short; use at least 8 characters.".to_string());
        }

        // Character variety: one rule per class.
        if has_uppercase(password) {
            score += 15;
        } else {
            feedback.push("Add an uppercase letter.".to_string());
        }
        if has_lowercase(password) {
            score += 15;
        } else {
            feedback.push("Add a lowercase letter.".to_string());
        }
        if has_digit(password) {
            score += 15;
        } else {
            feedback.push("Add a digit.".to_string());
        }
        if has_special(password) {
            score += 15;
        } else {
            feedback.push("Add a special character (!@#$%^&*).".to_string());
        }

        let score = score.min(100) as u8;
        let strength = if score >= 70 {
            PasswordStrength::Strong
        } else if score >= 30 {
            PasswordStrength::Medium
        } else {
            PasswordStrength::Weak
        };

        PasswordReport {
            input: password.to_string(),
            score,
            feedback,
            strength,
        }
    }
}

#[test]
fn test_validate_weak() {
    let weak = PasswordValidator::validate("12345");
    assert!(weak.score < 30);
    assert!(!weak.is_strong());
    assert!(!weak.feedback.is_empty());
}

#[test]
fn test_validate_medium() {
    let medium = PasswordValidator::validate("Password1");
    assert!(medium.score >= 30 && medium.score < 70);
}

#[test]
fn test_validate_strong() {
    let strong = PasswordValidator::validate("MySecure!Password123");
    assert!(strong.score >= 70);
    assert!(strong.is_strong());
}

#[test]
fn test_validate_feedback_mentions_length() {
    let report = PasswordValidator::validate("weak");
    // "too short", "at least 8 characters", "increase the length", etc.
    // are all reasonable phrasings; accept any of them.
    let mentions_length = report.feedback.iter().any(|msg| {
        let m = msg.to_lowercase();
        m.contains("character")
            || m.contains("length")
            || m.contains("short")
            || m.contains("longer")
            || m.contains("at least")
    });
    assert!(
        mentions_length,
        "feedback should mention the password being too short; got {:?}",
        report.feedback
    );
}
