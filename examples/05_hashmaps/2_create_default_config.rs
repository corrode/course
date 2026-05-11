//! # Creating a config map
//!
//! The simplest way to use a `HashMap` is to create one and insert a few
//! key-value pairs. `HashMap::new()` gives you an empty map; the type is
//! usually inferred from the first `insert`.
//!
//! Here you'll build a small configuration map with two defaults: a
//! `"host"` and a `"port"`.

use std::collections::HashMap;

/// Creates a configuration map with default settings.
/// Returns a HashMap with "host" -> "localhost" and "port" -> "8080".
fn create_default_config() -> HashMap<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_config() {
        let config = create_default_config();
        assert_eq!(config.get("host"), Some(&"localhost".to_string()));
        assert_eq!(config.get("port"), Some(&"8080".to_string()));
    }
}
