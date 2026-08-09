# Wrapping up numbers

You've just acquainted with Rust's stance on numbers: overflow is caught rather
than ignored, type conversions are explicit, and parsing returns a `Result` you
have to deal with.

## What we learned

- Overflow isn't silent. If you try to add two numbers and the result doesn't
  fit into the type, Rust will panic in debug builds and wrap in release builds.
  When overflow is possible, pick the behavior you want: `saturating_add`
  (clamp), `checked_add` (returning an `Option`), or `wrapping_add` (wrap around
  is okay).
- Rust never mixes numeric types for you. The types must align, and if they
  don't, you must convert them explicitly. For quick conversions, use `as` for a
  truncating cast, or `.into()` / `.try_into()` when you want a checked
  conversion.
- `as u32` on a float truncates toward zero (`1.7 as u32` is `1`); `f64::round`
  rounds to the nearest integer. (For example, we used truncation for the health
  damage example.)
- If you first need to convert from a different type into a number type, there
  are helper functions like `str::parse()`, which turns your text into a value
  of a requested type. It returns a `Result`, and `.unwrap_or(...)` supplies a
  fallback when parsing fails.
