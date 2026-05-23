# `damage_with_crit`

Rust never converts between numeric types implicitly. If you want
to multiply a `u32` by an `f64`, one of them has to change shape
first, and you have to say so. The `as` keyword does the conversion.

The function takes a base damage as `u32` and a crit bonus as an
`f64` percentage (so `50.0` means "+50%"), and returns the final
damage as a `u32`. You'll need to:

1. Turn the percentage into a multiplier (divide by `100.0`).
2. Multiply it by the base — but only after casting the base to
   `f64`, because Rust won't mix the two for you.
3. Add the bonus to the base.
4. Cast back to `u32` to return.

That last cast *truncates* the fractional part toward zero, which
is exactly what we want here: most games quantise to whole HP, so
`8.085` damage becomes `8`, not `9`. The final test pins this
down.

## Useful from the standard library

- [`as`](https://doc.rust-lang.org/std/keyword.as.html) is the cast
  operator. `1.7_f64 as u32` is `1` (truncation), not `2`. That
  same truncation is what drops fractional HP in this exercise.
- [`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round)
  exists too, and rounds to the nearest integer. It's the right
  tool when you want nearest-integer rounding, but this exercise
  asks for truncation, so you won't need it here.
