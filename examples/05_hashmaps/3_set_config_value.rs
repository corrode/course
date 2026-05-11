use std::collections::HashMap;

/// Updates a configuration value.
/// Inserts or updates the key-value pair.
///
/// One method on `HashMap` covers both cases; it doesn't care whether
/// the key was already there.
fn set_config_value(config: &mut HashMap<String, String>, key: &str, value: &str) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_config_value() {
        let mut config = HashMap::new();
        set_config_value(&mut config, "debug", "true");
        assert_eq!(config.get("debug"), Some(&"true".to_string()));
    }
}
