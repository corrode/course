# Wrapping up numbers

You met Rust's stance on numbers: overflow is caught rather than ignored, type
conversions are explicit, and parsing returns a `Result` you have to deal with.

## What we learned

- **Overflow isn't silent.** A plain `+` past a type's maximum panics in debug
  and wraps in release. When overflow is possible, pick the behavior you want:
  `saturating_add` (clamp), `checked_add` (`Option`), or `wrapping_add` (opt-in
  wraparound).
- **No implicit conversions.** Rust never mixes numeric types for you. Use `as`
  for a truncating cast, or `.into()` / `.try_into()` when you want a checked
  conversion.
- `as u32` on a float truncates toward zero (`1.7 as u32` is `1`); `f64::round`
  rounds to the nearest integer. We used truncation for the damage bonus because
  games drop fractional HP.
- `str::parse()` is the universal text-to-value method. It returns a `Result`;
  pair it with `.unwrap_or(...)` until you've met `Result` properly in its own
  chapter.
