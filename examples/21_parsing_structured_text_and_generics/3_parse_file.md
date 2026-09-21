# Parsing a Whole File

You can now reuse your line parser for a whole file. Return a `HashMap` of its
entries, ignoring blank lines and lines whose first non-whitespace character is
`#`. Stop at the first malformed entry and return its error rather than a
partial map. Strict parsing makes configuration bugs easier to spot.

To keep this page runnable on its own, `ParseError` is re-declared and
`parse_env_line` has a `todo!()` stub. Paste or reimplement your earlier
solution before building the file-level parser.

## Useful from the Standard Library

- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
  iterates over the lines of the file content, stripping `\n` and `\r\n` for
  you.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
  returns a slice without surrounding whitespace.
- [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
  checks a prefix;
  [`str::is_empty`](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty)
  checks whether a string has any content.
- [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)
  stores a key-value pair, replacing the value if the key is already present.

If your parser returns a partial map for `HOST=localhost\nBROKEN\nPORT=8080`,
trace where the line parser's error goes. A malformed entry is not a comment.
