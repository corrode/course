/// Finds the first user with this ID, or returns `None`.
fn find_user_by_id(users: &[(u32, String)], id: u32) -> Option<&str> {
    users
        .iter()
        .find(|(uid, _)| *uid == id)
        .map(|(_, name)| name.as_str())
}

#[test]
fn test_find_user_by_id() {
    let mut users = vec![(1, String::from("alice")), (2, String::from("bob"))];
    assert_eq!(find_user_by_id(&users, 2).as_deref(), Some("bob"));
    assert_eq!(find_user_by_id(&users, 99).as_deref(), None);
    users.push((3, String::from("charlie")));
    assert_eq!(find_user_by_id(&users, 3).as_deref(), Some("charlie"));
}

#[test]
fn test_empty_records() {
    assert_eq!(find_user_by_id(&[], 1).as_deref(), None);
}

#[test]
fn test_first_match() {
    let users = [
        (7, String::from("Férris 🦀")),
        (7, String::from("impostor")),
    ];
    assert_eq!(find_user_by_id(&users, 7).as_deref(), Some("Férris 🦀"));
}

#[test]
fn test_username_survives_records() {
    todo!("Remove this todo and uncomment the survival check below");

    /*
    let username = {
        let users = [(42, String::from("Férris 🦀"))];
        find_user_by_id(&users, 42)
    };
    assert_eq!(username.as_deref(), Some("Férris 🦀"));
    */
}
