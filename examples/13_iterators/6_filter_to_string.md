# Filter, then own the result

Same shape as the previous step, but the input is a `&[&str]` (a
borrowed slice of borrowed strings), so the iterator yields `&&str`.
We sidestep that double-reference by returning owned `String`s; the
lesson here is iterators, not lifetimes.

To go from `&&str` to `String`, reach for [`str::to_string`]. Chain
it after your `filter` with a `map`, then `collect` into a `Vec`.

## Useful from the standard library

- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
  again. Same closure shape; auto-deref still saves you for
  `.ends_with(".rs")`.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
  is what converts the surviving `&&str`s into owned `String`s.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  is the easy `&str` -> `String` call. Auto-deref reaches through
  the extra reference for you.
- [`str::ends_with`](https://doc.rust-lang.org/std/primitive.str.html#method.ends_with)
  is the suffix check used by the predicate.
