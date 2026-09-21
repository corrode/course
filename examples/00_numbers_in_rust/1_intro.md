# Numbers in Rust

Rust won't mix numeric types for you, and it catches overflow in debug builds.
The types themselves will look familiar:

```rust
let byte: u8 = 255;           // a single byte, holds 0 to 255
let b: u32 = 42;              // unsigned, so it can't go negative
let a: i32 = -42;             // signed 32-bit, the everyday default
let big: i64 = 9_000_000_000; // 64-bit, for numbers too large for i32
let i: usize = 0;             // the type for sizes and indices
let price: f64 = 19.99;       // floating point (f32 is the smaller one)
```

Those are the types you'll see most often.

## No Silent Overflows

Languages handle integer overflow differently: Java wraps, Python's integers
grow to hold the result, and C wraps unsigned arithmetic but leaves signed
overflow undefined. Rust panics on integer overflow in a debug build by default.

```rust
let hp: u8 = 200;
let bonus: u8 = 100;

// Panics in debug mode because this would overflow
let total = hp + bonus; 
```

Storing that sum in an unsigned 8-bit integer in C would give 44 instead of 300.
Imagine you play Diablo and pick up a bonus item and your health bar suddenly
drops from 200 to 44.

Rust's integer methods let you choose what happens when a result does not fit:

- `a.saturating_add(b)` clamps at the maximum, so 255 stays 255.
- `a.checked_add(b)` returns `None` on overflow, so you can handle it yourself.
- `a.wrapping_add(b)` opts back into wraparound, for the times you actually want
  it.

If you want the health bar to stop at 255, use `saturating_add`.

Release builds wrap by default for speed.

I prefer to choose the overflow behavior explicitly rather than rely on the
release default. These methods also give you the same behavior in debug and
release builds.

## No Implicit Conversions

**Rust never mixes numeric types for you.** `u32 + i32` won't compile, and you
can't multiply a `u32` by an `f64` either. You convert explicitly: `as` performs
a cast that may lose information, `.into()` handles infallible conversions, and
`.try_into()` returns a `Result` when a conversion can fail.

```rust
let count: u32 = 42;
let price: f64 = 19.99;

// No implicit conversions! 
// We have to spell out the cast here. 
let total = price * count as f64; 
```

## Text into Numbers

Parsing a string can fail because the input might not be a number at all, so
`parse` hands back a `Result`.

```rust
let n: u32 = "123".parse().unwrap_or(0);
```

We'll talk about `Result` later. For now, it gives you a value that represents
either success or failure, so you can't ignore a parsing problem by accident.

Knowing that you can call `.unwrap_or(fallback_value_if_the_parsing_failed)` on
a `Result` is enough for this exercise.
