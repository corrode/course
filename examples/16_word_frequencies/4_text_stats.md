# Text statistics

The orchestrator step, and the one with the most aggregations in a single body.
`text_stats` returns three numbers about a piece of text: total word count, number of unique words, and the average word length as an `f64`.
You can compute all three from a single pass over `count_words`'s result, or split the work; either is fine.

The average is where Rust makes you slow down.
Integer division truncates, so cast to `f64` before you divide, not after.
And once the result is a float, the test can't check it with `==`: floats don't land on exact values, so it compares against a small tolerance instead.

`count_words` is stubbed with `todo!()` again so this file compiles on its own.
Wire `text_stats` up however you like.
The test only cares about the returned tuple.

## Useful from the standard library

- The total word count is the sum of every value in the map: `counts.values().sum::<usize>()`.
- The unique-word count is `counts.len()`.
- For the average length, sum `key.chars().count() * count` across the map (or sum `word.len()` straight from a fresh `text.split_whitespace()` pass) and divide by the total.
  Watch the integer-division trap: cast both operands to `f64` before the divide.
- [`HashMap::values`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.values) and [`HashMap::iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter) are the two iterator entry points you'll likely use here.
