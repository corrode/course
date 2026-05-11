//! # Option with a fallback
//!
//! The simplest `Option` pattern: you're handed one, and you either use
//! what's inside or fall back to a default. `match` works, and `Option`
//! also has a few helper methods that are shorter.
//!
//! See: <https://doc.rust-lang.org/std/option/enum.Option.html>

/// Returns the value if `Some`, otherwise returns the default.
fn get_setting_or_default(setting: Option<u32>, default: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_setting_or_default() {
        assert_eq!(get_setting_or_default(Some(42), 100), 42);
        assert_eq!(get_setting_or_default(None, 100), 100);
    }
}
