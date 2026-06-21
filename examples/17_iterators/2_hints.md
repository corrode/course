# Hints

## `sum`

1. The whole function body is one chained call.
   Start with `sales.iter()`.
2. There is a single-call consumer that adds up a numeric iterator.
3. `sales.iter().sum()`.
   And you'll need a type annotation (`let total: i32 = …`, or `.sum::<i32>()`) because `sum` is generic.

## `map`

1. `into_iter()` (consume the input vec) → `map(...)` → `collect()`.
2. The closure receives an owned `String`.
   Call `.to_lowercase()` on it.

## `filter`

1. `into_iter()` → `filter(...)` → `collect()`.
2. **Gotcha:** `filter`'s closure takes a *reference* to each item.
   Since the iterator yields `&str`, the closure parameter is `&&str`.
   Method calls auto-deref, so `|s| s.starts_with('a')` Just Works.

## `filter_to_string`

1. Same as the previous one, but the closure now sees `&&&str`.
   Auto-deref still saves you for `.ends_with(".rs")`.
2. The function returns `Vec<String>`, not `Vec<&str>`.
   Add a `.map(...)` step that converts each `&&str` into an owned `String`.
