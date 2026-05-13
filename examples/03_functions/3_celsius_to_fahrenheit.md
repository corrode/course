# Celsius to Fahrenheit

A function with one numeric input and one numeric output. The
conversion formula is `(c * 9.0 / 5.0) + 32.0`. Both the parameter
and the return are `f64`.

A few small things to notice:

- The numeric literals all carry a `.0` so they're `f64`. Mixing
  `f64` and `i32` in one expression won't compile; Rust never
  silently widens between numeric types.
- The expression is the return value. No `return`, no semicolon at
  the end.

## Useful from the standard library

- The `*`, `/`, and `+` operators on `f64` are all you need. Order
  of operations is the usual one (`*` and `/` before `+`), but
  parentheses make the intent obvious and don't cost anything.
- `f64` doesn't implement `Eq`, only `PartialEq`, so the test uses
  `(a - b).abs() < eps` for a tolerance check rather than `==`.
