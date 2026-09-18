# Wrapping up the env-file parser

You can now parse a configuration file, read its values as the types you need, and check for missing required keys.
The file parser builds on the line parser, so you only had to write the rules for `KEY=value` once.

## What we learned

- `split_once(delim)` is the right tool for "key/value, split at the *first* separator".
  It returns `Option<(&str, &str)>` without allocating.
- Walking a config file combines four small operations: `lines()`, `trim()`, `starts_with('#')`, and `is_empty()`.
  A `for` loop with `continue` and `?` reads better than an iterator chain when the body has both kinds of control flow.
- Keeping a small custom error enum such as `ParseError` next to the parser gives the parsing code one concrete error type.
  You can propagate it with `?` as long as the function returns `Result<_, ParseError>`.
- When a function mixes error types (reading the file gives `io::Error`, parsing gives `ParseError`), `?` needs one common type.
  Owned errors implementing `std::error::Error + 'static` convert into `Box<dyn std::error::Error>`.
  A custom enum lets callers match on specific cases; [`thiserror`](https://docs.rs/thiserror) can generate its `Display` and `Error` implementations.
- Generics let one function serve many caller types.
  The `where T: FromStr` bound is what makes `.parse()` work, and the caller pins `T` with a type annotation or a turbofish.
- Use `result.ok()` to drop an error and produce `Option<T>` when you don't need to know why parsing failed.
- Raw string literals (`r#"..."#`) embed multi-line text without escaping.
  The number of `#`s on each side just has to be enough to avoid colliding with the body.
- For real apps, the [`dotenvy`](https://docs.rs/dotenvy) crate reads `.env` files into the process environment; the parser you just wrote is a stripped-down version of the same idea.
