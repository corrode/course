# Wrapping up the `?` operator

You replaced repetitive `match` chains with `?`, propagated errors out of multi-step functions, and used `?` after an iterator pipeline.

## What we learned

- `?` is shorthand for "if this is `Err`, return it from the current function; if it's `Ok`, unwrap the value and keep going."
  It works on `Option` too (returning `None` early).
- The function using `?` must return a `Result` (or `Option`) whose error type matches, or one that the failing error converts into via `From`.
- `?` can follow an iterator pipeline that produces a `Result`, so the first error still returns early.
- Every exercise here used a single error type, so `?` propagated with no conversion.
  When a function genuinely mixes error types (say file I/O and parsing), you need a common error type.
  The env-file parser uses `Box<dyn Error>` as a common error type.
- Tests that touch the filesystem can race when the harness runs in parallel.
  Use unique filenames or `cargo test -- --test-threads=1` if you see flaky failures.
