# Hints

## `add_health`

1. A plain `current + gain` would panic (debug) or wrap (release) once the sum
   passes 255.
2. Look for `saturating_add` on the integer types: it clamps at the maximum
   instead of overflowing.

## `damage_with_bonus`

1. The percentage describes the extra damage, not the final damage. What
   fraction of the base does it add?
2. `base` is `u32` and `bonus_percent` is `f64`, so you can't multiply them
   directly. Cast `base` with `as f64` first.
3. To go back to `u32` for the return value, use a plain `as u32` cast. That
   *truncates* the fractional part (drops any fractional HP), so the `15.5%`
   example returns `8`.

## `parse_positive_integer`

1. `parse` needs a target type. You can specify it with `::<u32>`, or let the
   function's return type guide inference once the parsed value is returned.
2. Parsing and handling failure are separate steps. Which method from the
   introduction lets you choose what happens when parsing fails?
