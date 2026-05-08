//! # Vector Basics
//!
//! Arrays and lists have been fundamental to computing since the beginning.
//! John von Neumann, one of the pioneers of computer science, designed the
//! stored-program concept that makes modern computers possible, and arrays
//! were central to his vision.
//!
//! Rust's `Vec<T>` is a growable, heap-allocated array. It can grow and
//! shrink at runtime, and the compiler ensures you never read or write past
//! its bounds.

/// Creates a shopping list with the given items.
/// Returns a vector containing all the items.
fn create_shopping_list(items: &[&str]) -> Vec<String> {
    // Convert &str items to owned Strings and collect into Vec
    todo!()
}

/// Adds an item to the shopping list.
/// Modifies the list in place.
fn add_item(list: &mut Vec<String>, item: &str) {
    todo!()
}

/// Gets the number of items in the list.
fn count_items(list: &Vec<String>) -> usize {
    todo!()
}

/// Checks if the list contains a specific item.
fn contains_item(list: &Vec<String>, item: &str) -> bool {
    todo!()
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
