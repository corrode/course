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

## No silent overflows

If you push a number past its type's maximum, most languages won't tell you.
For example, C wraps around, Java wraps around, and Python silently increases the capacity of its integers. 
Rust won't do that.
In a debug build, it panics instead of handing back a wrong answer.

```rust
let hp: u8 = 200;
let bonus: u8 = 100;

// Panics in debug mode because this would overflow
let total = hp + bonus; 
```

If that had been in a C program, the total would now be 44 instead of 300.
Imagine you play Diablo and pick up a bonus item and your health bar suddenly drops from 200 to 44.
That would be weird.

Rust catches the overflow where it happens instead.
When a result does not fit, you must choose the behavior:

- `a.saturating_add(b)` clamps at the maximum, so 255 stays 255.
- `a.checked_add(b)` returns `None` on overflow, so you can handle it yourself.
- `a.wrapping_add(b)` opts back into wraparound, for the times you actually want it.

Which variant you choose depends on what your program needs.
If you want the health bar to stop at 255, use `saturating_add`.

Release builds wrap by default for speed.

I prefer to choose the overflow behavior explicitly rather than rely on the release default.
These methods also give you the same behavior in debug and release builds.

## No implicit conversions

**Rust never mixes numeric types for you.**
`u32 + i32` won't compile, and you can't multiply a `u32` by an `f64` either.
You convert on purpose, either with an `as` cast that truncates, or with `.into()` and `.try_into()` when you want a checked conversion.

```rust
let count: u32 = 42;
let price: f64 = 19.99;

// No implicit conversions! 
// We have to spell out the cast here. 
let total = price * count as f64; 
```

## Text into numbers

Parsing a string can fail because the input might not be a number at all, so `parse` hands back a `Result`.

```rust
let n: u32 = "123".parse().unwrap_or(0);
```

We'll talk about `Result` later.
For now, it gives you a value that represents either success or failure, so you can't ignore a parsing problem by accident.

Knowing that you can call `.unwrap_or(fallback_value_if_the_parsing_failed)` on a `Result` is enough for this exercise.
