# Hints

## `simple_line`

1. There's a method on `&str` that splits on a delimiter and gives you an iterator.
   Combine it with `trim` and `collect`.

## `quoted_line`: the state machine

1. A single `bool` (`in_quotes`) is enough state.
   Walk the input with `line.chars().peekable()` so you can look one character ahead.
2. Match on the tuple `(c, in_quotes)`.
   Handle these five cases:
   - `('"', false)` → enter quoted mode.
   - `('"', true)` and the next char is also `"` → push a literal `"`, consume the second one with `chars.next()`.
   - `('"', true)` → exit quoted mode.
   - `(',', false)` → finish the current field, start a new one.
   - anything else → push the character into the current field.
3. After the loop, push the final field.
   Use `std::mem::take(&mut current)` to move a completed field into the results without cloning.
4. If you're still stuck, use the skeleton in the chapter intro as the body of `parse_csv_line` and run the tests.

## `parse_file`

1. `content.lines()` gives you an iterator over `&str` lines.
2. The first line is headers; the rest are rows.
   `next()` on the iterator pulls the first one off; the rest you can `map(parse_csv_line).collect()`.
