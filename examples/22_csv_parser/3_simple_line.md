# A First Pass: Comma-Splitting

Start with plain values separated by commas, possibly with surrounding whitespace.
You'll handle quotes, escapes, and embedded commas in the next exercise.

Return the trimmed fields as a `Vec<String>`.

## Useful from the Standard Library

- [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split) with a `','` argument yields each comma-separated piece as a `&str`.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim) drops leading/trailing whitespace from each piece.
- [`ToString::to_string`](https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string) in a `map` step turns the borrowed pieces into the owned `String`s the return type wants.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) finishes the chain.
