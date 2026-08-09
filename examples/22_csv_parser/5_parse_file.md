# Parsing a whole file

With a working line parser, the file-level parser is mostly plumbing: split on newlines, treat the first line as headers, and parse the rest as data rows.

Use [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) to split: it handles trailing newlines gracefully, so `"a,b\n"` gives one line, not two.

You'll reuse `parse_csv_line` from the previous page.
To keep this page independently runnable, its signature is re-declared here as a stub with `todo!()`.
Paste your earlier solution into the stub or call into it.

## Useful from the standard library

- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) yields each line as a `&str`, stripping `\n` and `\r\n`.
  A trailing newline does not create an empty trailing line.
- [`Iterator::next`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.next) on the iterator pulls off the header line; an empty file should return empty headers and rows.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) + `parse_csv_line` over the remaining lines builds the rows.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) to materialize both the headers and the rows into `Vec`s.
