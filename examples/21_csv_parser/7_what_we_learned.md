# Wrapping up the CSV parser

You wrote the easy version of CSV with `split` and `trim`, then
upgraded to a real state machine that handles quoted fields and
escaped quotes, glued lines into headers + rows, and converted
those rows into `HashMap` records.

## What we learned

- Stateful parsing comes up everywhere CSV doesn't (JSON, command
  lines, terminal escape sequences). The routine is always: walk the
  input character by character, keep a small flag (or enum) of
  current state, occasionally emit a result.
- A peekable iterator is the standard tool for "what comes next?"
  decisions like the `""` -> `"` escape rule. `Iterator::peekable`
  costs nothing in practice.
- `match (token, state) { ... }` over a tuple expresses each state
  transition in one line. Match guards (`if cond`) handle the cases
  where the transition depends on the lookahead.
- `std::mem::take(&mut s)` gives you the current value and replaces
  it with `Default` in one move. Cleaner than clone-then-clear when
  you're harvesting an accumulator.
- The simple `split`/`trim` version is worth writing first. It
  passes the easy tests and gives you a baseline; the state-machine
  upgrade then has concrete failing cases to react to.
- `iter.zip(other).collect::<HashMap<_, _>>()` is the standard "two
  parallel sequences -> a map" move. Add `cloned()` on each side
  when the map needs owned data.
- For real CSV in production, reach for the
  [`csv` crate](https://docs.rs/csv): it handles all the corner
  cases (BOMs, custom delimiters, escaped newlines inside fields)
  this exercise glosses over. Writing the parser by hand once is
  still worth doing for the transferable state-machine technique.
