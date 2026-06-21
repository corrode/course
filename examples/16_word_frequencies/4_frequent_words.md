# Filtering frequent words

A different type of result: instead of one winning word, return every word whose count meets some threshold.
The natural pipeline is `count_words(text).into_iter().filter(...).map(...).collect()`, and the `collect` infers `Vec<String>` from the return type.

As before, `count_words` is stubbed with `todo!()` so this step compiles standalone.
Your work is in `frequent_words`.

## Useful from the standard library

- [`HashMap::into_iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.into_iter) hands out owned `(String, usize)` pairs, so the resulting `Vec` doesn't have to clone anything.
- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) keeps the pairs whose count is high enough.
  Destructure the tuple with `|(_, count)|` to ignore the word and look only at the count.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) drops the count and keeps just the word, so `collect` can build a `Vec<String>`.
- HashMap iteration order is unspecified; if your test ever relies on a particular order, sort the result first.
