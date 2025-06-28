#[derive(Debug, PartialEq)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_log() {
        let line = "[2024-01-15 10:30:45] INFO: User logged in";
        let entry = parse_log_line(line).unwrap();
        assert_eq!(entry.timestamp, "2024-01-15 10:30:45");
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "User logged in");
    }
}