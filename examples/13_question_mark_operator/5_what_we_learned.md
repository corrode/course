# Wrapping up the `?` operator

You replaced repetitive `match` chains with `?`, propagated errors out of multi-step functions, and rode `?` straight through an iterator pipeline.

## What we learned

- `?` is shorthand for "if this is `Err`, return it from the current function; if it's `Ok`, unwrap the value and keep going."
  It works on `Option` too (returning `None` early).
- The function using `?` must return a `Result` (or `Option`) whose error type matches, or one that the failing error converts into via `From`.
- `?` composes nicely with iterator pipelines: a `.parse()` that returns `Result` slots straight in, and `sum::<Result<_, _>>()` short-circuits on the first error.
- Every exercise here used a single error type, so `?` propagated with no conversion.
  When a function genuinely mixes error types (say file I/O and parsing), you need a common error type.
  The env-file parser chapter picks that up with `Box<dyn Error>`.
- Tests that touch the filesystem can race when the harness runs in parallel.
  Use unique filenames or `cargo test -- --test-threads=1` if you see flaky failures.
