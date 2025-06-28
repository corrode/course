fn extract_emails(text: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_emails() {
        let text = "Contact us at info@company.com or support@example.org for help";
        let emails = extract_emails(text);
        assert_eq!(emails, vec!["info@company.com", "support@example.org"]);
    }
}