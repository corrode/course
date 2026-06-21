# Wrapping up the word counter

You glued together the chapters so far: a `HashMap` keyed by lowercased words, an `into_iter()` to escape the borrow checker, a `max_by_key` to pick a winner, a `filter`/`map`/`collect` to slice the map, and a few aggregations to compute summary stats.

## What we learned

- `split_whitespace()` is the right default for word-splitting in natural text.
  It collapses runs of whitespace and skips empties.
- Lowercasing keys (or any other normalization step) belongs to the same pipeline that builds the map, not to the consumer side.
- `into_iter` on a `HashMap` is the standard escape hatch when you need to return owned data out of it.
  `iter` only hands out borrows.
- `max_by_key` returns an `Option`, so empty input naturally collapses to `None` without a special-case branch.
- "Filter, then map, then collect" composes the same way over a `HashMap` as it does over a `Vec`.
  The collection on either end is just where the items live.
- Watch the integer-division trap when computing averages: divide *after* casting to `f64`, not before.
  `f64` comparisons need a tolerance (`(a - b).abs() < eps`); never `==`.
- Tuples like `(usize, usize, f64)` work for tiny ad-hoc returns, but a named struct (`TextStats { total, unique, avg_len }`) reads better at the call site as soon as a function takes off in scope.

## An optional detour

You now have every tool you need to build a small program from scratch: structs, enums, iterators, `Option`, `Result`, vectors, and strings.
There's an optional **Creative Break** chapter, an open-ended password validator project rather than a guided lesson.
It isn't part of the main sequence and nothing later depends on it, so take it whenever you want a change of pace, or skip straight ahead.
