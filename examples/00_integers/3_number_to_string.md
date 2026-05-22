# Number to string

Start with the simplest direction: take a number, hand back its
textual form. Rust's standard library has a one-shot method for
this on every primitive integer type, and the `format!` macro from
chapter 0 works too. Either is fine.

## Useful from the standard library

- [`u32::to_string`](https://doc.rust-lang.org/std/primitive.u32.html#method.to_string)
  produces a `String` from any number that implements `Display` (every
  primitive does).
- [`format!`](https://doc.rust-lang.org/std/macro.format.html) builds a
  `String` from a template, e.g. `format!("{n}")`.

> [!TIP]
> Press `Ctrl/Cmd + Enter` to run the tests without leaving the
> keyboard. The **Run** button does the same thing.
