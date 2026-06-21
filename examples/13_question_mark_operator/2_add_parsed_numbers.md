# `?` for the simple case

`?` is Rust's shortcut for "if this is `Err`, return it from the current function; otherwise, unwrap the value and continue."
It compresses a lot of `match` boilerplate into one character.

Here both calls to `parse()` return the *same* error type ([`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)), so `?` works directly without any conversion.
Compare to writing this out with a `match` on each `parse()` result.
That's the boilerplate `?` is replacing.

## Useful from the standard library

- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) returns `Result<T, T::Err>`.
  Combined with `?` you get the parsed number on the happy path and an early-return on failure.
- [`std::num::ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html) is the error type for integer parses.
  The function signature declares it directly, so `?` doesn't need to convert anything.
- The function returns one expression: `Ok(a.parse::<i32>()? + b.parse::<i32>()?)`.
  Each `?` unwraps an integer, then `+` adds them, then `Ok(...)` wraps the sum back up.
