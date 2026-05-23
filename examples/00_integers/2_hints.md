# Hints

## `number_to_string`

1. Every type that implements `Display` (and `u32` does) can be turned into
   a `String` with one method call.
2. Look for `to_string` on the integer types, or the `format!` macro.

## `calculate_total_with_tax`

1. The maths is `price_cents + price_cents * (tax_rate_percent / 100.0)`.
2. `price_cents` is `u32` and `tax_rate_percent` is `f64`. You can't
   multiply them directly. Cast `price_cents` with `as f64` first.
3. To go back to `u32` for the return value, use a plain `as u32` cast.
   That *truncates* the fractional part (drops any sub-cent amount),
   which is what the test for `8.49%` pins down.

## `parse_positive_integer`

1. `&str` has a `.parse()` method that can produce many number types.
   You'll need a type annotation so it knows which one.
2. `.parse::<u32>()` returns a `Result<u32, _>`. The exercise asks for
   `0` on failure, so reach for `.unwrap_or(0)`.
3. (Forward reference: in chapter 10 you'll learn why returning `Result`
   directly is the better signature.)
