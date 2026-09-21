# Exercise Break: Word Count

Enough syntax for a moment! You know more syntax than you might realize, so
let's put it to work. We'll write a small word-count library using strings,
`for` loops, and functions.

## Splitting Text into Words

The standard library provides `split_whitespace` for `&str`; you can use it in a
`for` loop to walk through each word in a string:

```rust
for word in "hello  world\nrust".split_whitespace() {
    println!("{word}"); // hello, world, rust
}
```

It splits on any run of whitespace (spaces, tabs, newlines) and skips empties,
which is what you want for natural text. `.split_whitespace()` returns an
iterator over the words. A `for` loop consumes that iterator without requiring
its concrete type.

## Walking Characters

The same idea works at the character level via `.chars()`:

```rust
for c in "hi".chars() {
    println!("{c}"); // h, i
}
```

With `.split_whitespace()` for words and `.chars()` for characters, a `for` loop
can count either one.
