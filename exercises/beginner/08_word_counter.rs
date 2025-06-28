use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_word_count() {
        let text = "hello world hello rust world";
        let counts = word_count(text);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&2));
        assert_eq!(counts.get("rust"), Some(&1));
    }
}