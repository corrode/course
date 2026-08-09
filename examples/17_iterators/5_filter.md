# Keeping elements with `filter`

With `map` you transformed every item, while `filter` keeps some items and drops the rest.
There is one borrowing detail to watch: `.iter()` yields `&T`, but `filter` gives its closure another reference on top, so the closure sees `&&T`.

That's why you'll often see `**c == ...` or `s.starts_with(...)` (which auto-derefs) instead of plain `c == ...`.
If the compiler reports a missing `&`, the [iterators entry in the cheatsheet](/cheatsheet) shows the reference layers side by side.

## Useful from the standard library

- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) keeps only items where the predicate returns `true`.
  The closure receives a reference to the item, regardless of whether the iterator yields owned values or borrows.
- [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with) takes a `char` (or another `&str`) and answers yes/no.
  Method-call syntax auto-derefs through the extra reference.
- `collect()` here picks `Vec<&str>` straight from the return type.
  No turbofish needed.
