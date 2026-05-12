use std::collections::HashMap;

/// Gets a configuration value by key.
/// Returns the value if found, "default" if not found.
///
/// Looking up a key returns an `Option<&V>`, because the key might not
/// be there. From chapter 7 you've already seen a few ways to collapse
/// an `Option` into a concrete value.
fn get_config_value(config: &HashMap<String, String>, key: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_value() {
        let mut config = HashMap::new();
        config.insert("timeout".to_string(), "30".to_string());

        assert_eq!(get_config_value(&config, "timeout"), "30");
        assert_eq!(get_config_value(&config, "missing"), "default");
    }
}
