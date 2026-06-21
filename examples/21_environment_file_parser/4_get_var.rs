use std::collections::HashMap;

/// Gets an environment variable with type conversion.
/// Parses the string value into the requested type.
///
/// Hint: the natural solution is `env.get(key)?.parse().ok()`. Don't try to
/// `?` the parse: `T::Err` is unconstrained here, so `?` would need a
/// `From<T::Err>` bound that we haven't added. `.ok()` collapses
/// `Result<T, T::Err>` into `Option<T>`, which is what the signature
/// returns anyway.
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
