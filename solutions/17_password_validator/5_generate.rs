use std::time::{SystemTime, UNIX_EPOCH};

struct PasswordGenerator {}

impl PasswordGenerator {
    /// Generates a "secure" password of the given length containing at
    /// least one uppercase letter, one lowercase letter, one digit, and
    /// one special character from `!@#$%^&*`.
    fn generate_secure_password(length: usize) -> String {
        let classes = [
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "abcdefghijklmnopqrstuvwxyz",
            "0123456789",
            "!@#$%^&*",
        ];

        // A poor person's randomness: the current time's nanoseconds.
        // Not cryptographically secure (use the `rand` crate for that),
        // but enough to vary the output between runs.
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.subsec_nanos() as usize);

        let mut password = String::with_capacity(length);
        for i in 0..length {
            // Cycling the class by position guarantees the first four
            // characters cover all four classes (for `length >= 4`), so
            // the result always passes the strength checks.
            let class = classes[i % classes.len()];
            let index = (seed + i) % class.chars().count();
            let ch = class.chars().nth(index).unwrap();
            password.push(ch);
        }
        password
    }
}

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
