# Numbers in Rust

*This chapter was supposed to start with an integer joke. Turns out, it's pointless.*

You already know how numbers work, so let's start with a quick tour of the types
and then spend the rest of the chapter on what Rust does differently.

```rust
let a: i32 = -42;             // signed 32-bit, the everyday default
let b: u32 = 42;              // unsigned, so it can't go negative
let big: i64 = 9_000_000_000; // 64-bit, for numbers too large for i32
let i: usize = 0;             // the type for sizes and indices
let byte: u8 = 255;           // a single byte, holds 0 to 255
let price: f64 = 19.99;       // floating point (f32 is the smaller one)
```

That covers almost everything you'll reach for. Now the parts that catch people
out.

## Overflow doesn't pass silently

Push a number past its type's maximum and most languages just shrug. C wraps
around, Java wraps around, Python quietly grows the integer to fit. Rust won't do
that to you. In a debug build it stops and panics instead of handing back a wrong
answer.

```rust
let hp: u8 = 200;       // a u8 holds 0 to 255
let bonus: u8 = 100;
let total = hp + bonus; // panics: attempt to add with overflow
```

The silent version of that bug is the one that quietly turns a 300-point health
bar into 44 and ships to players. Rust catches it at the source instead. When a
sum really might overflow, you tell Rust what you want to happen.

- `a.saturating_add(b)` clamps at the maximum, so 255 stays 255.
- `a.checked_add(b)` returns `None` on overflow, so you can handle it yourself.
- `a.wrapping_add(b)` opts back into wraparound, for the times you actually want it.

(Release builds wrap by default for speed, so don't lean on the panic in
production. Reach for these methods whenever overflow matters.)

## No implicit conversions

Rust never mixes numeric types for you. `u32 + i32` won't compile, and you can't
multiply a `u32` by an `f64` either. You convert on purpose, either with an `as`
cast that truncates, or with `.into()` and `.try_into()` when you want a checked
conversion.

```rust
let count: u32 = 42;
let price: f64 = 19.99;
let total = price * count as f64; // the cast is spelled out, so no surprises
```

## Text into numbers

Parsing a string can fail, because the input might not be a number at all. So
`parse` hands back a `Result`.

```rust
let n: u32 = "123".parse().unwrap_or(0);
```

`.unwrap_or(0)` is fine for now. The `Result` chapter shows a better way to deal
with the failure instead of papering over it.
