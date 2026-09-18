# Optional: choose your separator

A spreadsheet export arrives with semicolons instead of commas. Adapt your line
parser into `parse_delimited_line(line, delimiter)` without changing the quoting
rules. A delimiter inside quotes is still data, and doubled quotes still mean one
literal quote. Commas become ordinary data when the delimiter is `;`.

Start from your working quote parser. Keep `parse_csv_line` as a thin wrapper
that chooses a comma; don't maintain two parsing loops.

For this exercise, callers supply a delimiter other than `"`, `\r`, or `\n`.
Inputs have balanced quotes, quotes wrap whole fields, and records occupy one
line. Preserve whitespace, empty fields, and trailing empty fields. An empty
line is one empty field. Rejecting malformed CSV is outside this small challenge.

Before running the tests, predict the fields in `"a;""b";c` with `;` as the
delimiter. Then try a tab or a Unicode delimiter. Does your loop operate on
characters or bytes?
