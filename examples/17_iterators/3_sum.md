# Summing with an iterator

A running total is a good first place to see what an iterator consumer does.
Rust's iterators are lazy, so they don't do any work until you ask for a result.

You could add the values with a `for` loop and an accumulator.
Here, `sum` asks the iterator for each value and collapses the sequence into one total.

## Useful from the standard library

- [`<[T]>::iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter) produces an iterator of shared references over the slice.
- [`Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) reduces a numeric iterator to a single total.
  The function's `i32` return type supplies its output type when you return `sales.iter().sum()` directly.
  In other contexts, you may need an annotation (`let total: i32 = ...`) or the turbofish (`.sum::<i32>()`).
