/// Parses a simple CSV line without quotes.
/// Splits on commas and trims whitespace.
fn parse_simple_csv_line(line: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_csv_line() {
        let line = "name, age, city";
        let fields = parse_simple_csv_line(line);
        assert_eq!(fields, vec!["name", "age", "city"]);
    }
}
