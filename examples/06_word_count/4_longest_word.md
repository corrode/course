# Longest word

Last one.
Return the length (in characters) of the longest word in the text.
If the text has no words at all, return `0`.

Walk the words with `for word in text.split_whitespace()` and measure each one with `word.chars().count()`.
Keep the largest count so far in a `let mut max = 0` variable.

The "track the running maximum" pattern shows up everywhere:

```rust
let mut max = 0;
for x in candidates {
    if x > max {
        max = x;
    }
}
```

This is the manual version of "max by some property."
The iterator version expresses the same search as a one-liner.
Doing it once by hand first makes each part of that shortcut recognizable.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) and [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) together give you a correct character count: `word.chars().count()`.
  Using `word.len()` would return the *byte* length, which differs from the character count for accented or non-Latin text.
