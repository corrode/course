# Wrapping up `Result`

You started with `Result`s built from simple `if` checks, then returned both owned and borrowed success values.
Finally, you combined `strip_suffix`, `parse`, and a bounds check into a validating parser with several failure cases.

## What we learned

- `Result<T, E>` is how Rust expresses fallibility.
  There are no exceptions; a function that can fail says so in its signature.
- `Ok(value)` and `Err(error)` are the constructors.
  Both are in the prelude.
- `Result` has the same combinator family as `Option`: `unwrap_or`, `map`, `map_or`, plus `map_err` for transforming the error side and `ok` to drop the error and convert to `Option<T>`.
- `&'static str` is a convenient error type when every message is a fixed string literal.
  Applications often use error enums or owned `String`s once errors need data of their own.
- The turbofish (`parse::<u8>()`) spells out a generic type argument at the call site when the type isn't clear from context.
- Match guards (`Ok(n) if n > 0 => ...`) attach a boolean condition to a pattern.
  The arm only fires when both hold.
- Next, we'll use the `?` operator to chain fallible calls without writing `match` every time.
  For now, `match` keeps both paths visible.
