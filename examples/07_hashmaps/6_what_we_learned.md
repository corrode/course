# Wrapping up hashmaps

You built a configuration map from scratch, updated and read values,
and used the `entry` API to write the canonical word counter in one
line.

## What we learned

- `HashMap<K, V>` stores key-value pairs and looks them up in
  (average) constant time. All keys share one type; all values share
  one type. Mix-and-match goes through enums.
- `HashMap` lives in `std::collections` and isn't in the prelude, so
  you need a `use std::collections::HashMap;` at the top of the file.
- `insert` does double duty: it adds a new pair or replaces an
  existing one, and returns the previous value (if any) as
  `Option<V>`.
- `get` returns `Option<&V>`, never null. Combine with `unwrap_or`
  (or `cloned().unwrap_or(...)` when you need an owned value) to
  handle the missing case.
- `entry(key).or_insert(default)` is the idiomatic "look up; if
  missing, insert a default; then return a `&mut V`" pattern. It does
  one lookup instead of two and sidesteps the borrow checker.
- Reach through a `&mut T` with `*` to update the value it points
  at: `*map.entry(k).or_insert(0) += 1`. References get a proper
  treatment in chapter 11.
