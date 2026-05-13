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
