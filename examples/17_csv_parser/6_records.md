# From rows to records

Parallel `Vec`s of headers and row values are awkward to consume.
Most code wants to ask "what's the `name` for this row?" — a job for
a `HashMap<String, String>` per row.

Pair headers with each row using
[`Iterator::zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
and `collect` into a `HashMap`. You'll need `cloned()` on both
iterators because the map wants owned `String`s but iteration
yields `&String`.
