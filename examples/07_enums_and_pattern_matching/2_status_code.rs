// `Copy` lets you pass the same `HttpStatus` value to multiple functions
// without moving it on the first call. An enum like this one has no payload
// (`String`, `Vec`, or other heap data), so it is cheap to copy. Deriving
// `Copy` (and `Clone`) lets you reuse the value without borrowing it.
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
    // Match each status to its code: Ok: 200, NotFound: 404, Unauthorized: 401
    // InternalServerError: 500, BadRequest: 400
    todo!()
}

#[test]
fn test_status_code() {
    assert_eq!(status_code(HttpStatus::Ok), 200);
    assert_eq!(status_code(HttpStatus::NotFound), 404);
    assert_eq!(status_code(HttpStatus::Unauthorized), 401);
    assert_eq!(status_code(HttpStatus::BadRequest), 400);
    assert_eq!(status_code(HttpStatus::InternalServerError), 500);
}
