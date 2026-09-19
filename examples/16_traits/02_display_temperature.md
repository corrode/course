# Implementing Display for Your Own Type

`Display` is the trait behind the `{}` placeholder in `println!`, `format!`, and friends.
Implement it for your struct, and you can format its values as user-facing text just like a number or a `String`.

The trait lives in `std::fmt` and looks like this:

```rust
pub trait Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
```

That signature has a lot of punctuation for a method whose body is often one line.
You can delegate to `write!`, which uses the same template syntax as `println!` but writes into the formatter:

```rust
use std::fmt;

struct Pixel(u8, u8, u8);

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}
```

After that, `format!("{}", Pixel(255, 0, 0))` produces `"#FF0000"`.

## Why Isn't There a `#[derive(Display)]`?

Because there's no obvious default.
`Debug` has one (print the type name and fields), but `Display` is for *human-readable* output and only you know what that should look like for your type.
So you write it by hand.
The Java/C# parallel is overriding `toString()`; the Python one is `__str__`.

## Useful from the Standard Library

- [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) is the trait.
  `use std::fmt;` and then `impl fmt::Display for T` is the idiomatic spelling.
- [`write!`](https://doc.rust-lang.org/std/macro.write.html) is the formatter-targeted cousin of `println!`.
  It returns `std::fmt::Result`, which is exactly what your `fmt` method needs to return, so a single `write!(...)` call is usually the whole body.
- The [formatting syntax reference](https://doc.rust-lang.org/std/fmt/#precision) explains how to set floating-point precision.
