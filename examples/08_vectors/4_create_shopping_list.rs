/// Creates a shopping list from the given items.
///
/// Each input is a borrowed `&str`; the returned `Vec<String>` owns its text.
fn create_shopping_list(items: &[&str]) -> Vec<String> {
    todo!("Build an owned shopping list from the borrowed items")
}

#[test]
fn test_create_shopping_list() {
    let items = ["bread", "milk", "eggs"];
    let list = create_shopping_list(&items);
    assert_eq!(list.len(), 3);
    assert_eq!(list[0], "bread");
}
