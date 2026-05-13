# Keeping elements with `filter`

Same idea as `map`, but instead of transforming each element you keep
some and drop others. Watch out for one borrowing gotcha: `.iter()`
yields `&T`, but `filter` gives its closure another reference on top,
so the closure sees `&&T`.

That's why you'll often see `**c == ...` or `s.starts_with(...)` (which
auto-derefs) instead of plain `c == ...`. Don't be alarmed when the
compiler complains about a missing `&`, see
[the cheatsheet](/cheatsheet) entry on iterators.

## Useful from the standard library

- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
  keeps only items where the predicate returns `true`. The closure
  receives a reference to the item, regardless of whether the
  iterator yields owned values or borrows.
- [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
  takes a `char` (or another `&str`) and answers yes/no. Method-call
  syntax auto-derefs through the extra reference.
- `collect()` here picks `Vec<&str>` straight from the return type.
  No turbofish needed.
