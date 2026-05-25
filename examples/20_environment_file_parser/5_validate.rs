use std::collections::HashMap;

/// Validates required environment variables are present.
/// Returns Ok(()) if all required keys exist, Err with missing key otherwise.
fn validate_required_vars(env: &HashMap<String, String>, required: &[&str]) -> Result<(), String> {
    todo!()
}

#[test]
fn test_validate_required_vars() {
    let mut env = HashMap::new();
    env.insert("HOST".to_string(), "localhost".to_string());
    env.insert("PORT".to_string(), "8080".to_string());

    assert!(validate_required_vars(&env, &["HOST", "PORT"]).is_ok());
    assert!(validate_required_vars(&env, &["HOST", "MISSING"]).is_err());
}
