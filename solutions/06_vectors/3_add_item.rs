/// Adds an item to the shopping list.
///
/// Now we modify the list in place. The `&mut Vec<String>` says "I need
/// exclusive access for a moment", and that's what lets us add to it.
fn add_item(list: &mut Vec<String>, item: &str) {
    list.push(item.to_string());
}

#[test]
fn test_add_item() {
    let mut list = vec!["bread".to_string()];
    add_item(&mut list, "butter");
    assert_eq!(list.len(), 2);
    assert_eq!(list[1], "butter");
}
