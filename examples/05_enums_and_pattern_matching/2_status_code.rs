// `Copy` lets you pass the same `HttpStatus` value to multiple
// functions without it being moved on the first call. Plain enums
// like this one (no `String`, no `Vec`, no other heap data) are
// always cheap to copy, so deriving `Copy` (and `Clone`) costs you
// nothing and removes a borrow-checker speed bump.
#[derive(Debug, PartialEq, Clone, Copy)]
enum HttpStatus {
    Ok,
    NotFound,
    Unauthorized,
    InternalServerError,
    BadRequest,
}

/// Returns the HTTP status code number for the given status.
fn status_code(status: HttpStatus) -> u16 {
    // Match each status to its code:
    // Ok: 200, NotFound: 404, Unauthorized: 401
    // InternalServerError: 500, BadRequest: 400
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code() {
        assert_eq!(status_code(HttpStatus::Ok), 200);
        assert_eq!(status_code(HttpStatus::NotFound), 404);
        assert_eq!(status_code(HttpStatus::InternalServerError), 500);
    }
}
