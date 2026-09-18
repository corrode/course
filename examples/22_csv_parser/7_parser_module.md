# Optional: give the parser a module boundary

Return to the modules lesson with something worth hiding. This page includes your
comma-line parser inside an inline `csv` module, so it runs in the browser without
extra files. Don't paste another copy into the file parser.

Implement `csv::parse_file` by calling the existing `parse_line` for both the
headers and every data row. Expose only `parse_file`; leave the line parser
private. Empty input returns empty headers and rows. A trailing newline adds no
row, and quoted newlines remain outside our supported format.

The tests sit outside `csv`, like application code. First finish the function,
then make the smallest visibility change needed to call it from there.
Once they pass, temporarily call `csv::parse_line("a,b")` from a test.
Predict whether it compiles, try it, then remove the call. Why is it fine for
`parse_file` to call the same helper?

If you're working locally, you can move the module body into `csv.rs` and replace
the inline module with `mod csv;`. Keep the tests in the original file. This is
only a file-layout change: callers should still use `csv::parse_file`.
