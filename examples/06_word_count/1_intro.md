# Exercise break: word count

Congratulations, you've covered enough of Rust to write a small, useful program without any extra ceremony!
Time for a short break to enjoy the view.

A small word-count library puts these concepts to work.
Its implementation combines strings, `for` loops, and functions without introducing another language feature.

This first version is the running example we'll keep refactoring throughout the course.

## Splitting text into words

The standard library hands you `split_whitespace` on every `&str`.
For now, treat it as a "black box" that lets a `for` loop walk through each word in a string:

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
