use std::collections::HashMap;

/// Creates a configuration map with default settings.
/// Returns a `HashMap` with "host" -> "localhost" and "port" -> "8080".
fn create_default_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert("host".to_string(), "localhost".to_string());
    config.insert("port".to_string(), "8080".to_string());
    config
}

#[test]
fn test_create_default_config() {
    let config = create_default_config();
    assert_eq!(config.get("host"), Some(&"localhost".to_string()));
    assert_eq!(config.get("port"), Some(&"8080".to_string()));
}
