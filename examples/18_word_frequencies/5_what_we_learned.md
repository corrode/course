# Wrapping up the word counter

You glued together the chapters so far: a `HashMap` keyed by lowercased words, an `into_iter()` to escape the borrow checker, a `max_by_key` to pick a winner, and a few aggregations to compute summary stats.

## What we learned

- `split_whitespace()` is the right default for word-splitting in natural text.
  It collapses runs of whitespace and skips empties.
- Lowercasing keys (or any other normalization step) belongs to the same pipeline that builds the map, not to the consumer side.
- `into_iter` transfers the keys and values out of a `HashMap`, which lets you return owned data without cloning it.
  In contrast, `iter` only lends you references to entries that remain in the map.
- `max_by_key` returns an `Option`, so empty input naturally collapses to `None` without a special-case branch.
- Watch the integer-division trap when computing averages: divide *after* casting to `f64`, not before.
  Tests for calculated `f64` values usually compare a tolerance such as `(a - b).abs() < eps` instead of using `==`.
- Tuples like `(usize, usize, f64)` work for tiny ad-hoc returns, but a named struct (`TextStats { total, unique, avg_len }`) reads better at the call site as soon as a function takes off in scope.

## An optional detour

You now have every tool you need to build a small program from scratch: structs, enums, iterators, `Option`, `Result`, vectors, and strings.
If you want a change of pace, the optional **Creative Break** is an open-ended password validator project rather than a guided lesson.
Nothing later depends on it, so you can take the detour now or keep going.
