# Quotes, embedded commas, and escapes

Real CSV is a state machine in disguise. A field can be wrapped in
double quotes, in which case any commas *inside* the quotes are part
of the field, not separators. And a literal `"` inside a quoted
field is encoded as `""` (two quotes).

Suggested order of attack:

  1. Plain `a,b,c` and simply quoted `"a","b","c"` (the basic test).
  2. Commas inside quoted fields: `"a,b",c`.
  3. Escaped quotes: `"a""b",c` -> [`a"b`, `c`].

Walk the string character by character with a peekable iterator and
keep a small `in_quotes: bool` flag. When you see `"` while already
inside quotes, peek the next char: if it's another `"`, push a
literal `"` and consume both; otherwise close the field.
