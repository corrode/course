# Strings, slices, and `format!`

Before you open `main.rs`, here's the bit of Rust you'll need to know.
The exercise is small on purpose; this page exists so you don't have to
guess at the syntax.

## Two flavors of string

Rust splits "string" into two types, and you'll see both right away in
this chapter's signature:

```rust
fn format_welcome_message(name: &str) -> String
```

- **`&str`** ("string slice", pronounced *stir*) is a *borrowed* view
  into some text that lives elsewhere. Cheap to pass around. The
  parameter `name: &str` means "I just need to read this string; I'm
  not taking ownership of it."
- **`String`** is an *owned*, heap-allocated, growable buffer. Returning
  `-> String` means the caller gets a fresh, owned value back.

You'll spend the next few chapters getting comfortable with this
distinction. For now: take a `&str` in, hand a `String` out.

## Building a `String` with `format!`

The fastest way to assemble a new `String` is the `format!` macro. It
works like `println!`, except instead of printing, it returns the
formatted text:

```rust
let name = "Alice";
let greeting: String = format!("Hello, {name}!");
```

A few things worth noticing:

- The `{name}` inside the string is a **captured identifier**. Rust
  pulls the variable from the surrounding scope. (Pre-2021 code uses
  `format!("Hello, {}!", name)` instead; both still work.)
- The macro returns a `String`, ready to return from your function.
- The exclamation mark (`!`) means it's a macro, not a regular function
  call. You'll learn what that distinction buys you later; for now,
  treat it as a quirky bit of punctuation.

## Where to look things up

You won't memorize Rust's `std` library, and you don't need to. 
Two things you can open in separate tabs right now:

- [`std::fmt`](https://doc.rust-lang.org/std/fmt/) contains everything the
  formatting macros can do (padding, precision, hex, debug output…).
- [`str`](https://doc.rust-lang.org/std/primitive.str.html):
  the inventory of operations available on any `&str`.

