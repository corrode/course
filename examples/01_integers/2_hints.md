# Hints

Tap a hint at a time. Each one is one notch more specific.

## `number_to_string`

1. Every type that implements `Display` (and `u32` does) can be turned into
   a `String` with one method call.
2. Look for `to_string` on the integer types, or the `format!` macro.

## `calculate_total_with_tax`

1. The maths is `price + price * (tax_rate / 100.0)`.
2. `price` is `u32` and `tax_rate` is `f64`. You can't multiply them
   directly — cast `price` with `as f64` first.
3. To go back to `u32` for the return value, *round* (don't truncate).
   `f64::round` then `as u32` gives you the rounded integer that the
   `108` test expects.

## `parse_positive_integer`

1. `&str` has a `.parse()` method that can produce many number types.
   You'll need a type annotation so it knows which one.
2. `.parse::<u32>()` returns a `Result<u32, _>`. The exercise asks for
   `0` on failure, so reach for `.unwrap_or(0)`.
3. (Forward reference: in chapter 8 you'll learn why returning `Result`
   directly is the better signature.)
