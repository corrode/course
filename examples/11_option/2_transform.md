# Transforming the Inside

You need the string's length in bytes when the `Option` is `Some`, and `0` when it's `None`.
That means calling `.len()` on the inner string before returning the result.
A `match` makes both branches explicit, while `Option`'s combinator methods keep this common case shorter.

## Useful from the Standard Library

- [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) applies a function inside the `Some` and leaves `None` alone.
- [`Option::map_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) handles both cases in one call, with a default for `None` and a closure for `Some`.
