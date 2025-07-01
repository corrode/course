//! # Vector Basics
//!
//! Arrays and lists have been fundamental to computing since the beginning.
//! John von Neumann, one of the pioneers of computer science, designed the
//! stored-program concept that makes modern computers possible - and arrays
//! were central to his vision.
//!
//! Rust's Vec<T> is like a smart, memory-safe version of C arrays, but with
//! superpowers! They can grow and shrink dynamically, and the compiler ensures
//! you never access memory you shouldn't. It's like having a very protective
//! but helpful coding buddy watching your back.
//!
//! Time to wrangle some data! Your vector journey starts here! 📊

/// Creates a shopping list with the given items.
/// Returns a vector containing all the items.
fn create_shopping_list(items: &[&str]) -> Vec<String> {
    // Convert &str items to owned Strings and collect into Vec
    todo!()
}

/// Adds an item to the shopping list.
/// Modifies the list in place.
fn add_item(list: &mut Vec<String>, item: &str) {
    // Use .push() to add item (convert to String first)
    todo!()
}

/// Gets the number of items in the list.
fn count_items(list: &Vec<String>) -> usize {
    todo!()
}

/// Checks if the list contains a specific item.
fn contains_item(list: &Vec<String>, item: &str) -> bool {
    // Use .contains() method
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