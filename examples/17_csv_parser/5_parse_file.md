# Parsing a whole file

With a working line parser, the file-level parser is mostly
plumbing: split on newlines, treat the first line as headers, and
parse the rest as data rows.

Use [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
to split — it handles trailing newlines gracefully, so `"a,b\n"`
gives one line, not two.

This step composes on top of `parse_csv_line` from the previous
step. To keep each step independently runnable, the signature is
re-declared here as a stub with `todo!()` — replace it with your
solution from step 4 (or just call into it).
