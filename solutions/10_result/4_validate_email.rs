/// Validates an email address (basic check).
/// Returns Ok(email) if contains '@', Err(message) otherwise.
///
/// Now the `Ok` value is a borrow of the input. The `&str` in the return
/// type implicitly borrows from `email`, so the compiler infers a lifetime
/// linking input and output via lifetime elision. Chapter 12 makes this
/// explicit; for now, just notice the function compiles even though no
/// lifetimes appear in the signature.
fn validate_email(email: &str) -> Result<&str, &'static str> {
    if email.contains('@') {
        Ok(email)
    } else {
        Err("email must contain '@'")
    }
}

#[test]
fn test_validate_email() {
    assert_eq!(validate_email("user@example.com"), Ok("user@example.com"));
    assert!(validate_email("invalid-email").is_err());
}
