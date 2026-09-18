# Longest Word

Return the length (in characters) of the longest word in the text.
If the text has no words at all, return `0`.

Use a `for` loop.
What information do you need to remember after examining each word?

This is the manual version of "max by some property."
The iterator version expresses the same search as a one-liner.
Doing it once by hand first makes each part of that shortcut recognizable.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) and [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) together count Unicode scalar values.
  Using `word.len()` would return the *byte* length, which differs from the character count for accented or non-Latin text.
