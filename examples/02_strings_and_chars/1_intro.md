# Strings, &str, and chars

*An `i32` walks up to a `String` and asks for its number. The `String` replies: "Sorry, you're not my type."*

Rust splits "string" across three cooperating types:

- `char` is one Unicode scalar value (always 4 bytes).
- `&str` is a borrowed view into UTF-8 text. Cheap to pass around.
- `String` is an owned, growable UTF-8 buffer. You own the memory.

The split is what makes Rust strings both fast and safe. A function that
just *reads* text takes `&str`; a function that *produces* new text returns
`String`.

```rust
fn shout(text: &str) -> String {
    text.to_uppercase()
}

let s = String::from("hello");
let louder = shout(&s); // &String coerces to &str
```

A common gotcha: `s.len()` returns the number of *bytes*, not characters.
For character counts use `s.chars().count()`. UTF-8 means a single visible
character can take more than one byte.

You'll also meet `.chars()` a lot. It returns an iterator of `char`, and
iterators have many useful adapters like `.next()`, `.count()`, and
`.any(...)` (more on iterators in chapter 13).

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
