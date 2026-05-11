//! # Borrow in, own out
//!
//! This step is the canonical "borrowed in, owned out" shape. The
//! caller hands you a cheap `&str` view, and you give back a brand new
//! `String` that they get to keep. You'll see this pattern over and
//! over in real Rust code, so it's worth getting comfortable with the
//! signature now.

/// Takes a borrowed `&str` and returns an owned, uppercased `String`.
///
/// Notice the signature: borrow on the way in, own on the way out. That's
/// the pattern the table in the chapter intro is hinting at, and you'll
/// see it everywhere in real Rust code.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase>
fn shout(text: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shout() {
        assert_eq!(shout("hello"), "HELLO");
        assert_eq!(shout("Rust"), "RUST");
        assert_eq!(shout(""), "");
    }
}
