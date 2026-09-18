# Wrapping Up `Option`

You consumed `Option`s with fallbacks and combinators, produced new ones from string and slice operations, and chained `find` and `map` to turn a search into the exact return type the signature asked for.

## What We Learned

- `Option<T>` is Rust's stand-in for "value or absence".
  You cannot use it as a `T` without first extracting the value.
- You can use `match` to spell out both cases.
  For common patterns, reach for `unwrap_or`, `map`, and `map_or` to keep call sites short.
- `if let Some(x) = ...` is the lighter alternative to `match` when you only care about the `Some` branch.
- `Option::map` mirrors the iterator method of the same name: it transforms the inside if present, leaves `None` alone.
- Many standard-library operations already return `Option`: `iter().next()`, `slice.first()`, `Vec::pop`, `HashMap::get`, `iter().find(...)`.
- `unwrap` and `expect` extract the value but panic on `None`.
  Use them in tests or when you've already ruled out `None`; otherwise prefer the safer combinators.
- The `|x| ...` syntax is a closure: a tiny anonymous function.
  It shows up everywhere with `Option` and iterators.
  We'll cover closures in more detail later.
