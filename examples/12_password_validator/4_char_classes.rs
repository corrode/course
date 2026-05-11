//! # Character-class helpers
//!
//! The validator's scoring rules all boil down to questions like "does this
//! password contain an uppercase letter?" Before assembling the orchestrator,
//! write the small predicates that answer each one.
//!
//! All four functions take a `&str` and return `bool`. The "special"
//! character set for this exercise is `!@#$%^&*` — feel free to expand it if
//! you want a stricter validator later.
//!
//! Hint: `str::chars()` plus `Iterator::any` is the natural shape here.

/// Returns `true` if the password contains at least one ASCII uppercase letter.
fn has_uppercase(password: &str) -> bool {
    todo!()
}

/// Returns `true` if the password contains at least one ASCII lowercase letter.
fn has_lowercase(password: &str) -> bool {
    todo!()
}

/// Returns `true` if the password contains at least one ASCII digit.
fn has_digit(password: &str) -> bool {
    todo!()
}

/// Returns `true` if the password contains at least one character from
/// the set `!@#$%^&*`.
fn has_special(password: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_uppercase() {
        assert!(has_uppercase("Hello"));
        assert!(!has_uppercase("hello"));
        assert!(!has_uppercase("12345"));
    }

    #[test]
    fn test_has_lowercase() {
        assert!(has_lowercase("Hello"));
        assert!(!has_lowercase("HELLO"));
        assert!(!has_lowercase("12345"));
    }

    #[test]
    fn test_has_digit() {
        assert!(has_digit("abc1"));
        assert!(!has_digit("abcdef"));
    }

    #[test]
    fn test_has_special() {
        assert!(has_special("hi!"));
        assert!(has_special("a@b"));
        assert!(!has_special("plain"));
        assert!(!has_special("with space"));
    }
}
