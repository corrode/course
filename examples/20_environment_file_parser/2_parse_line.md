# Parsing a single line

Before tackling whole files, get one line right.
The `.env` format is `KEY=value`, but real-world files have surrounding whitespace too.
`KEY = value` should be accepted and produce `("KEY", "value")`.

`str::split_once('=')` is the right tool: it gives back `Option<(&str, &str)>` containing the part before and after the first `=`.
From there it's `trim` plus a couple of validity checks.

We also introduce a small `ParseError` enum that the rest of the chapter will reuse.

## Useful from the standard library

- [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) splits at the first match and returns `Option<(&str, &str)>`.
  The `None` case maps cleanly onto `ParseError::InvalidFormat`.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim) on each half drops the surrounding whitespace.
- [`str::is_empty`](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty) catches the `=value` and `KEY=` cases after trimming.
- The `Ok` arm needs owned `String`s, so finish with `.to_string()` on each half before wrapping in the tuple.
