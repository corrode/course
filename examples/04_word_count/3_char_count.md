# Counting characters

Same template as `word_count`, smaller granularity.
Count every character in the text, whitespace included.
So `"hi there"` returns `8` (seven letters plus the space).

The recipe is identical to the previous step: start a counter at `0`, walk `text.chars()` with a `for` loop, bump the counter once per iteration.

The interesting wrinkle here is *what* `chars()` actually returns.
Rust strings are UTF-8 internally, so a single visible character like `é` can take more than one byte.
`text.chars()` walks the *characters*, while `text.len()` returns the *byte* length — and the two differ as soon as you leave plain ASCII.
The unicode test below pins this down.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  walks through every `char` in a string, whitespace and all.
- [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)
  returns the *byte* length.
  Reach for `chars().count()` when you actually mean "how many characters?" — the two answers diverge the moment a non-ASCII character shows up.
