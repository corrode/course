# A Module for Our Parser

We've spent enough time worrying about commas. Let's give our users a function
that parses a whole file, so they don't have to worry about individual lines.

I've put a working `parse_line` inside `mod csv { ... }` below. You can use it
as is, even if you skipped the previous exercise. Your job is to implement
`parse_file`: parse the first line as headers and the remaining lines as rows,
then return both. Call `parse_line` for each line, including the headers.
No need to write that parser again!

Keep the same quoting rules as before. We still assume valid input and don't
support newlines inside quoted fields. A few details to watch out for:

- Empty input gives us empty headers and rows.
- A blank line between records counts as a row with one empty field.
- A trailing comma leaves an empty field, but a final newline adds no extra row.

I suggest using `str::lines()` to walk through the input. It handles both Unix
and Windows line endings.

Notice that `parse_file` has `pub` in front of it, but `parse_line` doesn't.
That's deliberate: callers use `csv::parse_file`, while `parse_line` stays
private. This gives us room to change how we parse lines later without breaking
anyone else's code.

Once the tests pass, let's see what Rust lets us get away with:

1. Remove `pub` from `parse_file`. Will the tests still compile? Try it, read
   the error, then put `pub` back.
2. Call `csv::parse_line("a,b")` from a test. Why can `parse_file` call it,
   but the test can't? Remove the call when you're done.

The tests live outside `mod csv`, so they follow the same rules as any other
caller. Being in the same file doesn't give them access to private functions.

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
