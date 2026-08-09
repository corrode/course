# State machines and stateful parsing

Reach for `split(',')` and CSV looks solved in one line.
Then a field contains a comma: the row `"a,b",c` is meant to hold two fields, `a,b` and `c`.
Split on every comma and you get three pieces (`"a`, `b"`, `c`), the quotes still attached and the first field torn in half.

The fix is to stop treating every comma as a separator.
Walk the input one character at a time and track a single piece of state: am I currently inside a quoted field?
A comma inside quotes is data; a comma outside quotes is a separator.

This "for each character, update some state, occasionally emit a result" pattern is called a *state machine*.
It comes up in any non-trivial parsing task: JSON, command-line arguments, terminal escape sequences, markup languages.

## A skeleton

```rust
fn parse(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match (c, in_quotes) {
            ('"', false) => in_quotes = true,
            ('"', true) if chars.peek() == Some(&'"') => {
                // Escaped quote inside a quoted field.
                current.push('"');
                chars.next();
            }
            ('"', true) => in_quotes = false,
            (',', false) => {
                fields.push(std::mem::take(&mut current));
            }
            (c, _) => current.push(c),
        }
    }
    fields.push(current);
    fields
}
```

Each less familiar tool removes one bit of bookkeeping from the loop:

- `peekable()` lets you look at the next character without consuming it.
  That lookahead matters when one character's meaning depends on the one after it, as in the `""` -> `"` rule.
- `match` on a tuple `(c, in_quotes)` lets you express each transition as one arm.
  The alternatives stay flatter than they would with nested `if`/`else` blocks.
- [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) gives you the current string and replaces it with an empty one in a single move.
  The old buffer moves into `fields` without a clone.

## A note on `while let`

You used `if let` with `Option` earlier; `while let` repeats the same pattern until it stops matching.
That extra control matters here because the loop sometimes calls `chars.next()` again to consume the second `"`.

You also used tuple patterns in `let (a, b) = pair`; here, `match` inspects the character and quote state together.
A guard adds the lookahead check only to the escaped-quote arm.

These tests use the raw strings you met in the env-file parser, so the CSV examples can contain literal commas and quotes without an escape forest.

## A useful tactic

When stateful parsing gets hairy, write the simple version first (`split_once`, `split(',')`) and let the easy tests pass.
Then upgrade to the state-machine version for the harder cases.
Failing tests give you concrete examples to think against, instead of trying to imagine every edge case up front.

For real CSV in production code, reach for the [`csv` crate](https://docs.rs/csv); it handles all the corners that this exercise glosses over.
Here we're after the state-machine loop, not a production-ready CSV implementation.
