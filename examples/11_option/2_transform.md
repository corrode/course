# Transforming the inside

You need the string's length in bytes when the `Option` is `Some`, and `0` when it's `None`.
That means calling `.len()` on the inner string before returning the result.
A `match` makes both branches explicit, while `Option`'s combinator methods keep this common case shorter.

## Useful from the standard library

- [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) applies a function inside the `Some` and leaves `None` alone.
  So `maybe.map(|s| s.len())` produces an `Option<usize>`.
- [`Option::map_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) handles both cases in one call, with a default for `None` and a closure for `Some`.
  You can write it as `maybe.map_or(0, |s| s.len())`.
