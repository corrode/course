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

## `fallible_sum`

1. What type does parsing each token produce? Keep those `Result` values as the iterator's items.
2. `map` can apply parsing to each token. The return type tells `sum` to produce `Result<i32, ParseIntError>`.

## `lazy_consumption`

1. Creating a `map` adapter doesn't call its closure. Something must ask for an item.
2. `filter` may request several inputs before it can yield one output.
3. `by_ref` borrows the existing iterator; consuming that borrow advances the original too.
