# Parsing a whole file

With single-line parsing solved, the file-level function is mostly
plumbing: iterate over `content.lines()`, skip blank lines and `#`
comments, and accumulate the rest into a `HashMap`. Stop at the first
malformed line and return an error. Strict parsing makes
configuration bugs obvious instead of silently dropping values.

Each step is self-contained, so the previous step's `parse_env_line`
and `ParseError` are re-declared here with `todo!()` bodies. Re-implement
them (or paste your earlier solution) so this step compiles on its
own.

## Useful from the standard library

- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
  iterates over the lines of the file content, stripping `\n` and
  `\r\n` for you.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
  on each line lets you handle leading/trailing whitespace once.
- [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
  with a `'#'` argument is the comment check. Combine with
  `str::is_empty` to skip blank lines.
- [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)
  fills in each parsed pair. The `?` after `parse_env_line(line)`
  short-circuits on the first malformed line.
- A `for` loop reads more naturally here than an iterator chain
  because the body has both a `continue` skip and a `?` early
  return.
