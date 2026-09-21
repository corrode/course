# Optional: Give the Parser a Module Boundary

Rust checks the parser's module boundary at compile time, even when the caller
and helper live in the same file. This page includes a working comma-line parser
inside an inline `csv` module, so it runs in the browser without extra files.
Use it as supplied; this task doesn't depend on your delimiter parser.

Implement `csv::parse_file` by calling the existing `parse_line` for both the
headers and every data row. Don't paste another copy into the file parser.
Expose only `parse_file`; leave the line parser private. Empty input returns
empty headers and rows. Keep blank interior lines as rows with a single empty
field, and preserve trailing empty fields within a row. A final newline adds no
extra record. The line parser's quoting rules still apply; malformed quoting and
quoted newlines remain outside our supported format.

The tests sit outside `csv`, like application code. The starter exposes
`parse_file`, so it compiles before you implement the body.

Once the tests pass, check what this API lets callers access with two visibility
experiments, one at a time.

1. Remove `pub` from `parse_file`. Predict what happens at the calls in the
   tests, then compile and inspect the caller's privacy error. Restore `pub` and
   run the tests again.
2. Add a call to `csv::parse_line("a,b")` in a test outside `csv`. Compile and
   inspect the privacy error for the helper, then remove the call and run the
   tests again. Why can `parse_file` call this private helper while the test
   cannot?

These errors are temporary experiments; finish with both the public API and the
tests working.

For an optional local multi-file experiment, move the contents of
`mod csv { ... }` (without the outer braces) into `csv.rs` beside
`3_parser_module.rs`. Replace the inline module with:

```rust
#[path = "csv.rs"]
mod csv;
```

Keep the tests in `3_parser_module.rs`. The explicit path finds the sibling file
both when this step is compiled alone and when the chapter's generated `main.rs`
includes it as a module. Don't edit the generated aggregator. Callers still use
`csv::parse_file`; only the file layout changes. Keep the inline version in the
browser, where the editor submits a single source file.
