# Exercise break: word count

Congratulations, you've covered enough of Rust to write a small, useful program without any extra ceremony!
Time for a short break to enjoy the view.

Over the next three steps, we'll build a tiny word-count library.
We'll combine the strings, `for` loops, and functions you already know rather than introduce another language feature.

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
Don't worry yet about the exact type returned by `.split_whitespace()`.
It is an *iterator*, and we'll unpack that term later.

## Walking characters

The same idea works at the character level via `.chars()`:

```rust
for c in "hi".chars() {
    println!("{c}"); // h, i
}
```

With `.split_whitespace()` for words and `.chars()` for characters, a `for` loop can count either one.
