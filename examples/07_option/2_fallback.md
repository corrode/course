# Option with a fallback

The simplest `Option` pattern: you're handed one, and you either use
what's inside or fall back to a default. `match` works, and `Option`
also has a few helper methods that are shorter.

## Useful from the standard library

- [`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)
  returns the inner value when `Some`, the argument when `None`. The
  reach-for-this-first method.
- [`Option::unwrap_or_default`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default)
  is the same idea but uses `T::default()` (which for `u32` is `0`).
- A `match` always works too:
  `match setting { Some(v) => v, None => default }`. Pick whichever
  reads better.
