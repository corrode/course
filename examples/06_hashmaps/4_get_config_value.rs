use std::collections::HashMap;

/// Gets a configuration value by key.
/// Returns the value if found, "default" if not found.
///
/// Looking up a key returns an `Option<&V>`, because the key might not
/// be there. Collapse it into a concrete value with a fallback for
/// the missing case (`Option` is covered properly in chapter 9).
fn get_config_value(config: &HashMap<String, String>, key: &str) -> String {
    todo!()
}

#[test]
fn test_get_config_value() {
    let mut config = HashMap::new();
    config.insert("timeout".to_string(), "30".to_string());

    assert_eq!(get_config_value(&config, "timeout"), "30");
    assert_eq!(get_config_value(&config, "missing"), "default");
}
