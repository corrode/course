# Quotes, Embedded Commas, and Escapes

A field can be wrapped in double quotes, in which case any commas *inside* the quotes are part of the field, not separators.
And a literal `"` inside a quoted field is encoded as `""` (two quotes).

Implement the cases in the order shown by the tests:

  1. Plain `a,b,c` and simply quoted `"a","b","c"` (the basic test).
  2. Commas inside quoted fields: `"a,b",c`.
  3. Escaped quotes: `"a""b",c` -> [`a"b`, `c`].

Walk the string character by character with a peekable iterator and keep a small `in_quotes: bool` flag.
When you see `"` while already inside quotes, peek at the next character.
If it's another `"`, push a literal `"` and consume both; otherwise close the field.
Keeping track of which quote you've consumed is the hard part here.
If a test fails, trace its input character by character and write down `in_quotes` after each step.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) is the entry point for character-level iteration.
- [`Iterator::peekable`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable) wraps the iterator so you can look ahead one character.
  You need that lookahead for the `""` -> `"` rule.
- [`Peekable::peek`](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.peek) returns `Option<&Item>` without advancing.
- [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) swaps the current `String` with a fresh empty one in a single move.
  That saves you a copy compared with `current.clone()` followed by `current.clear()`.
- A `match (c, in_quotes)` on the tuple lets you express each state transition as a single arm.
  Add a guard (`if chars.peek() == Some(&'"')`) for the escape rule.
