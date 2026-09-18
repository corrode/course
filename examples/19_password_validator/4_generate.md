# Generating a password

Implement `generate_secure_password(length)` that returns a `String` of the requested length, containing at least one uppercase letter, one lowercase letter, one digit, and one special character from `!@#$%^&*`.

For variability without pulling in `rand`, you can use `std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()` as a seed and cycle through your character sets.
Despite the function's name, this clock-based approach is **not** cryptographically secure.
Use it only for this exercise.
In real code, use the `rand` crate.

Tips:
- Define one `&[u8]` (or `&str`) per character class.
- Make sure each class appears at least once (assume `length >= 4`).
- The test below only checks the four character classes are present and the length is right; how you mix them is up to you.

## Useful from the standard library

- [`SystemTime::duration_since`](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.duration_since) returns a `Result<Duration, SystemTimeError>`.
  After handling that result, [`Duration::subsec_nanos`](https://doc.rust-lang.org/std/time/struct.Duration.html#method.subsec_nanos) gives the fractional second in nanoseconds as a `u32`; cast it to `usize` for an index.
- [`String::with_capacity`](https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity) reserves capacity in bytes, which matches the character count for these ASCII alphabets.
- [`String::push`](https://doc.rust-lang.org/std/string/struct.String.html#method.push) appends a `char` one at a time.
  Combine with a `for _ in 0..length` loop and an index that walks the alphabets.
- For "pick the i-th character of an alphabet", a `&str` plus `.chars().nth(i)` works, or index a `&[u8]` and cast back to `char`.
