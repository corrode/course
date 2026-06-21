/// Returns all users whose usernames start with 'a'.
///
/// Same idea, but instead of transforming each element you keep some
/// and drop others. Watch out for one borrowing gotcha: the closure
/// receives a reference to each element, not the element itself.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.starts_with>
fn select_usernames_starting_with_a(usernames: Vec<&str>) -> Vec<&str> {
    usernames
        .into_iter()
        .filter(|username| username.starts_with('a'))
        .collect()
}

#[test]
fn test_select_usernames_starting_with_a() {
    let users = vec!["alice", "admin", "bob", "anonymous", "charlie"];
    let active = select_usernames_starting_with_a(users);
    assert_eq!(active, vec!["alice", "admin", "anonymous"]);
}
