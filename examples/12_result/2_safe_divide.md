# Safe divide

The simplest way to produce a `Result`: an `if` checks the failure case, and the `else` branch returns `Ok(...)`.

The signature is the interesting part: `&'static str` for the error is the simplest possible error type and is fine while you're learning.

## Useful from the standard library

- The `Result` constructors `Ok(value)` and `Err(message)` are in the prelude, so you can use them without importing anything.
- `f64 == 0.0` is the bounds check.
  Floating-point comparison has plenty of nasty edge cases in general, but exact zero is fine.
- [`Result::is_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err)
  is what the test uses; you don't need it inside the function.
