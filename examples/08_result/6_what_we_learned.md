# Wrapping up `Result`

You produced `Result`s with simple `if` checks, returned an owned
`String` in the `Ok` arm, borrowed from the input via lifetime
elision, and combined `strip_suffix`, `parse`, and a bounds check
into a real validating parser.

## What we learned

- `Result<T, E>` is how Rust expresses fallibility. There are no
  exceptions; a function that can fail says so in its signature.
- `Ok(value)` and `Err(error)` are the constructors. Both are in
  the prelude.
- `Result` has the same combinator family as `Option`: `unwrap_or`,
  `map`, `map_or`, plus `map_err` for transforming the error side
  and `ok` to drop the error and convert to `Option<T>`.
- `&'static str` is the cheapest error type: a borrowed string
  literal that lives forever. Real applications usually graduate to
  enums or `String`-based errors, but this is a fine starting point.
- The turbofish (`parse::<u8>()`) spells out a generic type argument
  at the call site when the type isn't clear from context.
- Match guards (`Ok(n) if n > 0 => ...`) attach a boolean condition
  to a pattern. The arm only fires when both hold.
- The `?` operator (chapter 13) will let you chain fallible calls
  without writing `match` every time. For now, `match` is fine.
