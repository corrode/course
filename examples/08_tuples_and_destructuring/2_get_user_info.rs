/// Returns a user's name and age as a tuple.
/// For example, return "Alice" and 25.
/// Useful for functions that need to return multiple values.
fn get_user_info() -> (String, u32) {
    todo!()
}

#[test]
fn test_get_user_info() {
    let (name, age) = get_user_info();
    assert_eq!(name, "Alice");
    assert_eq!(age, 25);
}
