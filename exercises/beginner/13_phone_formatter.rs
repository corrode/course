fn format_phone(phone: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_phone() {
        assert_eq!(format_phone("5551234567"), Some("(555) 123-4567".to_string()));
        assert_eq!(format_phone("555-123-4567"), Some("(555) 123-4567".to_string()));
        assert_eq!(format_phone("+1 555 123 4567"), Some("(555) 123-4567".to_string()));
        assert_eq!(format_phone("invalid"), None);
    }
}