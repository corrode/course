# Safe divide

You can produce a `Result` with an `if` that checks the failure case and an `else` branch that returns `Ok(...)`.

The error type is `&'static str`, so you can return a fixed error message without defining a new type.

## Useful from the standard library

- The `Result` constructors `Ok(value)` and `Err(message)` are in the prelude, so you can use them without importing anything.
- `f64 == 0.0` detects the failure case.
  Floating-point comparison has plenty of nasty edge cases in general, but checking for exact zero is fine here.
- [`Result::is_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err) is what the test uses; you don't need it inside the function.
