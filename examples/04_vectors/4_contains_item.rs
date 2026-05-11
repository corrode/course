//! # Searching the list
//!
//! Back to a read-only operation, but now we have to compare each
//! element against the item we're looking for. This is where the
//! borrowed-vs-owned distinction starts to bite: the `Vec` holds
//! `String`s, but we're searching with a `&str`.

/// Checks if the list contains a specific item.
///
/// A read-only operation again, but this time we have to compare each
/// element against `item`.
///
/// Heads-up: you'll want to reach for `Vec::contains`, but its
/// signature is `fn contains(&self, x: &T) -> bool`, and here that's `&String`,
/// while we have a `&str`. The most direct fix at this point in the course
/// is a `for` loop:
///
/// ```ignore
/// for name in list {
///     if name == item {
///         return true;
///     }
/// }
/// false
/// ```
///
/// We will cover iterators in chapter 11.
fn contains_item(list: &Vec<String>, item: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_item() {
        let list = vec!["apple".to_string(), "banana".to_string()];
        assert_eq!(contains_item(&list, "apple"), true);
        assert_eq!(contains_item(&list, "orange"), false);
    }
}
