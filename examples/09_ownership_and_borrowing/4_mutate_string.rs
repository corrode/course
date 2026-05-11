//! # Mutable borrows
//!
//! Sometimes you want to modify a value in place without taking
//! ownership of it. That's a mutable borrow: `&mut T`. The caller
//! still owns the value, but the callee gets exclusive write access
//! for the duration of the call.
//!
//! Two things to notice in the signature:
//!
//! 1. The parameter is `&mut String`, not `&mut str`. We need the
//!    *owned* `String` because growing it (with `push_str`) may
//!    reallocate; a bare string slice has a fixed length.
//! 2. There's no return value. The mutation happens through the
//!    reference and is visible to the caller after the call returns.
//!
//! On the call site (see the test): the caller has to write
//! `&mut s` explicitly, and `s` itself has to have been declared
//! `let mut s = ...`. Mutability is opt-in at every layer.

/// Takes a mutable reference to modify the string in place.
/// The &mut allows us to change the string's contents.
fn mutate_string(s: &mut String) {
    // Append " - now with extra crab" to the string
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutate_string() {
        let mut s = String::from("Ferris");
        mutate_string(&mut s);
        assert_eq!(s, "Ferris - now with extra crab");
    }
}
