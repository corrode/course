# Counting Characters

You counted characters in the strings chapter.
The same catch applies here: `text.len()` returns bytes, not characters.
Rust strings are UTF-8 internally, so a single visible character like `é` can take more than one byte.
`text.chars()` walks Unicode scalar values (`char`s), which is what we count here.
For `"café"` that's 5 bytes but 4 characters, and the two only agree on plain ASCII.

Use a `for` loop, as in the previous exercise.
Count every character, whitespace included, so `"hi there"` returns `8` (seven letters plus the space).
The Unicode test below checks the difference between bytes and characters.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) walks through every `char` in a string, whitespace and all.
