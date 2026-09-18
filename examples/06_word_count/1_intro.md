# Exercise break: word count

You now have enough Rust to write a small word-count library.
You'll combine strings, `for` loops, and functions without having to learn another language feature.

This first version is the running example we'll keep refactoring throughout the course.

## Splitting text into words

The standard library hands you `split_whitespace` on every `&str`.
You can use it in a `for` loop to walk through each word in a string:

```rust
for word in "hello  world\nrust".split_whitespace() {
    println!("{word}"); // hello, world, rust
}
```

It splits on any run of whitespace (spaces, tabs, newlines) and skips empties, which is what you want for natural text.
`.split_whitespace()` returns an iterator over the words.
A `for` loop consumes that iterator without requiring its concrete type.

## Walking characters

The same idea works at the character level via `.chars()`:

```rust
for c in "hi".chars() {
    println!("{c}"); // h, i
}
```

With `.split_whitespace()` for words and `.chars()` for characters, a `for` loop can count either one.
