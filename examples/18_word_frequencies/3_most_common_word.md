# The Most Common Word

Return the word with the largest count together with that count. Empty or
whitespace-only input should return `None`.

A working `count_words` is included so this step runs on its own. Reuse it and
implement only `most_common_word`.

## Useful from the Standard Library

- [`HashMap::into_iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.into_iter)
  consumes the map and yields owned `(K, V)` pairs. In contrast, `iter` yields
  references to keys and values.
- [`Iterator::max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key)
  returns the item with the largest derived key as an `Option`.
