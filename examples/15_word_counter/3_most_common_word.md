# The most common word

Now that you can count, finding the maximum is a one-liner, almost.
The borrow checker has an opinion about returning data out of a
`HashMap`, and that's the real lesson of this step.

`count_words` is duplicated below as a `todo!()` stub so this step
compiles in isolation; you don't need to fill it in again. Focus on
`most_common_word`. Once you have it, the test will drive both
through `unwrap()`.

## Useful from the standard library

- [`HashMap::into_iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.into_iter)
  consumes the map and yields owned `(K, V)` pairs. That's how you
  get an owned `String` out without cloning.
- [`Iterator::max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key)
  returns the entry with the largest derived key as an `Option`.
  `max_by_key(|(_, count)| *count)` does the trick here.
- An empty input naturally produces `None`: `count_words` returns an
  empty map, `into_iter().max_by_key(...)` returns `None`, and the
  function signature already says `Option<(String, usize)>`. No
  special case needed.
