//! # Typed lookup with generics
//!
//! Configuration values are stored as strings, but consumers want
//! `u16` ports, `bool` flags, and so on. Rather than write one helper
//! per type, declare a generic function bounded by `FromStr` and let the
//! caller pick the type at the call site with a turbofish or a type
//! annotation.
//!
//! Inside the body, `env.get(key)?` short-circuits on a missing key and
//! `.parse().ok()` collapses the parse `Result` into an `Option`. Don't
//! try to `?` the parse: `T::Err` is unconstrained here and would need
//! an extra `From` bound.

use std::collections::HashMap;

/// Gets an environment variable with type conversion.
/// Parses the string value into the requested type.
///
/// Hint: the natural shape is `env.get(key)?.parse().ok()`. Don't try to
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
