/// Checks if the list contains a specific item.
///
/// A read-only operation again, but this time we have to compare each
/// element against `item`.
///
/// Heads-up: you'll want to reach for `Vec::contains`, but its
/// signature is `fn contains(&self, x: &T) -> bool`, and here that's `&String`,
/// while we have a `&str`. The most direct fix at this point in the course
/// is a `for` loop. We will cover iterators later.
fn contains_item(list: &Vec<String>, item: &str) -> bool {
    for entry in list {
        if entry == item {
            return true;
        }
    }
    false
}

#[test]
fn test_contains_item() {
    let list = vec!["apple".to_string(), "banana".to_string()];
    assert_eq!(contains_item(&list, "apple"), true);
    assert_eq!(contains_item(&list, "orange"), false);
}
