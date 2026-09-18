# Wrapping up numbers

You chose how to handle overflow, converted numeric types explicitly, and used a fallback when parsing failed.

## What we learned

- If you add two numbers and the result doesn't fit the type, Rust panics in debug builds and wraps in release builds.
  When overflow is possible, pick the behavior you want: `saturating_add` (clamp), `checked_add` (return an `Option`), or `wrapping_add` (wrap around).
- Rust never mixes numeric types for you.
  If the types don't match, you must convert them explicitly.
  For quick conversions, use `as` for a truncating cast, or `.into()` / `.try_into()` when you want a checked conversion.
- `as u32` on a float truncates toward zero (`1.7 as u32` is `1`); `f64::round` rounds to the nearest integer.
  We used truncation for the damage example.
- `str::parse()` turns text into a value of the type you ask for.
  It returns a `Result`, and `.unwrap_or(...)` supplies a fallback when parsing fails.
