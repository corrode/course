# `damage_with_bonus`

Rust never converts between numeric types implicitly.
If you want to multiply a `u32` by an `f64`, one of them has to be converted first.
The `as` keyword is probably the simplest way to do the conversion.

The function takes base damage as a `u32` and an `f64` bonus percentage, then returns the final damage as a `u32`.
A bonus of `50.0` means adding half of the base damage again, whether it came from a critical hit, equipment, or some other modifier.
Keep the calculation in `f64` so the fractional percentage is not lost, then convert the final damage back to `u32`.

Converting the final value back to `u32` *truncates* the fractional part toward zero.
That matches games which use whole HP, so `8.085` damage becomes `8`, not `9`.
The final test checks that we truncate rather than round.

## Useful from the standard library

- [`as`](https://doc.rust-lang.org/std/keyword.as.html) is the cast operator.
  `1.7_f64 as u32` is `1`, not `2`, because the cast truncates.
- [`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round) rounds to the nearest integer instead.
  This exercise asks for truncation, so compare the two behaviors before choosing.
