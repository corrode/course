# Wrapping up strings and chars

You worked with all three string types: counted UTF-8 characters
correctly, took a `&str` and produced a fresh `String`, and walked
a string character by character to answer a yes/no question.

## What we learned

- `&str` is a borrowed view into UTF-8 text; `String` is an owned,
  growable buffer; `char` is one Unicode scalar value. Functions that
  read take `&str`, functions that produce return `String`.
- `str::len` is byte length, not character count. Use
  `s.chars().count()` when you mean characters.
- `str::chars()` returns an iterator. Anything that takes an iterator
  works on it: `for c in s.chars()`, `s.chars().any(...)`,
  `s.chars().count()`, and so on.
- Case conversion (`to_uppercase`, `to_lowercase`) returns a new
  `String`. Originals are immutable.
- `char::is_uppercase` is the Unicode-aware classifier; the
  `is_ascii_*` family is faster when you know the input is ASCII.
