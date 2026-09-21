# Hints

## `simple_line`

1. There's a method on `&str` that splits on a delimiter and gives you an
   iterator. Combine it with `trim` and `collect`.

## `quoted_line`: The State Machine

Start with the distinction from the bracket example: does this character act as
syntax in the current state? Keep the completed fields separate from the field
you're still reading.

For escapes, compare `"a",c` with `"a""b",c`. At the quote after `a`, what
information tells you whether the field is ending? `peek` borrows the next item
without consuming it. Check which characters your loop has consumed before its
next iteration.

If the last field is missing, trace an input without a trailing comma. What
happens to the unfinished field when the iterator runs out?

## `parse_file`

1. `content.lines()` gives you an iterator over `&str` lines.
2. The first line is headers; the rest are rows. `next()` on the iterator pulls
   the first one off; the rest you can `map(parse_csv_line).collect()`.
