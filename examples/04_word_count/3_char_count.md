# Counting characters

This looks like the same exercise again, but Rust has a surprise waiting: `text.len()` does *not* return the number of characters.
Rust strings are UTF-8 internally, so a single visible character like `é` can take more than one byte.
`text.len()` gives the *byte* length, while `text.chars()` walks the actual characters.
For `"café"` that's 5 bytes but 4 characters, and the two only agree on plain ASCII.

So reach for `text.chars()`: start a counter at `0`, walk the characters with a `for` loop, and bump the counter once per iteration.
Count every character, whitespace included, so `"hi there"` returns `8` (seven letters plus the space).
The unicode test below pins the bytes-vs-characters difference down.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  walks through every `char` in a string, whitespace and all.
- [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)
  returns the *byte* length.
  Reach for `chars().count()` when you actually mean "how many characters?" The two answers diverge the moment a non-ASCII character shows up.
