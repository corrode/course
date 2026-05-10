# Hints

Tap a hint at a time. Each one is one notch more specific.

## `calculate_total_revenue`

1. The whole function body is one chained call. Start with `sales.iter()`.
2. There is a single-call consumer that adds up a numeric iterator.
3. `sales.iter().sum()` — and you'll need a type annotation
   (`let total: i32 = …`, or `.sum::<i32>()`) because `sum` is generic.

## `normalize_emails`

1. `into_iter()` (consume the input vec) → `map(...)` → `collect()`.
2. The closure receives an owned `String`. Call `.to_lowercase()` on it.
3. ```rust
   emails.into_iter().map(|e| e.to_lowercase()).collect()
   ```

## `select_usernames_starting_with_a`

1. `into_iter()` → `filter(...)` → `collect()`.
2. **Gotcha:** `filter`'s closure takes a *reference* to each item. Since
   the iterator yields `&str`, the closure parameter is `&&str`. Method
   calls auto-deref, so `|s| s.starts_with('a')` Just Works.
3. ```rust
   usernames.into_iter().filter(|s| s.starts_with('a')).collect()
   ```

## `find_rust_files`

1. Same shape as the previous one, but the closure now sees `&&&str`.
   Auto-deref still saves you for `.ends_with(".rs")`.
2. The function returns `Vec<String>`, not `Vec<&str>`. Add a `.map(...)`
   step that converts each `&&str` into an owned `String`.
3. ```rust
   files
       .iter()
       .filter(|name| name.ends_with(".rs"))
       .map(|name| name.to_string())
       .collect()
   ```
