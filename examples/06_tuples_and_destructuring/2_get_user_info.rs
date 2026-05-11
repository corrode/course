//! # Returning multiple values
//!
//! Functions in Rust return a single value, but a tuple lets you bundle
//! several values into that single return. It's the lightest-weight way
//! to hand back more than one thing without defining a new type.
//!
//! Here you'll return a `(String, u32)` pair: a name and an age.

/// Returns a user's name and age as a tuple.
/// For example, return "Alice" and 25.
/// Useful for functions that need to return multiple values.
fn get_user_info() -> (String, u32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_info() {
        let (name, age) = get_user_info();
        assert_eq!(name, "Alice");
        assert_eq!(age, 25);
    }
}
