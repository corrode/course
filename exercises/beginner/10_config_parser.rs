#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
}

#[derive(Debug)]
enum ConfigError {
    InvalidPort,
    InvalidTimeout,
    MissingField(&'static str),
}

fn parse_config(input: &str) -> Result<Config, ConfigError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_config() {
        let config = parse_config("host=localhost,port=8080,timeout=30").unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.timeout, 30);
        
        assert!(parse_config("host=localhost,port=99999,timeout=30").is_err());
    }
}