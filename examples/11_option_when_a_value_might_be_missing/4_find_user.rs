fn find_user_by_id(users: &[(u32, String)], id: u32) -> Option<&str> {
    todo!("Find the user by ID and return Some of the borrowed username, or None if missing")
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
