/// Finds a user by ID. Returns `Some(username)` if found, `None` if not.
fn find_user_by_id(users: &[(u32, String)], id: u32) -> Option<&str> {
    users
        .iter()
        .find(|(uid, _)| *uid == id)
        .map(|(_, name)| name.as_str())
}

#[test]
fn test_find_user_by_id() {
    let users = [
        (1, "alice".to_string()),
        (2, "bob".to_string()),
        (3, "charlie".to_string()),
    ];
    assert_eq!(find_user_by_id(&users, 2), Some("bob"));
    assert_eq!(find_user_by_id(&users, 99), None);
}
