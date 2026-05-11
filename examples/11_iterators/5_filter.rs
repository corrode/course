//! # Keeping elements with `filter`
//!
//! Same idea as `map`, but instead of transforming each element you keep
//! some and drop others. Watch out for one borrowing gotcha: `.iter()`
//! yields `&T`, but `filter` gives its closure another reference on top,
//! so the closure sees `&&T`.
//!
//! That's why you'll often see `**c == ...` or `s.starts_with(...)` (which
//! auto-derefs) instead of plain `c == ...`. Don't be alarmed when the
//! compiler complains about a missing `&`, see
//! [the cheatsheet](/cheatsheet) entry on iterators.

/// Returns all users whose usernames start with 'a'.
///
/// Same idea, but instead of transforming each element you keep some
/// and drop others. Watch out for one borrowing gotcha: the closure
/// receives a reference to each element, not the element itself.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.starts_with>
fn select_usernames_starting_with_a(usernames: Vec<&str>) -> Vec<&str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_usernames_starting_with_a() {
        let users = vec!["alice", "admin", "bob", "anonymous", "charlie"];
        let active = select_usernames_starting_with_a(users);
        assert_eq!(active, vec!["alice", "admin", "anonymous"]);
    }
}
