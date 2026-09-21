# A Module for Our Parser

This step deliberately reuses the whole-file parser from the state-machine
chapter. The new work is experimenting with its module boundary, not inventing
another parsing loop.

I've put a working `parse_line` inside `mod csv { ... }` below. You can use it
as is, even if you skipped the delimiter exercise. Fill in `parse_file` by
reusing your earlier file parser and adjusting the helper name to `parse_line`.
If you haven't written that step, parse the first line as headers and the rest
as rows, then return both. Use the supplied line parser for headers and rows.

Keep the same quoting rules as before. We still assume valid input and don't
support newlines inside quoted fields. A few details to watch out for:

- Empty input gives us empty headers and rows.
- A blank line between records counts as a row with one empty field.
- A trailing comma leaves an empty field, but a final newline adds no extra row.

I suggest using `str::lines()` to walk through the input. It handles both Unix
and Windows line endings.

## Try the Module Boundary

The tests live outside `mod csv`. Once they pass, try these changes one at a
time. Before each run, predict which call the compiler will accept or reject.

1. Remove `pub` from `parse_file`. Run the tests, inspect the error, then put
   `pub` back.
2. Call `csv::parse_line("a,b")` from a test. Compare that call with the one
   inside `parse_file`, then remove the added call.

Which parts of the parser do callers need to see? Keeping the line helper
private lets you change it without changing the public API. Being in the same
file doesn't give the outer tests access to private items inside `csv`.

## What About a Separate File?

If you're working locally, try moving the code inside `mod csv { ... }` into
`csv.rs`, next to `3_parser_module.rs`. Leave the tests where they are, and
replace the module block with:

```rust
#[path = "csv.rs"]
mod csv;
```

The path tells Rust where to find our file, including when you run the exercise
through the chapter's generated `main.rs`. You don't need to edit `main.rs`.
Callers still use `csv::parse_file`, just as before.

If you're using the course's browser editor, keep everything in one file:
it only sends that file to the playground.
