fn parse_csv(data: &str) -> Vec<Vec<String>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_csv() {
        let data = "name,age\nJohn,25\nJane,30";
        let rows = parse_csv(data);
        assert_eq!(rows[0], vec!["name", "age"]);
        assert_eq!(rows[1], vec!["John", "25"]);
        assert_eq!(rows[2], vec!["Jane", "30"]);
    }
}