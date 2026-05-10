# State machines and stateful parsing

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

## A useful tactic

When stateful parsing gets hairy, write the simple version first
(`split_once`, `split(',')`) and let the easy tests pass. Then upgrade
to the state-machine version for the harder cases. Failing tests give you
concrete examples to think against, instead of trying to imagine every
edge case up front.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  is the entry point for character-level iteration.
- [`Iterator::peekable`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable)
  wraps any iterator so you can `.peek()` at the next item.
- [`Peekable::peek`](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.peek)
  returns `Option<&Item>` without advancing.
- [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html)
  swaps a value with its `Default`. Used here to "harvest" the current
  field and start a fresh one.
- [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split)
  is the simple-case answer for the warm-up function.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
  for cleaning up whitespace around fields.

For real CSV in production code, reach for the
[`csv` crate](https://docs.rs/csv); it handles all the corners that this
exercise glosses over. But knowing how to write a state machine yourself
is a transferable skill.
