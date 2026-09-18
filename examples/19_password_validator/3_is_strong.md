# Warm-up: `is_strong`

A `PasswordReport` holds the result of your checks: a numeric score, feedback, and a rough strength label.

Implement the `is_strong` method on `PasswordReport`.
By convention in this exercise, "strong" means the score is at least `70`.

You'll meet the shared `PasswordStrength` enum and `PasswordReport` struct here.
The validation page re-declares them so it can run on its own.

The body is one comparison expression: `self.score >= 70`.
No semicolon needed since the expression is the function's return.
