struct PasswordGenerator {}

impl PasswordGenerator {
    /// Generates deterministic test data ONLY, never real secrets.
    ///
    /// Reject lengths below 4 with `"Need at least 4 characters."`. Otherwise,
    /// return exactly `length` ASCII characters drawn from uppercase letters,
    /// lowercase letters, digits, and `!@#$%^&*`, including at least one of
    /// each. No randomness or clock is needed; this is not a secure password
    /// generator.
    fn generate_example_password(length: usize) -> Result<String, &'static str> {
        if length < 4 {
            return Err("Need at least 4 characters.");
        }
        Ok("Aa1!".chars().cycle().take(length).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_example_contract(length: usize) {
        let password = PasswordGenerator::generate_example_password(length)
            .expect("lengths of at least four must succeed");
        assert_eq!(password.len(), length);
        assert_eq!(password.chars().count(), length);
        assert!(password.is_ascii());
        assert!(password.chars().all(|c| {
            c.is_ascii_uppercase()
                || c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || "!@#$%^&*".contains(c)
        }));
        assert!(password.chars().any(|c| c.is_ascii_uppercase()));
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));
        assert!(password.chars().any(|c| c.is_ascii_digit()));
        assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));
    }

    #[test]
    fn rejects_every_length_below_four() {
        for length in 0..4 {
            assert_eq!(
                PasswordGenerator::generate_example_password(length),
                Err("Need at least 4 characters.")
            );
        }
    }

    #[test]
    fn minimum_length_includes_all_four_classes() {
        assert_example_contract(4);
    }

    #[test]
    fn handles_odd_even_and_longer_lengths() {
        for length in [5, 8, 12, 16, 33] {
            assert_example_contract(length);
        }
    }
}
