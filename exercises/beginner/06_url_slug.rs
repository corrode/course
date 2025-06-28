fn to_slug(title: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_slug() {
        assert_eq!(to_slug("Hello World!"), "hello-world");
        assert_eq!(to_slug("Rust is AWESOME!!!"), "rust-is-awesome");
        assert_eq!(to_slug("Multiple   spaces"), "multiple-spaces");
    }
}