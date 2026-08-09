# Text statistics

Now you'll combine several small aggregations in one function.
`text_stats` returns three numbers about a piece of text: total word count, number of unique words, and the average word length as an `f64`.
You can compute all three from a single pass over `count_words`'s result, or split the work; either is fine.

The average is where Rust makes you slow down.
Integer division truncates, so cast to `f64` before you divide, not after.
The test compares the result against a small tolerance because calculations with `f64` can introduce rounding error.

`count_words` is stubbed with `todo!()` again so this file compiles on its own.
Wire `text_stats` up however you like.
The test only cares about the returned tuple.

## Useful from the standard library

- Each map value is an occurrence count, so adding the values gives you the total number of words.
- The map's length gives you the number of unique words because each key appears once.
- For the average length, account for both the length of each word and the number of times it occurred.
  Convert the total characters and total words to `f64` before dividing.
- [`HashMap::values`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.values) and [`HashMap::iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter) are the two iterator entry points you'll likely use here.
