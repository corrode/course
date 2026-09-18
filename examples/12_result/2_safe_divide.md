# Safe Divide

You can produce a `Result` with an `if` that checks the failure case and an `else` branch that returns `Ok(...)`.

The error type is `&'static str`, so you can return a fixed error message without defining a new type.

## Useful from the Standard Library

- The `Result` constructors `Ok(value)` and `Err(message)` are in the prelude, so you can use them without importing anything.
- `divisor == 0.0` detects both positive and negative zero, the failure cases for this exercise.
