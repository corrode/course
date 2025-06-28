use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ApiResponse {
    status: String,
    data: Option<Vec<String>>,
    error: Option<String>,
    count: Option<u32>,
}

fn parse_api_response(json: &str) -> Result<ApiResponse, serde_json::Error> {
    todo!()
}

fn is_success(response: &ApiResponse) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_response() {
        let success_json = r#"{"status":"ok","data":["item1","item2"],"count":2}"#;
        let response = parse_api_response(success_json).unwrap();
        assert!(is_success(&response));
        assert_eq!(response.count, Some(2));
        
        let error_json = r#"{"status":"error","error":"Not found"}"#;
        let response = parse_api_response(error_json).unwrap();
        assert!(!is_success(&response));
    }
}