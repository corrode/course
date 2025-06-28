//! # Password Validator - Open-Ended Exercise
//!
//! Create a comprehensive password validation system.
//! This is your chance to combine everything you've learned and be creative!

#[derive(Debug, Clone)]
struct PasswordStrength {
    score: u8,          // 0-100
    feedback: Vec<String>,
    is_strong: bool,
}

impl PasswordStrength {
    fn new() -> Self {
        Self {
            score: 0,
            feedback: Vec::new(),
            is_strong: false,
        }
    }
}

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
fn validate_password(password: &str) -> PasswordStrength {
    let mut strength = PasswordStrength::new();
    
    // TODO: Implement your validation logic here!
    // Start with the base requirements, then get creative.
    // 
    // Hints:
    // - Use .chars() to iterate through characters
    // - Use .any() to check if any character matches a condition
    // - Use .contains() to check for substrings
    // - Build your feedback Vec with helpful messages
    
    unimplemented!()
}

/// Generates a random secure password.
/// Challenge: Implement a password generator that meets all your validation criteria!
fn generate_secure_password(length: usize) -> String {
    // Optional: implement password generation
    // Hints: 
    // - Define character sets (uppercase, lowercase, numbers, symbols)
    // - Use rand crate (you'll need to add it to Cargo.toml)
    // - Ensure the generated password passes your validator
    unimplemented!()
}

/// Suggests improvements for a weak password.
/// Takes the PasswordStrength result and provides specific suggestions.
fn suggest_improvements(strength: &PasswordStrength) -> Vec<String> {
    // Challenge: Analyze the feedback and provide actionable suggestions
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_weak_passwords() {
        let weak = validate_password("123");
        assert!(weak.score < 30);
        assert!(!weak.is_strong);
        assert!(!weak.feedback.is_empty());
    }
    
    #[test]
    fn test_medium_passwords() {
        let medium = validate_password("Password1");
        assert!(medium.score >= 30 && medium.score < 70);
    }
    
    #[test]
    fn test_strong_passwords() {
        let strong = validate_password("MySecure!Password123");
        assert!(strong.score >= 70);
        assert!(strong.is_strong);
    }
    
    #[test]
    fn test_feedback_quality() {
        let result = validate_password("weak");
        // Ensure feedback is helpful and specific
        assert!(result.feedback.iter().any(|msg| msg.contains("characters") || msg.contains("length")));
    }
    
    // Add your own tests here!
    // Test edge cases, specific character requirements, etc.
}