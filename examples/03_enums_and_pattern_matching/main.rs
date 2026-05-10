//! # Enums and Pattern Matching
//!
//! HTTP status codes were introduced by Tim Berners-Lee when he created the
//! World Wide Web at CERN in 1989. These three-digit codes tell the client
//! what happened with a request.
//!
//! Fun fact: the famous "404 Not Found" is sometimes said to be named after
//! room 404 at CERN, though that story is more legend than fact.
//!
//! In this exercise you'll use Rust's `match` expression. Think of it as a
//! `switch` statement that the compiler forces you to handle exhaustively.

// `Copy` lets you pass the same `HttpStatus` value to multiple
// functions without it being moved on the first call. Plain enums
// like this one — no `String`, no `Vec`, no other heap data — are
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

/// Returns the HTTP status code number.
/// Essential for building web APIs and handling responses.
fn status_code(status: HttpStatus) -> u16 {
    // Match each status to its code:
    // Ok: 200, NotFound: 404, Unauthorized: 401
    // InternalServerError: 500, BadRequest: 400
    todo!()
}

#[test]
fn test_status_codes() {
    assert_eq!(status_code(HttpStatus::Ok), 200);
    assert_eq!(status_code(HttpStatus::NotFound), 404);
    assert_eq!(status_code(HttpStatus::InternalServerError), 500);
}

/// Determines if the request should be retried.
/// Only retry on server errors, not client errors.
fn should_retry(status: HttpStatus) -> bool {
    // Only InternalServerError should trigger a retry
    todo!()
}

#[test]
fn test_retry_logic() {
    assert_eq!(should_retry(HttpStatus::InternalServerError), true);
    assert_eq!(should_retry(HttpStatus::NotFound), false);
    assert_eq!(should_retry(HttpStatus::BadRequest), false);
    assert_eq!(should_retry(HttpStatus::Ok), false);
}
