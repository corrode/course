#[derive(Debug)]
struct PasswordStrength {
    score: u8,
    feedback: Vec<String>,
}

fn validate_password(password: &str) -> PasswordStrength {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_strength() {
        let weak = validate_password("123");
        assert!(weak.score < 30);
        assert!(!weak.feedback.is_empty());
        
        let strong = validate_password("MyStr0ng!P@ssw0rd");
        assert!(strong.score > 80);
    }
}