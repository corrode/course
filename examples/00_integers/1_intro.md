# Numbers in Rust

*This chapter was supposed to start with an integer joke. Turns out, it's pointless.*

Numbers feel familiar, so let's skip past what you already know and go straight
to what Rust does differently.

## Overflow is not silent

Push a number past its type's maximum and most languages shrug: C wraps around,
Java wraps around, Python silently grows the integer. Rust refuses. In a debug
build, this **panics** instead of handing you a corrupted value:

```rust
let hp: u8 = 200;          // u8 holds 0–255
let bonus: u8 = 100;
let total = hp + bonus;    // panics: attempt to add with overflow
```

That's a whole class of bug — the silent wraparound that turns a 300-point
health bar into 44 — caught before it can ship. When overflow is actually
possible, you say what should happen instead of hoping:

- `a.saturating_add(b)` — clamp at the maximum (255 stays 255).
- `a.checked_add(b)` — return `None` on overflow so you can handle it.
- `a.wrapping_add(b)` — opt *in* to wraparound, for when you genuinely want it.

(Release builds wrap by default for speed, so don't lean on the panic in
production — reach for these methods whenever overflow matters.)

## No implicit conversions

Rust never mixes numeric types for you. `u32 + i32` doesn't compile, and neither
does multiplying a `u32` by an `f64`. You convert on purpose, with an `as` cast
(which truncates) or `.into()` / `.try_into()` (checked):

```rust
let count: u32 = 42;
let price: f64 = 19.99;
let total = price * count as f64;   // explicit cast, no surprises
```

A quick reference for the types you'll meet: `i32` / `u32` (signed / unsigned
32-bit, the everyday default), `usize` (sizes and indices), `u8` (a single
byte), and `f32` / `f64` (floats). The `u` versions can't be negative.

## Text into numbers

Parsing a string can fail — the input might not be a number at all — so `parse`
hands back a `Result`:

```rust
let n: u32 = "123".parse().unwrap_or(0);
```

`.unwrap_or(0)` is enough for now; the `Result` chapter shows the better way to
handle the failure instead of papering over it.
