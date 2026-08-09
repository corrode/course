# Wrapping up the CSV parser

The parser begins with `split` and `trim`, then adds state for quoted fields and escaped quotes before collecting headers and rows.

## What we learned

- The same stateful parsing pattern appears in JSON, command lines, and terminal escape sequences.
  In each case, read one item, consult the current state, then update the state or emit a result.
- A peekable iterator lets you inspect what comes next without consuming it, as the `""` -> `"` escape rule requires.
- `match (token, state) { ... }` over a tuple expresses each state transition in one line.
  Match guards (`if cond`) handle the cases where the transition depends on the lookahead.
- `std::mem::take(&mut s)` gives you the current value and replaces it with `Default` in one move.
  Cleaner than clone-then-clear when you're harvesting an accumulator.
- The simple `split`/`trim` version is worth writing first.
  It passes the easy tests and gives you a baseline; the state-machine upgrade then has concrete failing cases to react to.
- Hand production CSV files to the [`csv` crate](https://docs.rs/csv), which handles BOMs, custom delimiters, and escaped newlines inside fields.
  Keep the state-machine loop for parsers you do need to write yourself.
