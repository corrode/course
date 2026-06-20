/// Returns `true` if the password contains at least one ASCII uppercase letter.
fn has_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}

/// Returns `true` if the password contains at least one ASCII lowercase letter.
fn has_lowercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_lowercase())
}

/// Returns `true` if the password contains at least one ASCII digit.
fn has_digit(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_digit())
}

/// Returns `true` if the password contains at least one character from
/// the set `!@#$%^&*`.
fn has_special(password: &str) -> bool {
    password.chars().any(|c| "!@#$%^&*".contains(c))
}

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
