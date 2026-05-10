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

/// Gets the number of items in the list.
///
/// Start here. The simplest possible operation: ask a `Vec` how long it
/// is. Notice the parameter is `&Vec<String>` — a borrow, so the caller
/// keeps ownership.
/// See: <https://doc.rust-lang.org/std/vec/struct.Vec.html>
fn count_items(list: &Vec<String>) -> usize {
    todo!()
}

/// Adds an item to the shopping list.
///
/// Now we modify the list in place. The `&mut Vec<String>` says "I need
/// exclusive access for a moment" — that's what lets us add to it.
fn add_item(list: &mut Vec<String>, item: &str) {
    todo!()
}

/// Checks if the list contains a specific item.
///
/// A read-only operation again, but this time we have to compare each
/// element against `item`. You can spell this out with a `for` loop;
/// `Vec` also has a one-call method that does exactly this.
fn contains_item(list: &Vec<String>, item: &str) -> bool {
    todo!()
}

/// Creates a shopping list from the given items.
///
/// The trickiest of the four: each input is a `&str`, but the output is
/// a `Vec<String>`. Each borrowed slice has to become an owned `String`
/// somewhere along the way — the `String::from` / `.to_string()` /
/// `.to_owned()` family all do this.
fn create_shopping_list(items: &[&str]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_and_contains() {
        let list = vec!["apple".to_string(), "banana".to_string()];
        assert_eq!(count_items(&list), 2);
        assert_eq!(contains_item(&list, "apple"), true);
        assert_eq!(contains_item(&list, "orange"), false);
    }

    #[test]
    fn test_add_item() {
        let mut list = vec!["bread".to_string()];
        add_item(&mut list, "butter");
        assert_eq!(list.len(), 2);
        assert_eq!(list[1], "butter");
    }

    #[test]
    fn test_create_list() {
        let items = ["bread", "milk", "eggs"];
        let list = create_shopping_list(&items);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], "bread");
    }
}
