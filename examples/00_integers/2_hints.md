# Hints

## `number_to_string`

1. Every type that implements `Display` (and `u32` does) can be turned into
   a `String` with one method call.
2. Look for `to_string` on the integer types, or the `format!` macro.

## `damage_with_crit`

1. The maths is `base + base * (crit_bonus_percent / 100.0)`.
2. `base` is `u32` and `crit_bonus_percent` is `f64`. You can't
   multiply them directly. Cast `base` with `as f64` first.
3. To go back to `u32` for the return value, use a plain `as u32` cast.
   That *truncates* the fractional part (drops any fractional HP),
   which is what the `15.5%` test pins down.

## `parse_positive_integer`

1. `&str` has a `.parse()` method that can produce many number types.
   You'll need a type annotation so it knows which one.
2. `.parse::<u32>()` returns a `Result<u32, _>`. The exercise asks for
   `0` on failure, so reach for `.unwrap_or(0)`.
3. (Forward reference: in chapter 10 you'll learn why returning `Result`
   directly is the better signature.)
