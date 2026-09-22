# Quotes, Embedded Commas, and Escapes

A field can be wrapped in double quotes, in which case any commas *inside* the
quotes are part of the field, not separators. And a literal `"` inside a quoted
field is encoded as `""` (two quotes).

Preserve whitespace rather than trimming fields.
[RFC 4180, section 2](https://www.rfc-editor.org/rfc/rfc4180#section-2) says:

> Spaces are considered part of a field and should not be ignored.

Spaces are part of the data, whether the field is quoted or not: `a, b` becomes
`["a", " b"]`, and `" a ",b` becomes `[" a ", "b"]`. Remove the surrounding
quotes, but keep the spaces inside them.

Implement the cases in the order shown by the tests:

  1. Plain `a,b,c` and simply quoted `"a","b","c"` (the basic test).
  2. Commas inside quoted fields: `"a,b",c`.
  3. Escaped quotes: `"a""b",c` -> [`a"b`, `c`].
  4. Whitespace in unquoted and quoted fields.

Escaped quotes are the tricky part: the same character can be syntax or data.
It's worth getting one case working at a time. If a test fails, trace its input
on paper and mark which characters belong in the output.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  is the entry point for character-level iteration.
- [`Iterator::peekable`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable)
  wraps an iterator so you can look ahead one character.
- [`Peekable::peek`](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.peek)
  returns `Option<&Item>` without advancing.
- [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) swaps the
  current `String` with a fresh empty one in a single move. That saves you a
  copy compared with `current.clone()` followed by `current.clear()`.
