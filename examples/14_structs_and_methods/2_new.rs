#[derive(Debug, PartialEq)]
struct User {
    email: String,
    name: String,
    is_verified: bool,
    login_count: u32,
}

impl User {
    fn new(email: String, name: String) -> Self {
        todo!("Create a user with the supplied email and name, unverified and with 0 logins")
    }
}

#[test]
fn test_new() {
    let user = User::new("alice@example.com".to_string(), "Alice".to_string());
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.name, "Alice");
    assert_eq!(user.login_count, 0);
    assert_eq!(user.is_verified, false);
}
