# Keeping Elements with `filter`

With `map` you transformed every item, while `filter` keeps some items and drops
the rest. There is one borrowing detail to watch: `usernames.into_iter()` yields
`&str`, and `filter` gives its closure a reference to each item, so the closure
sees `&&str`.

Method calls such as `s.starts_with(...)` automatically dereference these
layers. The extra references can be hard to track at first. If the compiler
reports a missing `&`, check what the iterator yields and what the closure
receives. The [iterators entry in the cheatsheet](/cheatsheet) shows those
reference layers side by side.

## Useful from the Standard Library

- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
  keeps only items where the predicate returns `true`. The closure receives a
  reference to the item, regardless of whether the iterator yields owned values
  or borrows.
- [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
  takes a `char` (or another `&str`) and answers yes/no. Method-call syntax
  auto-derefs through the extra reference.
- `collect()` here picks `Vec<&str>` straight from the return type. You don't
  need a turbofish here.
