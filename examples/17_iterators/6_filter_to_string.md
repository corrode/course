# Filter, Then Own the Result

This time the input is a `&[&str]`, a borrowed slice of borrowed strings, so the
iterator yields `&&str`. You'll return owned `String`s so the caller can keep
the results independently of the input. That lets us focus on the iterator chain
without adding lifetime annotations.

Combine what you've used so far to return only paths ending in `.rs`, in their
original order. An empty input or no matching paths should give an empty vector.
This time, choose how the adapters fit together yourself.

## Useful from the Standard Library

- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
  borrows each item for its predicate. When the iterator yields `&&str`, the
  predicate receives `&&&str`. String method calls auto-deref through these
  layers.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
  can change the item type by applying a closure to each item.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  converts a borrowed string slice into an owned `String`. Auto-deref reaches
  through the extra reference for you.
- [`str::ends_with`](https://doc.rust-lang.org/std/primitive.str.html#method.ends_with)
  checks whether a string ends with a given suffix.
