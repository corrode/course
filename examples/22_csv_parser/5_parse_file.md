# Parsing a Whole File

Your line parser has done the harder work. For the whole file, split on
newlines, treat the first line as headers, and parse the rest as data rows.

Assume each record occupies one line; this parser won't handle newlines inside
quoted fields.

You'll reuse `parse_csv_line` from the previous page. To keep this page
independently runnable, its signature is re-declared here as a stub with
`todo!()`. Paste your earlier solution into the stub.

## Useful from the Standard Library

- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
  yields each line as a `&str`, stripping `\n` and `\r\n`. A trailing newline
  does not create an empty trailing line.
- [`Iterator::next`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.next)
  on the iterator pulls off the header line; an empty file should return empty
  headers and rows.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) +
  `parse_csv_line` over the remaining lines builds the rows.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
  gathers the parsed rows into a `Vec<Vec<String>>`. `parse_csv_line` already
  returns the headers as a `Vec<String>`.
