# Strings, &str, and Chars

*An `i32` walks up to a `String` and asks for its number. The `String` replies: "Sorry, you're not my type."*

Rust splits "string" across three cooperating types:

- `char` is one Unicode scalar value (always 4 bytes).
- `&str` is a borrowed view into UTF-8 text. Cheap to pass around.
- `String` is an owned, growable UTF-8 buffer. You own the memory.

Before we go further, two words that show up everywhere in Rust:

- **Owned** means *this value is mine; when I go out of scope, the
  memory behind it is freed*. In C++ terms, it's the object held by
  `std::unique_ptr`; in Python or Java terms, it's the role of the
  variable that decides when the object can be collected. In Rust
  every heap value has exactly one owner at a time.
- **Borrowed** means *I'm looking at someone else's value without
  taking it over*. It's the equivalent of passing a `const T&` in
  C++, or handing out a read-only pointer in C. Borrows are written
  with an `&` (or `&mut` if you also want to mutate). The borrow has
  to end before the owner is dropped, and the compiler enforces that
  for you, ruling out use-after-free and dangling pointers.

Chapter 12 is dedicated to ownership and borrowing; for now just keep
the mental picture of "one owner, many short-lived borrows."

The split between `&str` and `String` is what makes Rust strings both
fast and safe. A function that just *reads* text takes `&str`; a
function that *produces* new text returns `String`. You'll see this
rhythm again and again:

```rust
fn shout(text: &str) -> String {
    text.to_uppercase()
}

let s = String::from("hello");
let louder = shout(&s); // &String coerces to &str
```

- **`&str`** ("string slice", pronounced *stir*) is a *borrowed* view
  into text that lives elsewhere. Taking `name: &str` means "I just need
  to read this string; I'm not taking ownership of it."
- **`String`** is *owned* and heap-allocated. Returning `-> String`
  means the caller gets a fresh, owned value back.

A common gotcha: `s.len()` returns the number of *bytes*, not characters.
For character counts use `s.chars().count()`. UTF-8 means a single visible
character can take more than one byte.

You'll also meet `.chars()` a lot. It returns an iterator of `char`, and
iterators have many useful adapters like `.next()`, `.count()`, and
`.any(...)` (more on iterators in chapter 16).

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

## A note on `for` loops

The simplest way to consume an iterator is a `for` loop:

```rust
for c in "hello".chars() {
    println!("{c}");
}
```

You can read it as "for each `c` produced by the iterator on the right,
run the body once." The loop variable is a fresh binding scoped to each
iteration. Anything that produces an iterator (a `Vec`, a slice, a
`HashMap`, `0..10`, ...) works on the right-hand side.

## Where to look things up

You won't memorize Rust's `std` library, and you don't need to.
Two things you can open in separate tabs right now:

- [`std::fmt`](https://doc.rust-lang.org/std/fmt/) contains everything the
  formatting macros can do (padding, precision, hex, debug output…).
- [`str`](https://doc.rust-lang.org/std/primitive.str.html):
  the inventory of operations available on any `&str`.
