# Wrapping up the `?` operator

You replaced repetitive `match` chains with `?`, propagated errors out of multi-step functions, and used `?` inside a loop.

## What we learned

- `?` is shorthand for "if this is `Err`, return it from the current function; if it's `Ok`, unwrap the value and keep going."
  It works on `Option` too (returning `None` early).
- For `Result`, the surrounding function must return a `Result` with the same error type, or one the error converts into via `From`.
  For `Option`, the surrounding function returns `Option`; there is no error value to convert.
- Inside a loop, a failed `?` returns from the whole function, not just the current iteration.
  Later tokens are never parsed.
- Every exercise here used a single error type, so `?` propagated with no conversion.
  When a function mixes error types (say file I/O and parsing), you need a common error type.
  The env-file parser uses `Box<dyn Error>` as a common error type.
- Tests that share a filesystem path can race when they run in parallel.
  Use separate paths; `--test-threads=1` only serializes tests within one test process.
