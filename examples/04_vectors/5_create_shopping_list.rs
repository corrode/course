//! # Building a list from borrowed slices
//!
//! The trickiest of the four: each input is a `&str`, but the output is
//! a `Vec<String>`. Each borrowed slice has to become an owned `String`
//! somewhere along the way. The `String::from` / `.to_string()` /
//! `.to_owned()` family all do this conversion.

/// Creates a shopping list from the given items.
///
/// The trickiest of the four: each input is a `&str`, but the output is
/// a `Vec<String>`. Each borrowed slice has to become an owned `String`
/// somewhere along the way. The `String::from` / `.to_string()` /
/// `.to_owned()` family all do this.
fn create_shopping_list(items: &[&str]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shopping_list() {
        let items = ["bread", "milk", "eggs"];
        let list = create_shopping_list(&items);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], "bread");
    }
}
