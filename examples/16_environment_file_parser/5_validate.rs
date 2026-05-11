//! # Validating required variables
//!
//! Most apps need *some* configuration to be present at startup —
//! a database URL, a port, an API key. This last helper takes a list of
//! required keys and reports the first one that's missing.
//!
//! `Iterator::find` is a good fit: scan the slice, return the first
//! key that isn't in the map, and turn that into an `Err`. If `find`
//! returns `None`, every required key was present and the result is
//! `Ok(())`.

use std::collections::HashMap;

/// Validates required environment variables are present.
/// Returns Ok(()) if all required keys exist, Err with missing key otherwise.
fn validate_required_vars(env: &HashMap<String, String>, required: &[&str]) -> Result<(), String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_required_vars() {
        let mut env = HashMap::new();
        env.insert("HOST".to_string(), "localhost".to_string());
        env.insert("PORT".to_string(), "8080".to_string());

        assert!(validate_required_vars(&env, &["HOST", "PORT"]).is_ok());
        assert!(validate_required_vars(&env, &["HOST", "MISSING"]).is_err());
    }
}
