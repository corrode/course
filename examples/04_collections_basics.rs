//! # Collections: Storing Multiple Values
//!
//! Welcome to the wonderful world of collections! 🗃️
//!
//! Imagine trying to manage a library with individual variables for each book:
//! let book1, book2, book3... That would be madness! Collections let us 
//! store multiple related items together.
//!
//! Rust's collections are incredibly powerful. The Vec (vector) is like a 
//! resizable array, perfect for when you don't know how many items you'll have.
//! Spotify uses vectors to store your playlists, Netflix uses them for 
//! movie recommendations!
//!
//!## Bonus Exercises
//!
//! This lesson contains bonus exercises for curious learners.
//! These are optional and can be skipped if you're short on time.
//! You can run the bonus tests in this lesson with:
//!
//! ```sh
//! cargo test --example 04_collections_basics --features bonus
//! ```

/// Create a shopping list for a party
/// E-commerce carts work exactly like this
/// Expected output: an empty Vec<String> (new shopping list)
fn create_shopping_list() -> Vec<String> {
    todo!()
}

/// Add an item to the shopping list
/// Every "Add to Cart" button does this operation
/// Expected behavior: adds the item to the end of the list
fn add_item(list: &mut Vec<String>, item: String) {
    todo!()
}

/// Count how many items are in the list
/// Shopping apps show this count in the cart icon
/// Expected output: the number of items in the list (e.g., empty list -> 0)
fn count_items(list: &Vec<String>) -> usize {
    todo!()
}

/// Check if we have a specific item in our list
/// Search functionality in every app works like this
/// Expected output: true if the item exists in the list, false otherwise
fn has_item(list: &Vec<String>, item: &str) -> bool {
    todo!()
}

/// Remove an item from the list (if it exists)
/// This is what happens when you remove something from your cart
/// Expected output: true if item was found and removed, false if item wasn't in the list
fn remove_item(list: &mut Vec<String>, item: &str) -> bool {
    todo!()
}

/// Check if the shopping list is empty
/// E-commerce sites show "Your cart is empty" messages this way
/// Expected output: true if list has 0 items, false otherwise
/// See: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty
fn is_list_empty(list: &Vec<String>) -> bool {
    todo!()
}

/// Get the total number of characters in all item names
/// Useful for calculating storage space or display width
/// Expected output: sum of lengths of all item names (e.g., ["Egg", "Bread"] -> 8)
/// See: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter
fn total_characters(list: &Vec<String>) -> usize {
    todo!()
}

/// BONUS: Clear all items from the shopping list
/// Shopping carts need "Empty Cart" functionality
/// See: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear
fn clear_list(list: &mut Vec<String>) {
    todo!()
}

/// BONUS: Get the item at a specific position (index)
/// Lists often need to access items by position
/// See: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get
fn get_item_at_index(list: &Vec<String>, index: usize) -> String {
    todo!()
}

/// BONUS: Add multiple items at once to the list
/// Bulk operations are common in real applications
/// See: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend
fn add_multiple_items(list: &mut Vec<String>, items: Vec<String>) {
    todo!()
}

/// BONUS: Create a new list with all items in UPPERCASE
/// Data transformation is common in processing systems
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase
fn items_to_uppercase(list: &Vec<String>) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = create_shopping_list();
        assert_eq!(count_items(&list), 0);
    }

    #[test]
    fn test_adding_items() {
        let mut list = create_shopping_list();
        add_item(&mut list, "Bread".to_string());
        add_item(&mut list, "Milk".to_string());
        assert_eq!(count_items(&list), 2);
    }

    #[test]
    fn test_finding_items() {
        let mut list = create_shopping_list();
        add_item(&mut list, "Apples".to_string());
        add_item(&mut list, "Bananas".to_string());
        
        assert_eq!(has_item(&list, "Apples"), true);
        assert_eq!(has_item(&list, "Oranges"), false);
    }

    #[test]
    fn test_removing_items() {
        let mut list = create_shopping_list();
        add_item(&mut list, "Coffee".to_string());
        add_item(&mut list, "Tea".to_string());
        
        assert_eq!(remove_item(&mut list, "Coffee"), true);
        assert_eq!(count_items(&list), 1);
        assert_eq!(remove_item(&mut list, "Juice"), false); // Wasn't there
    }

    #[test]
    fn test_empty_check() {
        let list = create_shopping_list();
        assert_eq!(is_list_empty(&list), true);
        
        let mut list_with_items = create_shopping_list();
        add_item(&mut list_with_items, "Milk".to_string());
        assert_eq!(is_list_empty(&list_with_items), false);
    }

    #[test]
    fn test_character_counting() {
        let mut list = create_shopping_list();
        assert_eq!(total_characters(&list), 0);
        
        add_item(&mut list, "Egg".to_string()); // 3 characters
        add_item(&mut list, "Bread".to_string()); // 5 characters
        assert_eq!(total_characters(&list), 8); // 3 + 5 = 8
    }
}

#[cfg(all(test, feature = "bonus"))]
mod bonus {
    use super::*;

    #[test]
    fn test_list_clearing() {
        let mut list = vec!["Apple".to_string(), "Banana".to_string()];
        clear_list(&mut list);
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_index_access() {
        let list = vec!["First".to_string(), "Second".to_string(), "Third".to_string()];
        assert_eq!(get_item_at_index(&list, 0), "First");
        assert_eq!(get_item_at_index(&list, 1), "Second");
    }

    #[test]
    fn test_bulk_adding() {
        let mut list = vec!["Milk".to_string()];
        let new_items = vec!["Bread".to_string(), "Eggs".to_string()];
        add_multiple_items(&mut list, new_items);
        assert_eq!(list.len(), 3);
        assert!(list.contains(&"Bread".to_string()));
    }

    #[test]
    fn test_uppercase_transformation() {
        let list = vec!["apple".to_string(), "banana".to_string()];
        let uppercase_list = items_to_uppercase(&list);
        assert_eq!(uppercase_list, vec!["APPLE", "BANANA"]);
        // Original list unchanged
        assert_eq!(list, vec!["apple", "banana"]);
    }
}