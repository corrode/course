# From rows to records

Parallel `Vec`s of headers and row values are awkward to consume.
Most code wants to ask "what's the `name` for this row?" A job for
a `HashMap<String, String>` per row.

Pair headers with each row using
[`Iterator::zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
and `collect` into a `HashMap`. You'll need `cloned()` on both
iterators because the map wants owned `String`s but iteration
yields `&String`.

## Useful from the standard library

- [`Iterator::zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
  pairs items from two iterators. Stops at the shorter of the two,
  which silently drops trailing fields when a row has the wrong
  arity.
- [`Iterator::cloned`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)
  turns `&String` items into owned `String`s. Apply on both sides of
  the `zip` so the `HashMap` ends up owning its keys and values.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
  on a `(K, V)` iterator builds a `HashMap` straight from the type
  annotation. The outer `map` then collects per-row maps into the
  final `Vec`.
- [`HashMap::get`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get)
  is what callers will use afterwards: `record.get("name")`.
