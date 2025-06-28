//! # Vector Basics
//!
//! Vectors are growable arrays - the most common collection type.
//! Learn basic operations: create, add, access, and length.

/// Creates a shopping list with the given items.
/// Returns a vector containing all the items.
fn create_shopping_list(items: &[&str]) -> Vec<String> {
    // Convert &str items to owned Strings and collect into Vec
    unimplemented!()
}

/// Adds an item to the shopping list.
/// Modifies the list in place.
fn add_item(list: &mut Vec<String>, item: &str) {
    // Use .push() to add item (convert to String first)
    unimplemented!()
}

/// Gets the number of items in the list.
fn count_items(list: &Vec<String>) -> usize {
    unimplemented!()
}

/// Checks if the list contains a specific item.
fn contains_item(list: &Vec<String>, item: &str) -> bool {
    // Use .contains() method
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_list() {
        let items = ["bread", "milk", "eggs"];
        let list = create_shopping_list(&items);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], "bread");
    }
    
    #[test]
    fn test_add_item() {
        let mut list = vec!["bread".to_string()];
        add_item(&mut list, "butter");
        assert_eq!(list.len(), 2);
        assert_eq!(list[1], "butter");
    }
    
    #[test]
    fn test_count_and_contains() {
        let list = vec!["apple".to_string(), "banana".to_string()];
        assert_eq!(count_items(&list), 2);
        assert_eq!(contains_item(&list, "apple"), true);
        assert_eq!(contains_item(&list, "orange"), false);
    }
}