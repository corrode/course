# Numbers in Rust

*This chapter was supposed to start with an integer joke. Turns out, it's pointless.*

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
