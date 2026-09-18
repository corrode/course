# Hints

## `sum`

1. The whole function body is one chained call.
   Start with `sales.iter()`.
2. There is a single-call consumer that adds up a numeric iterator.
3. `sales.iter().sum()`.
   When you return this expression directly, the function's `i32` return type tells `sum` which type to produce.

## `map`

1. `into_iter()` (consume the input vec) → `map(...)` → `collect()`.
2. The closure receives an owned `String`.
   Call `.to_lowercase()` on it.

## `filter`

1. `into_iter()` → `filter(...)` → `collect()`.
2. **Gotcha:** `filter`'s closure takes a *reference* to each item.
   Since the iterator yields `&str`, the closure parameter is `&&str`.
   Method calls auto-deref, so `|s| s.starts_with('a')` works without an explicit dereference.

## `filter_to_string`

1. Same as the previous one, but the closure now sees `&&&str`.
   Method-call auto-deref also works for `.ends_with(".rs")`.
2. The function returns `Vec<String>`, not `Vec<&str>`.
   Add a `.map(...)` step that converts each `&&str` into an owned `String`.
