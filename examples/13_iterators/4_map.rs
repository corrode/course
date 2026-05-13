/// Normalizes email addresses to lowercase.
///
/// Now you need to transform every element instead of collapsing the
/// sequence. The shape is `vec.into_iter()` -> some combinator that
/// applies a closure -> back to a `Vec` via `collect()`.
/// See: <https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase>
fn normalize_emails(emails: Vec<String>) -> Vec<String> {
    todo!()
}

#[test]
fn test_normalize_emails() {
    let emails = vec!["Alice@EXAMPLE.COM".to_string(), "BOB@test.ORG".to_string()];
    let normalized = normalize_emails(emails);
    assert_eq!(normalized, vec!["alice@example.com", "bob@test.org"]);
}
