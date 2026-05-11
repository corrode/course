//! # Producing an Option
//!
//! Now you have to produce an `Option` instead of consume one. You could
//! write the empty-check yourself with `if items.is_empty()`, but slices
//! already have a method that returns exactly this shape.
//!
//! See: <https://doc.rust-lang.org/std/primitive.slice.html>

/// Returns `Some(item)` if the list has items, `None` if empty.
fn get_first_item(items: &[String]) -> Option<&String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_item() {
        let items = vec!["first".to_string(), "second".to_string()];
        assert_eq!(get_first_item(&items), Some(&"first".to_string()));

        let empty: Vec<String> = vec![];
        assert_eq!(get_first_item(&empty), None);
    }
}
