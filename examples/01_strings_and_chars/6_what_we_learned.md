# Wrapping up strings and chars

You counted UTF-8 characters, took a `&str` and produced a new `String`, and walked a string character by character.

## What we learned

- `&str` is a borrowed view into UTF-8 text; `String` is an owned, growable buffer; `char` is one Unicode scalar value.
  Functions that read text can take a `&str`, while functions that produce new text return a `String`.
- `str::len` returns the byte length.
  Use `s.chars().count()` when you want the number of `char` values.
- `str::chars()` returns an iterator.
  You can use it in a loop or with iterator methods: `for c in s.chars()`, `s.chars().any(...)`, `s.chars().count()`, and so on.
- Case conversion (`to_uppercase`, `to_lowercase`) returns a new `String`.
  It doesn't change the original string.
- The `is_ascii_*` family of functions is fast when you know the input is ASCII, but `char::is_uppercase` is the Unicode-aware version and is the safer default.
