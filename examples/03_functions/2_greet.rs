/// Returns `"Hello, <name>!"`.
///
/// Practice the basic shape: a `&str` parameter, a `String` return,
/// one expression in the body. The exclamation mark is part of the
/// expected output.
fn greet(name: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Ferris"), "Hello, Ferris!");
        assert_eq!(greet("world"), "Hello, world!");
    }
}
