# Optional: Give the Parser a Module Boundary

Return to the modules lesson with something worth hiding. This page includes your
comma-line parser inside an inline `csv` module, so it runs in the browser without
extra files. Don't paste another copy into the file parser.

Implement `csv::parse_file` by calling the existing `parse_line` for both the
headers and every data row. Expose only `parse_file`; leave the line parser
private. Empty input returns empty headers and rows. A trailing newline adds no
row, and quoted newlines remain outside our supported format.

The tests sit outside `csv`, like application code. The starter exposes
`parse_file`, so it compiles before you implement the body.

Once the tests pass, try two optional visibility experiments, one at a time:

1. Remove `pub` from `parse_file`. Predict the error, compile, then restore `pub`.
2. Call `csv::parse_line("a,b")` from a test. Predict whether it compiles, try it,
   then remove the call. Why can `parse_file` call this private helper?

Restore the working version after each experiment so other tests can compile.

For a local multi-file experiment, move the contents of `mod csv { ... }`
(without the outer braces) into `csv.rs` beside `3_parser_module.rs`.
Replace the inline module with:

```rust
#[path = "csv.rs"]
mod csv;
```

Keep the tests in `3_parser_module.rs`. The explicit path finds the sibling file
both when this step is compiled alone and when the chapter's generated `main.rs`
includes it as a module. Don't edit the generated aggregator. Callers still use
`csv::parse_file`; only the file layout changes. Keep the inline version in the
browser, where the editor submits a single source file.
