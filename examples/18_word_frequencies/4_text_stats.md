# Text statistics

Now you'll combine several small aggregations in one function.
`text_stats` returns three numbers about a piece of text: total word count, number of unique words, and the average word length as an `f64`.
You can compute all three from a single pass over `count_words`'s result, or split the work; either is fine.

Watch the order of operations when you compute the average.
Integer division truncates, so cast to `f64` before you divide.
Converting an already-truncated result won't bring the fraction back.
The test compares the result against a small tolerance because calculations with `f64` can introduce rounding error.

`count_words` is stubbed with `todo!()` again so this file compiles on its own.
Paste your earlier implementation into it before calling it from `text_stats`.
You can organize the body of `text_stats` as you like.
The test only cares about the returned tuple.

## Useful from the standard library

- [`HashMap::values`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.values) yields the occurrence counts; [`Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) adds them to give the total number of words.
- [`HashMap::len`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.len) gives the number of unique words because each key appears once.
- [`HashMap::iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter) yields each word and its count.
  Multiply `word.chars().count()` by its occurrence count, then sum those lengths for the average.
  `chars()` counts Unicode scalar values; `len()` would count bytes.
