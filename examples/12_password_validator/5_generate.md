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

## Useful from the standard library

- [`std::time::SystemTime::now`](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now)
  and [`Duration::subsec_nanos`](https://doc.rust-lang.org/std/time/struct.Duration.html#method.subsec_nanos)
  give you a quick pseudo-random `u32` for the cycling index.
- [`String::with_capacity`](https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity)
  pre-allocates the buffer if you know the final length up front.
- [`String::push`](https://doc.rust-lang.org/std/string/struct.String.html#method.push)
  appends a `char` one at a time. Combine with a `for _ in 0..length`
  loop and an index that walks the alphabets.
- For "pick the i-th character of an alphabet", a `&str` plus
  `.chars().nth(i)` works, or index a `&[u8]` and cast back to
  `char`.
