# Warm-up: `is_strong`

A `PasswordReport` holds the result of your checks: a numeric score, feedback, and a rough strength label.

Start with a small helper.
Before tackling the actual scoring, get a feel for the data by implementing the one-line `is_strong` method on `PasswordReport`.
By convention in this exercise, "strong" means the score is at least `70`.

You'll meet the shared `PasswordStrength` enum and `PasswordReport` struct here.
Each later page re-declares them so it can run on its own.

## Useful from the standard library

- The body is one comparison expression: `self.score >= 70`.
  No semicolon needed since the expression is the function's return.
- `#[derive(Debug, Clone)]` on the struct is what makes the test helper compile.
  You don't need to add anything else.
