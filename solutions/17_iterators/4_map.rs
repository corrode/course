/// Normalizes email addresses to lowercase.
///
/// Return one normalized address per input, in the same order, using an
/// iterator pipeline. See:
/// <https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase>
fn normalize_emails(emails: Vec<String>) -> Vec<String> {
    emails
        .into_iter()
        .map(|email| email.to_lowercase())
        .collect()
}

#[test]
fn test_normalize_emails() {
    let emails = vec!["Alice@EXAMPLE.COM".to_string(), "BOB@test.ORG".to_string()];
    let normalized = normalize_emails(emails);
    assert_eq!(normalized, vec!["alice@example.com", "bob@test.org"]);
}
