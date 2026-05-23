# Mixing integers and floats

Rust never converts between numeric types implicitly. If you want
to multiply a `u32` by an `f64`, one of them has to change shape
first, and you have to say so. The `as` keyword does the conversion.

This exercise also follows the real-world rule for money:
**represent amounts as whole cents in an integer**, not as `f64`
dollars. Floats can't represent values like `0.10` exactly, so a
seemingly tidy `10.0 * 0.085` may come back as `0.8499999999`, and
that creeps into every total you compute. Using integer cents
sidesteps the whole class of bug.

The tax rate stays an `f64` (so `8.5` means 8.5%), and the function
has to combine the two: cast the price to `f64` to multiply, then
cast the tax back to `u32` to add. `as u32` *truncates* the
fractional part toward zero, which is exactly what we want here —
sub-cent amounts are simply dropped.

## Useful from the standard library

- [`as`](https://doc.rust-lang.org/std/keyword.as.html) is the cast
  operator. `1.7_f64 as u32` is `1` (truncation), not `2`. That
  same truncation is what drops fractional cents in this exercise.
- [`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round)
  exists too, and rounds to the nearest integer. It's the right
  tool when you want bankers'-style rounding, but this exercise
  asks for truncation, so you won't need it here.
