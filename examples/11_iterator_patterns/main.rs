//! # Data Processing with Iterators
//!
//! Iterators were popularised by functional languages like Lisp (created by
//! John McCarthy in 1958), and today they're a core building block in most
//! modern languages.
//!
//! Rust's iterators are lazy: they don't do any work until you ask for a
//! result. You can chain operations together like a pipeline, and the
//! compiler will usually fuse them into a single tight loop.

/// Returns all users whose usernames start with 'a'.
/// See: <https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter>
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter>
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect>
fn select_usernames_starting_with_a(usernames: Vec<&str>) -> Vec<&str> {
    todo!()
}

/// Normalizes email addresses to lowercase.
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map>
/// See: <https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase>
fn normalize_emails(emails: Vec<String>) -> Vec<String> {
    todo!()
}

/// Calculates total revenue from sales data.
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.iter>
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum>
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold>
fn calculate_total_revenue() -> i32 {
    let sales = vec![1200, 850, 2300, 950, 1800, 3200, 1100, 2800];
    todo!()
}

/// Finds all files with ".rs" extension.
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned>
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect>
fn find_rust_files<'file>(files: &[&'file str]) -> Vec<&'file str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_users() {
        let users = vec!["alice", "admin", "bob", "anonymous", "charlie"];
        let active = select_usernames_starting_with_a(users);
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
        let files = &[
            "main.rs",
            "README.md",
            "lib.rs",
            "package.json",
            "config.rs",
        ];
        let rust_files = find_rust_files(files);
        assert_eq!(rust_files, vec!["main.rs", "lib.rs", "config.rs"]);
    }
}
