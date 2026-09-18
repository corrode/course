# State machines and stateful parsing

Reach for `split(',')` and CSV looks solved in one line.
Then a field contains a comma: the row `"a,b",c` is meant to hold two fields, `a,b` and `c`.
Split on every comma and you get three pieces (`"a`, `b"`, `c`), the quotes still attached and the first field torn in half.

The fix is to stop treating every comma as a separator.
Walk the input one character at a time and track a single piece of state: am I currently inside a quoted field?
A comma inside quotes is data; a comma outside quotes is a separator.

This "for each character, update some state, occasionally emit a result" pattern is called a *state machine*.
It comes up in any non-trivial parsing task: JSON, command-line arguments, terminal escape sequences, markup languages.

## A smaller state machine

Before writing CSV, try counting characters outside square brackets.
Assume brackets are balanced and never nested:

```rust
fn count_visible(text: &str) -> usize {
    let mut hidden = false;
    let mut count = 0;
    for c in text.chars() {
        match (c, hidden) {
            ('[', false) => hidden = true,
            (']', true) => hidden = false,
            (_, false) => count += 1,
            (_, true) => {}
        }
    }
    count
}

assert_eq!(count_visible("ab[secret]c"), 3);
```

Trace the state before and after each bracket. CSV needs a similar distinction
between data and syntax, but also needs to collect fields and recognize escaped
quotes. Work out those transitions in the exercise rather than copying this loop.

## Tools for the CSV loop

- `peekable()` lets you inspect the next character without consuming it.
  CSV needs lookahead to distinguish an escaped quote from a closing quote.
- `while let Some(c) = chars.next()` repeats until the iterator is exhausted.
  Unlike a `for` loop, it lets you call `chars.next()` inside the body to consume
  a second character when needed.
- Matching a tuple `(c, in_quotes)` lets an arm consider both character and state.
  A match guard can add a lookahead condition.
- [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) moves out a
  completed `String` and leaves an empty one behind, without cloning its contents.

The tests use raw strings from the env-file chapter so quotes don't need
backslash escapes in the Rust source. CSV itself escapes quotes by doubling them.

## A useful tactic

When stateful parsing gets hairy, write the simple version first (`split_once`, `split(',')`) and let the easy tests pass.
Then upgrade to the state-machine version for the harder cases.
Failing tests give you concrete examples to think against, instead of trying to imagine every edge case up front.

For production code, use the [`csv` crate](https://docs.rs/csv); it handles cases this exercise leaves out, such as quoted fields containing newlines.
This exercise focuses on the state-machine loop rather than a production-ready CSV implementation.
