//! # Environment File Parser
//!
//! Parse .env files containing key=value pairs.
//! A practical exercise that combines string parsing, error handling, and collections.

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidFormat,
    EmptyKey,
    EmptyValue,
}

/// Parses a single line of an .env file.
/// Format: "KEY=value" (no spaces around =)
/// Returns Ok((key, value)) or Err(ParseError).
fn parse_env_line(line: &str) -> Result<(String, String), ParseError> {
    // 1. Check if line contains '='
    // 2. Split on '=' (use .split_once())
    // 3. Validate key and value are not empty
    // 4. Trim whitespace from both parts
    unimplemented!()
}

/// Parses a complete .env file content.
/// Ignores empty lines and lines starting with '#'.
/// Returns HashMap of all valid key-value pairs.
fn parse_env_file(content: &str) -> Result<HashMap<String, String>, ParseError> {
    unimplemented!()
}

/// Gets an environment variable with type conversion.
/// Parses the string value into the requested type.
fn get_env_var<T>(env: &HashMap<String, String>, key: &str) -> Option<T> 
where
    T: std::str::FromStr,
{
    // Use .get() then .parse() 
    unimplemented!()
}

/// Validates required environment variables are present.
/// Returns Ok(()) if all required keys exist, Err with missing key otherwise.
fn validate_required_vars(env: &HashMap<String, String>, required: &[&str]) -> Result<(), String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_env_line("PORT=8080"), Ok(("PORT".to_string(), "8080".to_string())));
        assert_eq!(parse_env_line("HOST=localhost"), Ok(("HOST".to_string(), "localhost".to_string())));
        assert!(parse_env_line("INVALID").is_err());
        assert!(parse_env_line("=value").is_err());
        assert!(parse_env_line("KEY=").is_err());
    }
    
    #[test]
    fn test_parse_file() {
        let content = r#"
# Database configuration
HOST=localhost
PORT=5432
DATABASE=myapp

# Empty line above should be ignored
DEBUG=true
"#;
        let env = parse_env_file(content).unwrap();
        assert_eq!(env.get("HOST"), Some(&"localhost".to_string()));
        assert_eq!(env.get("PORT"), Some(&"5432".to_string()));
        assert_eq!(env.get("DEBUG"), Some(&"true".to_string()));
        assert_eq!(env.len(), 4);
    }
    
    #[test]
    fn test_type_conversion() {
        let mut env = HashMap::new();
        env.insert("PORT".to_string(), "8080".to_string());
        env.insert("DEBUG".to_string(), "true".to_string());
        
        let port: Option<u16> = get_env_var(&env, "PORT");
        assert_eq!(port, Some(8080));
        
        let debug: Option<bool> = get_env_var(&env, "DEBUG");
        assert_eq!(debug, Some(true));
    }
    
    #[test]
    fn test_validation() {
        let mut env = HashMap::new();
        env.insert("HOST".to_string(), "localhost".to_string());
        env.insert("PORT".to_string(), "8080".to_string());
        
        assert!(validate_required_vars(&env, &["HOST", "PORT"]).is_ok());
        assert!(validate_required_vars(&env, &["HOST", "MISSING"]).is_err());
    }
}