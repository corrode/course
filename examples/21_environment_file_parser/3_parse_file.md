# Parsing a whole file

You can now reuse your line parser for a whole file.
Iterate over `content.lines()`, skip blank lines and `#` comments, and collect the remaining entries into a `HashMap`.
Stop at the first malformed line and return an error.
Strict parsing makes configuration bugs obvious instead of silently dropping values.

To keep this page runnable on its own, `parse_env_line` and `ParseError` are re-declared here with `todo!()` bodies.
Paste or reimplement your earlier solution before building the file-level parser.

## Useful from the standard library

- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) iterates over the lines of the file content, stripping `\n` and `\r\n` for you.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim) on each line lets you handle leading/trailing whitespace once.
- [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with) with a `'#'` argument is the comment check.
  Combine with `str::is_empty` to skip blank lines.
- [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert) fills in each parsed pair.
  The `?` after `parse_env_line(line)` short-circuits on the first malformed line.
- I'd use a `for` loop here rather than an iterator chain because the body needs both `continue` to skip lines and `?` to return early.
