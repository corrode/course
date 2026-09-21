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

/// Returns `true` if the request should be retried.
///
/// Only retry on server errors, not client errors.
fn should_retry(status: HttpStatus) -> bool {
    todo!("Return true only for InternalServerError")
}

#[test]
fn test_should_retry() {
    assert_eq!(should_retry(HttpStatus::InternalServerError), true);
    assert_eq!(should_retry(HttpStatus::NotFound), false);
    assert_eq!(should_retry(HttpStatus::Unauthorized), false);
    assert_eq!(should_retry(HttpStatus::BadRequest), false);
    assert_eq!(should_retry(HttpStatus::Ok), false);
}
