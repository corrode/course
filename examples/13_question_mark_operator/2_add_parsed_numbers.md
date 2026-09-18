# `?` for the Simple Case

`?` is Rust's shortcut for "if this is `Err`, return it from the current function; otherwise, unwrap the value and continue."

Here both calls to `parse()` return the *same* error type ([`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)), so `?` works directly without any conversion.
Compare to writing this out with a `match` on each `parse()` result.

## Useful from the Standard Library

- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) returns `Result<T, T::Err>`.
  Combined with `?` you get the parsed number on the happy path and an early-return on failure.
- [`std::num::ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html) is the error type for integer parses.
  The function signature declares it directly, so `?` doesn't need to convert anything.
