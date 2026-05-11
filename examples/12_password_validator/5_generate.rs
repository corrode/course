//! # Generating a secure password
//!
//! Implement `generate_secure_password(length)` that returns a `String`
//! of the requested length, mixing uppercase letters, lowercase letters,
//! digits, and special characters so it would pass a strict validator.
//!
//! For variability without pulling in `rand`, you can use
//! `std::time::SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos()`
//! as a seed and cycle through your character sets. This is **not**
//! cryptographically secure — it's plenty for this exercise. In real code,
//! use the `rand` crate.
//!
//! Tips:
//! - Define one `&[u8]` (or `&str`) per character class.
//! - Make sure each class appears at least once so the result will pass
//!   strength checks regardless of `length` (assume `length >= 4`).
//! - The test below only checks the four character classes are present and
//!   the length is right; how you mix them is up to you.

struct PasswordGenerator {}

impl PasswordGenerator {
    /// Generates a "secure" password of the given length containing at
    /// least one uppercase letter, one lowercase letter, one digit, and
    /// one special character from `!@#$%^&*`.
    fn generate_secure_password(length: usize) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_length() {
        let password = PasswordGenerator::generate_secure_password(12);
        assert_eq!(password.chars().count(), 12);
    }

    #[test]
    fn test_generate_has_all_classes() {
        let password = PasswordGenerator::generate_secure_password(16);
        assert!(password.chars().any(|c| c.is_ascii_uppercase()));
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));
        assert!(password.chars().any(|c| c.is_ascii_digit()));
        assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));
    }
}
