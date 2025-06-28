fn calculate(expression: &str) -> Result<f64, &'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate() {
        assert_eq!(calculate("2 + 3"), Ok(5.0));
        assert_eq!(calculate("2 + 3 * 4"), Ok(14.0));
        assert_eq!(calculate("(2 + 3) * 4"), Ok(20.0));
        assert!(calculate("2 / 0").is_err());
    }
}