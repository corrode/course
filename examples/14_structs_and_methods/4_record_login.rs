#[derive(Debug, PartialEq)]
struct User {
    email: String,
    name: String,
    is_verified: bool,
    login_count: u32,
}

impl User {
    const fn new(email: String, name: String) -> Self {
        Self {
            email,
            name,
            is_verified: false,
            login_count: 0,
        }
    }

    /// Records a successful login attempt.
    ///
    /// Increment `login_count` and set `is_verified` to `true`. Setting the
    /// verification flag is idempotent, so you can set it on every login
    /// without checking it first. We keep verification simple here; a real
    /// system would decide separately when an account counts as verified.
    fn record_login(&mut self) {
        todo!("Record a login by incrementing login_count and setting is_verified to true")
    }
}

#[test]
fn test_record_login() {
    let mut user = User::new("alice@example.com".to_string(), "Alice".to_string());

    user.record_login();
    assert_eq!(user.login_count, 1);
    assert_eq!(user.is_verified, true);

    user.record_login();
    assert_eq!(user.login_count, 2);
    // Verification stays on across subsequent logins (idempotent).
    assert!(user.is_verified);
}
