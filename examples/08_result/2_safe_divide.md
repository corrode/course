# Safe divide

The simplest way to produce a `Result`: an `if` checks the failure
case, and the `else` branch returns `Ok(...)`.

The signature is the interesting part: `&'static str` for the error
is the simplest possible error type and is fine while you're learning.
