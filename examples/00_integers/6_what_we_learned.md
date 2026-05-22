# Wrapping up numbers

You converted numbers to strings, mixed integers with floats (carefully),
and turned text back into a number. The shape "convert, then operate"
is one you'll keep using.

## What we learned

- Rust never converts between numeric types implicitly. Use `as` for a
  truncating cast, or `.into()` / `.try_into()` when you want a checked
  conversion.
- `format!` and `to_string()` produce a `String` from any value that
  implements `Display`.
- `f64::round()` rounds to the nearest integer; `as u32` on its own
  truncates. They give different answers for `1.7`.
- `str::parse()` is the universal text-to-value method. It returns a
  `Result`; pair it with `.unwrap_or(...)` until you've met `Result`
  properly in chapter 10.
- For arithmetic that might overflow, `checked_add` / `checked_sub` /
  `checked_mul` return an `Option<T>` instead of panicking. Worth
  remembering once you stop trusting your inputs.
