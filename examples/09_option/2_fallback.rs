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
