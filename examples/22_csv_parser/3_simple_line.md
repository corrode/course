# A first pass: comma-splitting

Before tackling the messy realities of CSV (quotes, escapes, embedded commas), let's handle the trivial case: a line that's nothing but plain values separated by commas, possibly with surrounding whitespace.
This is what most "I'll just split on commas" CSV parsers do, and it's also why so many of them break.

Use [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split) and [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim).
Collect into a `Vec<String>`.

## Useful from the standard library

- [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split) with a `','` argument yields each comma-separated piece as a `&str`.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim) drops leading/trailing whitespace from each piece.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string) in a `map` step turns the borrowed pieces into the owned `String`s the return type wants.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) finishes the chain.
  The body fits on one line: `line.split(',').map(|s| s.trim().to_string()).collect()`.
