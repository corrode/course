# Generating a secure password

Implement `generate_secure_password(length)` that returns a `String`
of the requested length, mixing uppercase letters, lowercase letters,
digits, and special characters so it would pass a strict validator.

For variability without pulling in `rand`, you can use
`std::time::SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos()`
as a seed and cycle through your character sets. This is **not**
cryptographically secure, but it's plenty for this exercise. In real code,
use the `rand` crate.

Tips:
- Define one `&[u8]` (or `&str`) per character class.
- Make sure each class appears at least once so the result will pass
  strength checks regardless of `length` (assume `length >= 4`).
- The test below only checks the four character classes are present and
  the length is right; how you mix them is up to you.
