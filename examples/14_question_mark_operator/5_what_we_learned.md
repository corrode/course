# Wrapping up the `?` operator

You replaced repetitive `match` chains with `?`, used it across
matching error types, then mixed two error sources in one function
via `Box<dyn Error>`.

## What we learned

- `?` is shorthand for "if this is `Err`, return it from the current
  function; if it's `Ok`, unwrap the value and keep going." It works
  on `Option` too (returning `None` early).
- The function using `?` must return a `Result` (or `Option`) whose
  error type matches, or one that the failing error converts into via
  `From`.
- When all calls produce the same error type, `?` just propagates.
  When they don't, you need a common error type. The two standard
  options are `Box<dyn std::error::Error>` (cheap, lossy on type
  info) and a custom enum (more work, more clarity).
- `?` chains compose nicely with iterator pipelines: a `.parse()`
  that returns `Result` slots straight in, and `sum::<Result<_, _>>()`
  short-circuits on the first error.
- For real applications the [`anyhow`](https://docs.rs/anyhow) and
  [`thiserror`](https://docs.rs/thiserror) crates are the usual
  upgrades over `Box<dyn Error>` and hand-rolled enums respectively.
- Tests that touch the filesystem can race when the harness runs in
  parallel. Use unique filenames or `cargo test -- --test-threads=1`
  if you see flaky failures.
