# Counting characters

Same shape as `word_count`, smaller granularity. Count every
character in the text *except* whitespace, so `"hi there"` returns
`7` (the seven letters; the space doesn't count).

The recipe: start a counter at `0`, walk `text.chars()` with a
`for` loop, and only bump the counter when the character is *not*
whitespace.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  walks through every `char` in a string. Prefer this over
  `str::len` for "how many letters?" — `len` returns the byte
  length, which differs from the character count the moment you
  hit a non-ASCII character.
- [`char::is_whitespace`](https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace)
  returns `true` for spaces, tabs, newlines, and a handful of
  Unicode oddballs.
