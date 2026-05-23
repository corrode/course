# State Machines and Stateful Parsing

CSV looks easy until you hit quoted fields with commas inside, or
escaped quotes inside quoted fields. The trick is to walk the input
character-by-character while tracking a small amount of state: "am I
currently inside a quoted field?"

This kind of "for each character, update some state, occasionally emit a
result" pattern is called a *state machine*. It comes up in any non-trivial
parsing task: JSON, command-line arguments, terminal escape sequences,
markup languages.

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

Two things worth pointing out:

- `peekable()` lets you look at the next character without consuming it.
  Essential when one character's meaning depends on the one after it (the
  `""` -> `"` rule here).
- `match` on a tuple `(c, in_quotes)` lets you express each transition as
  one arm. Easier to read than nested `if`/`else`.
- [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html)
  gives you the current string and replaces it with an empty one in a
  single move. No clone, no temporary.

## A note on `while let`

`while let Some(c) = chars.next() { ... }` is the loop counterpart of
`if let` from chapter 9. It keeps running as long as the pattern
matches, and stops as soon as it doesn't. Iterators return `None` at
the end, so `while let Some(...)` is a natural fit when you need more
control than a `for` loop gives you (here we want to call
`chars.next()` again *inside* the loop body to consume the second `"`).

Tuple matching like `match (c, in_quotes) { ... }` is the same idea as
chapter 8's `let (a, b) = pair`, just used as a `match` scrutinee. The
arms then pattern-match both elements at once, and the guards (chapter
10's `if chars.peek() == Some(&'"')`) do the rest.

The tests in this chapter also lean heavily on raw strings
(`r#"..."#`, introduced in chapter 20) so the CSV examples can contain
literal commas and quotes without an escape forest.

## A useful tactic

When stateful parsing gets hairy, write the simple version first
(`split_once`, `split(',')`) and let the easy tests pass. Then upgrade
to the state-machine version for the harder cases. Failing tests give you
concrete examples to think against, instead of trying to imagine every
edge case up front.

For real CSV in production code, reach for the
[`csv` crate](https://docs.rs/csv); it handles all the corners that this
exercise glosses over. But knowing how to write a state machine yourself
is a transferable skill.
