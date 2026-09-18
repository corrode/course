# Filter, then own the result

This time the input is a `&[&str]`, a borrowed slice of borrowed strings, so the iterator yields `&&str`.
You'll return owned `String`s so the caller can keep the results independently of the input.
That lets us focus on the iterator chain without adding lifetime annotations.

[`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string) converts each surviving `&&str` into an owned `String` through auto-deref.
Chain it after your `filter` with a `map`, then `collect` into a `Vec`.

## Useful from the standard library

- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) passes `&&&str` to the predicate here: a reference to the iterator's `&&str` item.
  Method-call auto-deref lets you call `.ends_with(".rs")` directly.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) applies your conversion closure to each surviving `&&str`.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string) converts a borrowed string slice into an owned `String`.
  Auto-deref reaches through the extra reference for you.
- [`str::ends_with`](https://doc.rust-lang.org/std/primitive.str.html#method.ends_with) is the suffix check used by the predicate.
