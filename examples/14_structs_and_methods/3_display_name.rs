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

    fn display_name(&self) -> String {
        todo!("Return the user's display name as \"{{name}} ({{email}})\"")
    }
}

#[test]
fn test_display_name() {
    let user = User::new("alice@example.com".to_string(), "Alice".to_string());
    assert_eq!(user.display_name(), "Alice (alice@example.com)");
}
