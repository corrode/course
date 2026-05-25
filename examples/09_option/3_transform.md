# Transforming the inside

Same idea as before, but now the fallback isn't the value itself. You
need to call `.len()` on the inner string first. A `match` makes both
branches explicit; iterator-style methods on `Option` are tidier once
you spot them.

## Useful from the standard library

- [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
  applies a function inside the `Some` and leaves `None` alone. So
  `maybe.map(|s| s.len())` produces an `Option<usize>`.
- [`Option::map_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or)
  collapses both steps into one call: a default for `None` and a
  closure for `Some`. Reads as
  `maybe.map_or(0, |s| s.len())`.
- The chapter intro explains the `|s| ...` closure syntax. For now
  read it as a tiny one-shot function from `s` to its body.
