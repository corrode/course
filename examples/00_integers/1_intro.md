# Numbers in Rust

*This chapter was supposed to start with an integer joke. Turns out, it's pointless.*

You already know how numbers work, so we'll take a quick tour of the types and spend our time on what Rust does differently.

```rust
let byte: u8 = 255;           // a single byte, holds 0 to 255
let b: u32 = 42;              // unsigned, so it can't go negative
let a: i32 = -42;             // signed 32-bit, the everyday default
let big: i64 = 9_000_000_000; // 64-bit, for numbers too large for i32
let i: usize = 0;             // the type for sizes and indices
let price: f64 = 19.99;       // floating point (f32 is the smaller one)
```

Those are the types you'll see most often.
The first difference matters when a value no longer fits its type.

## No silent overflows

If you push a number past its type's maximum, most languages won't tell you.
For example, C wraps around, Java wraps around, and Python quietly grows the integer to fit.
Rust won't do that.
In a debug build it stops and panics instead of handing back a wrong answer.

```rust
let hp: u8 = 200;
let bonus: u8 = 100;

// The next line panics in debug mode
// because we attempt to add with overflow
let total = hp + bonus; 
```

If that bug had been in a C program, the total would be 44 instead of 300.
The player would have no idea why their health bar suddenly *dropped* from 200 to 44 after picking up a bonus item. 

Rust catches the overflow where it happens instead.
When a result may not fit, choose the behavior that matches the problem:

- `a.saturating_add(b)` clamps at the maximum, so 255 stays 255.
- `a.checked_add(b)` returns `None` on overflow, so you can handle it yourself.
- `a.wrapping_add(b)` opts back into wraparound, for the times you actually want it.

Release builds wrap by default for speed.
Reach for these methods when you need the same behavior in both debug and release builds.

## No implicit conversions

**Rust never mixes numeric types for you.**
`u32 + i32` won't compile, and you can't multiply a `u32` by an `f64` either.
You convert on purpose, either with an `as` cast that truncates, or with `.into()` and `.try_into()` when you want a checked conversion.

```rust
let count: u32 = 42;
let price: f64 = 19.99;

// We spell out the cast here. No implicit conversions. 
let total = price * count as f64; 
```

## Text into numbers

Parsing a string can fail because the input might not be a number at all, so `parse` hands back a `Result`.

```rust
let n: u32 = "123".parse().unwrap_or(0);
```

We haven't talked about `Result` yet, but the important part for now is that a failed parse cannot be ignored by accident.
Using `.unwrap_or(0)` is enough for this exercise.
We'll come back to failed parses after introducing `Result`.
