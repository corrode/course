/// Checks if the list contains a specific item.
///
/// You only need to read the list, comparing each element against `item`.
///
/// `Vec::contains` looks like a good fit, but its signature is `fn contains(&self, x: &T) -> bool`, and here that's `&String`, while you have a `&str`.
/// The most direct fix at this point in the course is a `for` loop.
/// We will cover iterators later.
fn contains_item(list: &Vec<String>, item: &str) -> bool {
    todo!()
}

#[test]
fn test_contains_item() {
    let list = vec!["apple".to_string(), "banana".to_string()];
    assert_eq!(contains_item(&list, "apple"), true);
    assert_eq!(contains_item(&list, "orange"), false);
}
