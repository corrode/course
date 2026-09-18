# Wrapping Up the CSV Parser

You started with `split` and `trim`, then kept track of quotes so commas inside a field stayed where they belonged.
Once that worked, you reused the line parser to collect headers and rows.

## What We Learned

- The same stateful parsing pattern appears in JSON, command lines, and terminal escape sequences.
  In each case, read one item, consult the current state, then update the state or emit a result.
- A peekable iterator lets you inspect what comes next without consuming it, as the `""` -> `"` escape rule requires.
- `match (token, state) { ... }` over a tuple expresses each state transition in one line.
  Match guards (`if cond`) handle the cases where the transition depends on the lookahead.
- `std::mem::take(&mut s)` gives you the current value and replaces it with `Default` in one move.
  You can move the accumulated string into your results without cloning it and then clearing the original.
- The simple `split`/`trim` version is worth writing first.
  It passes the easy tests and gives you a baseline; the state-machine upgrade then has concrete failing cases to react to.
- Hand production CSV files to the [`csv` crate](https://docs.rs/csv), which handles BOMs, custom delimiters, and newlines inside quoted fields.
  Keep the state-machine loop for parsers you do need to write yourself.

Want a little more practice? Try the optional [CSV parser challenges](22_csv_parser_challenges):
choose a delimiter, then put the parser behind a module boundary. These don't count toward course completion.
