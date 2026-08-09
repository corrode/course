# Wrapping up strings and chars

You worked with all three string types: counted UTF-8 characters correctly, took
a `&str` and produced a fresh `String`, and walked a string character by
character.

## What we learned

- `&str` is a borrowed view into UTF-8 text; `String` is an owned, growable
  buffer; `char` is one Unicode scalar value. Functions that **read** take a
  `&str`, functions that **produce** return a `String`.
- `str::len` is for byte length, NOT character count. Use `s.chars().count()`
  when you mean characters.
- `str::chars()` returns an iterator. Anything that takes an iterator works on
  it: `for c in s.chars()`, `s.chars().any(...)`, `s.chars().count()`, and so
  on.
- Case conversion (`to_uppercase`, `to_lowercase`) returns a new `String`. The
  original string does not get mutated.
- The `is_ascii_*` family of functions is fast when you know the input is ASCII,
  but `char::is_uppercase` is the Unicode-aware version and is the safer
  default. 
