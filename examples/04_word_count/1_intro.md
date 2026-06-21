# Exercise break: word count

Congratulations, you've covered enough of Rust to write a small, useful program without any extra ceremony!
Time for a short break to enjoy the view.

The next three steps build a tiny word-count library.
The whole chapter is just strings (the strings chapter), `for` loops (the conditionals and loops chapter), and functions (the functions chapter) applied together.

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
Don't worry yet about what kind of thing `.split_whitespace()` returns (it's an *iterator*, which we will cover later). 

## Walking characters

The same idea works at the character level via `.chars()`:

```rust
for c in "hi".chars() {
    println!("{c}"); // h, i
}
```

Both `.split_whitespace()` and `.chars()` are exactly the tools we need to count words and characters in a string. 
