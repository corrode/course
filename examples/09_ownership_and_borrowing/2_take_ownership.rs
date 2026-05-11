//! # Taking ownership by value
//!
//! When a function parameter has an owned type like `String` (no `&` in
//! front), calling the function *moves* the argument in. The caller's
//! binding is no longer usable afterwards — the value lives at the
//! callee now, and will be dropped when the callee finishes (unless it
//! hands ownership back via the return value, which is exactly what
//! happens here).
//!
//! Note the signature: `String` in, `String` out. Implement the body by
//! mutating the parameter (`s.push_str(...)`) and then returning `s`.
//! Because you own `s`, you're free to mutate it without any `&mut`
//! dance — ownership implies the right to modify.

/// Takes ownership of a String and modifies it.
/// When you pass a String to this function, ownership transfers.
fn take_ownership(s: String) -> String {
    // Add " - owned by Rust!" to the end and return
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_ownership() {
        let s = String::from("Rust");
        let result = take_ownership(s);
        // Note: s is no longer valid here! It was moved.
        assert_eq!(result, "Rust - owned by Rust!");
    }
}
