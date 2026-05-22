# Mixing integers and floats

Rust never converts between numeric types implicitly. If you want
to multiply a `u32` by an `f64`, one of them has to change shape
first, and you have to say so. The `as` keyword does the conversion.

Heads-up on rounding: `.round()` rounds to the nearest integer
(with ties going away from zero), while a bare `as u32` *truncates*
the fractional part. The tests pin down which behaviour we want
here, so try one and see what happens if you guessed wrong.

## Useful from the standard library

- [`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round)
  rounds to the nearest integer (returned as an `f64`). Pair with
  `as u32` once you've rounded.
- [`as`](https://doc.rust-lang.org/std/keyword.as.html) is the cast
  operator. `1.7_f64 as u32` is `1`; `1.7_f64.round() as u32` is `2`.
