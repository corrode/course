//! # HTTP Status Handling
//!
//! Pattern matching with `match` handles different cases exhaustively.
//! This is essential for error handling and API responses.

#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok,
    NotFound,
    Unauthorized,
    InternalServerError,
    BadRequest,
}

/// Returns the HTTP status code number.
/// Essential for building web APIs and handling responses.
fn status_code(status: HttpStatus) -> u16 {
    // Match each status to its code:
    // Ok: 200, NotFound: 404, Unauthorized: 401
    // InternalServerError: 500, BadRequest: 400
    unimplemented!()
}

/// Determines if the request should be retried.
/// Only retry on server errors, not client errors.
fn should_retry(status: HttpStatus) -> bool {
    // Only InternalServerError should trigger a retry
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_status_codes() {
        assert_eq!(status_code(HttpStatus::Ok), 200);
        assert_eq!(status_code(HttpStatus::NotFound), 404);
        assert_eq!(status_code(HttpStatus::InternalServerError), 500);
    }
    
    #[test]
    fn test_retry_logic() {
        assert_eq!(should_retry(HttpStatus::InternalServerError), true);
        assert_eq!(should_retry(HttpStatus::NotFound), false);
        assert_eq!(should_retry(HttpStatus::BadRequest), false);
        assert_eq!(should_retry(HttpStatus::Ok), false);
    }
}