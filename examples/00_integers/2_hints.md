# Hints

## `number_to_string`

1. Every type that implements `Display` (and `u32` does) can be turned into
   a `String` with one method call.
2. Look for `to_string` on the integer types, or the `format!` macro.
3. ```rust
   fn number_to_string(number: u32) -> String {
       number.to_string()
   }
   ```

## `damage_with_bonus`

1. The maths is `base + base * (bonus_percent / 100.0)`.
2. `base` is `u32` and `bonus_percent` is `f64`. You can't
   multiply them directly. Cast `base` with `as f64` first.
3. To go back to `u32` for the return value, use a plain `as u32` cast.
   That *truncates* the fractional part (drops any fractional HP),
   which is what the `15.5%` test pins down.
4. ```rust
   fn damage_with_bonus(base: u32, bonus_percent: f64) -> u32 {
       (base as f64 + base as f64 * (bonus_percent / 100.0)) as u32
   }
   ```

## `parse_positive_integer`

1. `&str` has a `.parse()` method that can produce many number types.
   You'll need a type annotation so it knows which one.
2. `.parse::<u32>()` returns a `Result<u32, _>`. The exercise asks for
   `0` on failure, so reach for `.unwrap_or(0)`.
3. (Forward reference: in chapter 11 you'll learn why returning `Result`
   directly is the better signature.)
4. ```rust
   fn parse_positive_integer(input: &str) -> u32 {
       input.parse::<u32>().unwrap_or(0)
   }
   ```
