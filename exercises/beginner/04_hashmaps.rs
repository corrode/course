//! # HashMap Basics
//!
//! HashMaps store key-value pairs for fast lookups.
//! Essential for caching, configuration, and associative data.

use std::collections::HashMap;

/// Creates a configuration map with default settings.
/// Returns a HashMap with "host" -> "localhost" and "port" -> "8080".
fn create_default_config() -> HashMap<String, String> {
    unimplemented!()
}

/// Gets a configuration value by key.
/// Returns the value if found, "default" if not found.
fn get_config_value(config: &HashMap<String, String>, key: &str) -> String {
    // Use .get() method and .unwrap_or() for default
    unimplemented!()
}

/// Updates a configuration value.
/// Inserts or updates the key-value pair.
fn set_config_value(config: &mut HashMap<String, String>, key: &str, value: &str) {
    // Use .insert() method
    unimplemented!()
}

/// Counts how many times each word appears.
/// Returns a HashMap mapping words to their counts.
fn count_words(words: &[&str]) -> HashMap<String, usize> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = create_default_config();
        assert_eq!(config.get("host"), Some(&"localhost".to_string()));
        assert_eq!(config.get("port"), Some(&"8080".to_string()));
    }
    
    #[test]
    fn test_get_config() {
        let mut config = HashMap::new();
        config.insert("timeout".to_string(), "30".to_string());
        
        assert_eq!(get_config_value(&config, "timeout"), "30");
        assert_eq!(get_config_value(&config, "missing"), "default");
    }
    
    #[test]
    fn test_set_config() {
        let mut config = HashMap::new();
        set_config_value(&mut config, "debug", "true");
        assert_eq!(config.get("debug"), Some(&"true".to_string()));
    }
    
    #[test]
    fn test_word_counting() {
        let words = ["hello", "world", "hello", "rust"];
        let counts = count_words(&words);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("rust"), Some(&1));
    }
}