//! # Adding items
//!
//! Now we modify the list in place. Where the previous step only read
//! from the `Vec`, this one needs to change it. The `&mut Vec<String>`
//! says "I need exclusive access for a moment", and that's what lets us
//! push a new item onto the end.

/// Adds an item to the shopping list.
///
/// Now we modify the list in place. The `&mut Vec<String>` says "I need
/// exclusive access for a moment", and that's what lets us add to it.
fn add_item(list: &mut Vec<String>, item: &str) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut list = vec!["bread".to_string()];
        add_item(&mut list, "butter");
        assert_eq!(list.len(), 2);
        assert_eq!(list[1], "butter");
    }
}
