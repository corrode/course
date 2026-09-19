# The Most Common Word

Once you have the counts, you can ask the iterator for the largest one. The
choice between `iter` and `into_iter` determines whether you can return the
winning word without cloning it.

A working `count_words` is included so this step runs on its own. Reuse it and
implement only `most_common_word`.

## Useful from the Standard Library

- [`HashMap::into_iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.into_iter)
  consumes the map and yields owned `(K, V)` pairs. That's how you get an owned
  `String` out without cloning.
- [`Iterator::max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key)
  returns the entry with the largest derived key as an `Option`. Use the count
  half of each `(word, count)` pair as that key.
- An empty input naturally produces `None`: `count_words` returns an empty map,
  `into_iter().max_by_key(...)` returns `None`, and the function signature
  already says `Option<(String, usize)>`. You don't need an extra branch for
  that case.
