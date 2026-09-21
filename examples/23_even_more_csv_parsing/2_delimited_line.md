# Choose Your Separator

Our little CSV parser is doing well, but one day a spreadsheet export arrives
with semicolons instead of commas.

We get a little nervous as we think about how to change the parser without
breaking the quoting rules.

It takes a little thought, but we can implement a function,
`parse_delimited_line(line, delimiter)`, that supports any delimiter without
changing the quoting rules. A delimiter inside quotes is still data, and doubled
quotes still mean one literal quote. Commas become ordinary data when the
delimiter is `;`.

If you'd like to adapt your earlier quote parser, copy its body into this
editor. But you can also work directly from the starter below.

I suggest keeping `parse_csv_line` as a thin wrapper that chooses a comma as the
separator to preserve our public API. (Imagine that our users rely on this function not to break.)

For this exercise, callers supply a delimiter other than `"`, `\r`, or `\n`.
You can assume that all inputs have balanced quotes, quotes wrap whole fields,
and records occupy one line.

Please preserve whitespace, empty fields, and trailing empty fields. An empty
line is one empty field. Rejecting malformed CSV is outside this small
challenge.

Before running the tests, predict the fields in `"a;""b";c` with `;` as the
delimiter. Then try a tab or a Unicode character as the delimiter instead.
Does your loop operate on characters or bytes?
