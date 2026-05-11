//! # Matching one variant
//!
//! Sometimes you only care about a single variant. You can still write a
//! full `match` with a `_` catch-all arm, or you can reach for the
//! [`matches!`](https://doc.rust-lang.org/std/macro.matches.html) macro
//! — both are idiomatic.
//!
//! Here, only a server-side failure (`InternalServerError`) is worth
//! retrying. Client errors like `NotFound` or `BadRequest` mean the
//! request itself is wrong, so retrying won't help.

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

/// Returns `true` if the request should be retried.
///
/// Only retry on server errors, not client errors.
fn should_retry(status: HttpStatus) -> bool {
    // Only InternalServerError should trigger a retry
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_retry() {
        assert_eq!(should_retry(HttpStatus::InternalServerError), true);
        assert_eq!(should_retry(HttpStatus::NotFound), false);
        assert_eq!(should_retry(HttpStatus::BadRequest), false);
        assert_eq!(should_retry(HttpStatus::Ok), false);
    }
}
