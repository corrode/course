//! # Data Processing with Iterators
//!
//! Iterators transform collections efficiently. This pattern is essential
//! for processing user data, API responses, and database results.

/// Filters active users from a user list.
/// Use .into_iter().filter().collect() to find users starting with 'A'
fn filter_active_users(usernames: Vec<&str>) -> Vec<&str> {
    unimplemented!()
}

/// Normalizes email addresses to lowercase.
/// Use .into_iter().map().collect() with .to_lowercase()
fn normalize_emails(emails: Vec<String>) -> Vec<String> {
    unimplemented!()
}

/// Calculates total revenue from sales data.
/// Use .iter().sum() to add up all values.
fn calculate_total_revenue() -> i32 {
    let sales = vec![1200, 850, 2300, 950, 1800, 3200, 1100, 2800];
    unimplemented!()
}

/// Finds all files with ".rs" extension.
/// Use .iter().filter().cloned().collect() pattern.
fn find_rust_files(files: &[&str]) -> Vec<&str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_active_users() {
        let users = vec!["alice", "admin", "bob", "anonymous", "charlie"];
        let active = filter_active_users(users);
        assert_eq!(active, vec!["alice", "admin", "anonymous"]);
    }
    
    #[test]
    fn test_email_normalization() {
        let emails = vec!["Alice@EXAMPLE.COM".to_string(), "BOB@test.ORG".to_string()];
        let normalized = normalize_emails(emails);
        assert_eq!(normalized, vec!["alice@example.com", "bob@test.org"]);
    }
    
    #[test] 
    fn test_revenue_calculation() {
        let total = calculate_total_revenue();
        assert_eq!(total, 14200); // Sum of all sales
    }
    
    #[test]
    fn test_rust_files() {
        let files = &["main.rs", "README.md", "lib.rs", "package.json", "config.rs"];
        let rust_files = find_rust_files(files);
        assert_eq!(rust_files, vec!["main.rs", "lib.rs", "config.rs"]);
    }
}