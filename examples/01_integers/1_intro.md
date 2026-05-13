# Numbers in Rust

*I tried to start this chapter with a joke about integers; it was pointless.*

Rust has many integer types. The most common ones are `i32` (signed,
32-bit) and `u32` (unsigned, 32-bit). The `u` versions can't be negative.
For sizes and indices you'll usually see `usize`. Floating-point comes in
two flavors: `f32` and `f64`. There's no implicit conversion between
numeric types in Rust, so you'll often write `as` casts or use `.into()`
when types need to meet.

A small example to ground the syntax:

```rust
let count: u32 = 42;
let price: f64 = 19.99;
let total = price * count as f64; // explicit cast
```

Parsing strings into numbers returns a `Result`, since the input might
not actually be a number. For now, `match` or `.unwrap_or(0)` is enough
to handle that.

```rust
let n: u32 = "123".parse().unwrap_or(0);
```

## Useful from the standard library

- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
  turns a string into any type that implements `FromStr`. Returns a
  `Result`, so handle the error case.
- [`u32::to_string`](https://doc.rust-lang.org/std/primitive.u32.html#method.to_string)
  goes the other way: number to `String`. The `format!` macro works too.
- [`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round)
  rounds to the nearest integer; pair with `as u32` when you need a whole
  number.
- [`u32::checked_add`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add)
  and friends (`checked_sub`, `checked_mul`) return `Option<u32>` instead
  of overflowing. Useful when you don't trust the inputs.
