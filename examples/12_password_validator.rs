//! # Password Validator - Open-Ended Exercise
//!
//! You've made it to the creative part!
//! This one's more of a recreational exercise to celebrate your newly learned
//! skills. (Congrats, BTW!)
//!
//! Now it's just a matter of refining your skills, which requires practice,
//! practice, practice.
//!
//! Password security is a fascinating
//! field that combines computer science, psychology, and cryptography.
//! The first computer password was created in 1961 at MIT for their Compatible
//! Time-Sharing System.
//!
//! Today, password security is more important than ever. The infamous
//! "password123" and "qwerty" are still among the most common passwords
//! worldwide (please don't use them!). Your validator could help make
//! the internet a safer place.
//!
//! This is your chance to be creative and combine everything you've learned.
//!
//! Build something awesome - the future of secure authentication counts on you.
//!
//! Oh, and it's totally fine to change all the code in this file to make it your own.

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

struct PasswordValidator {}

impl PasswordValidator {
    /// Validates a password and returns detailed feedback.
    ///
    /// Base requirements (implement these first):
    /// - At least 8 characters: +20 points
    /// - Contains uppercase letter: +15 points  
    /// - Contains lowercase letter: +15 points
    /// - Contains number: +15 points
    /// - Contains special character (!@#$%^&*): +15 points
    /// - At least 12 characters: +10 points
    /// - At least 16 characters: +10 points
    ///
    /// Advanced features (try these for extra challenge):
    /// - Detect common passwords (add your own list)
    /// - Check for repeated characters (aaa, 111)  
    /// - Bonus for mixed character types in sequence
    /// - Penalty for keyboard patterns (qwerty, 123456)
    /// - Check against breach databases (simulate with common passwords)
    ///
    /// Set is_strong = true if score >= 70
    fn validate(password: &str) -> PasswordReport {
        // TODO: Implement your validation logic here!
        // Start with the base requirements, then get creative.
        //
        // Hints:
        // - Use .chars() to iterate through characters
        // - Use .any() to check if any character matches a condition
        // - Use .contains() to check for substrings
        // - Build your feedback Vec with helpful messages

        todo!()
    }
}

struct PasswordGenerator {}

impl PasswordGenerator {
    /// Generates a random secure password.
    /// Challenge: Implement a password generator that meets all your validation criteria!
    fn generate_secure_password(length: usize) -> String {
        // Optional: implement password generation
        // Hints:
        // - Define character sets (uppercase, lowercase, numbers, symbols)
        // - Use rand crate (you'll need to add it to Cargo.toml)
        // - Ensure the generated password passes your validator
        todo!()
    }
}

struct PasswordAdvisor {}

impl PasswordAdvisor {
    /// Suggests improvements for a weak password.
    /// Takes the PasswordReport and provides specific suggestions.
    fn suggest_improvements(report: &PasswordReport) -> Vec<String> {
        // Challenge: Analyze the feedback and provide actionable suggestions
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_passwords() {
        let weak_password = PasswordValidator::validate("12345");
        assert!(weak_password.score < 30);
        assert!(!weak_password.is_strong());
        assert!(!weak_password.feedback.is_empty());
    }

    #[test]
    fn test_medium_passwords() {
        let medium = PasswordValidator::validate("Password1");
        assert!(medium.score >= 30 && medium.score < 70);
    }

    #[test]
    fn test_strong_passwords() {
        let strong = PasswordValidator::validate("MySecure!Password123");
        assert!(strong.score >= 70);
        assert!(strong.is_strong());
    }

    #[test]
    fn test_feedback_quality() {
        let report = PasswordValidator::validate("weak");
        // Ensure feedback is helpful and specific
        assert!(
            report
                .feedback
                .iter()
                .any(|msg| msg.contains("characters") || msg.contains("length"))
        );
    }

    #[test]
    fn test_password_generation() {
        let password = PasswordGenerator::generate_secure_password(12);
        let report = PasswordValidator::validate(&password);
        assert!(report.is_strong());
    }

    #[test]
    fn test_advisor_suggestions() {
        let report = PasswordValidator::validate("weak");
        let suggestions = PasswordAdvisor::suggest_improvements(&report);
        assert!(!suggestions.is_empty());
    }

    // Add your own tests here!
    // Test edge cases, specific character requirements, etc.
}
