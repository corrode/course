# Wrapping up the word counter

Your word counter now uses a `HashMap` keyed by lowercased words and `max_by_key` to find the most common one.
You used `into_iter()` to take ownership of the result and combined counts and lengths into summary statistics.

## What we learned

- `split_whitespace()` is the right default for word-splitting in natural text.
  It collapses runs of whitespace and skips empties.
- Normalize keys as you build the map.
  Here, lowercasing before insertion makes `Hello` and `hello` contribute to the same count.
- `into_iter` transfers the keys and values out of a `HashMap`, which lets you return owned data without cloning it.
  In contrast, `iter` only lends you references to entries that remain in the map.
- `max_by_key` returns an `Option`, so empty input naturally collapses to `None` without a special-case branch.
- Watch the integer-division trap when computing averages: divide *after* casting to `f64`, not before.
  Tests for calculated `f64` values usually compare a tolerance such as `(a - b).abs() < eps` instead of using `==`.
- Tuples like `(usize, usize, f64)` work for tiny ad-hoc returns, but a named struct (`TextStats { total, unique, avg_len }`) reads better at the call site as the function grows.

## An optional detour

You now have every tool you need to build a small program from scratch: structs, enums, iterators, `Option`, `Result`, vectors, and strings.
If you want a change of pace, the optional **Creative Break** is an open-ended password validator project rather than a guided lesson.
Nothing later depends on it, so you can take the detour now or keep going.
