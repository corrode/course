use std::collections::HashMap;

/// Gets an environment variable with type conversion.
/// Parses the string value into the requested type.
fn get_env_var<T>(env: &HashMap<String, String>, key: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    todo!()
}

#[test]
fn test_get_env_var() {
    let mut env = HashMap::new();
    env.insert("PORT".to_string(), "8080".to_string());
    env.insert("DEBUG".to_string(), "true".to_string());

    let port: Option<u16> = get_env_var(&env, "PORT");
    assert_eq!(port, Some(8080));

    let debug: Option<bool> = get_env_var(&env, "DEBUG");
    assert_eq!(debug, Some(true));
}

#[test]
fn missing_key_returns_none() {
    let env = HashMap::new();
    assert_eq!(get_env_var::<u16>(&env, "PORT"), None);
    assert_eq!(get_env_var::<bool>(&env, "DEBUG"), None);
}

#[test]
fn present_but_invalid_values_return_none() {
    let mut env = HashMap::new();
    for value in ["eight", "65536", "-1", ""] {
        env.insert("PORT".into(), value.into());
        assert_eq!(get_env_var::<u16>(&env, "PORT"), None, "{value:?}");
    }
    env.insert("DEBUG".into(), "yes".into());
    assert_eq!(get_env_var::<bool>(&env, "DEBUG"), None);
}

#[test]
fn caller_type_decides_whether_conversion_succeeds() {
    let env = HashMap::from([("VALUE".into(), "256".into())]);
    assert_eq!(get_env_var::<u8>(&env, "VALUE"), None);
    assert_eq!(get_env_var::<u16>(&env, "VALUE"), Some(256));
    assert_eq!(get_env_var::<String>(&env, "VALUE"), Some("256".into()));
}
