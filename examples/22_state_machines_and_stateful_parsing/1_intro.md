# State Machines and Stateful Parsing

CSV, or Comma-Separated Values, is a wonderfully simple file format:

```csv
Frank, 42, Engineer
Hannah, 24, Teacher
Jake, 35, Doctor
```

Each line is a record and each comma separates fields: name, age, and occupation.
Just split by commas and you are d... 

```csv
Joe, 30, Artist, Painter, Writer
```

Oh no...

## The Problem with CSV

Contrary to popular belief, CSV is a mischievous pain in the neck. It lures you
into thinking it's just a list of comma-separated fields, but the moment you
reach for `split(',')`, it will rear its ugly head. For example, a row `"a,b",c`
is meant to hold two fields, `a,b` and `c`. Split on every comma and you get
three pieces (`"a`, `b"`, `c`), the quotes still attached and the first field
torn in half.

From [csv-spec.org](https://csv-spec.org/):

> CSV is not a file format, it is a loose set of guidelines of how to structure
> tabular data into a plain text string. As such there’s an endless amount of
> *.csv files floating around which are highly incompatible with each other. The
> closest thing there is to a specification is RFC 4180.

This should tell you everything you need to know about CSV.
And yet! We will attempt to tame this beast and write a parser that can handle most of the common cases.

## Taming CSV

The trick is to stop treating every comma as a separator. Walk the input one
character at a time and track a single piece of state: "Am I currently inside a
quoted field?" A comma inside quotes is data; a comma outside quotes is a
separator.

This "for each character, update some state, occasionally emit a result" pattern
is called a *state machine*. It comes up in any non-trivial parsing task: JSON,
command-line arguments, terminal escape sequences, markup languages, the legacy
data format used by your favorite spreadsheet program.

## A Smaller State Machine

Before writing CSV, consider the following tiny state machine, where we are counting characters outside square brackets.
The input is something like "ab[secret]c" and the output is 3, because there are three characters outside the brackets: `a`, `b`, and `c`. The brackets themselves are not counted, and neither are the characters inside them. (We assume brackets are balanced and never nested.)

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

Here, we track the state before and after each bracket.

Our CSV parser needs a similar distinction between "data" and "syntax", but on top of that needs to
collect fields and recognize escaped quotes.

## Useful tools for CSV parsing

- `peekable()` lets you inspect the next character without consuming it. CSV
  needs lookahead to distinguish an escaped quote from a closing quote.
- `while let Some(c) = chars.next()` repeats until the iterator is exhausted.
  Unlike a `for` loop, it lets you call `chars.next()` inside the body to
  consume a second character when needed.
- Matching a tuple `(c, in_quotes)` lets an arm consider both character and
  state. A match guard can add a lookahead condition.
- [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) moves out a
  completed `String` and leaves an empty one behind, without cloning its
  contents.

The tests use raw strings so quotes don't need backslash escapes in the Rust
source. CSV itself escapes quotes by doubling them.

## A Useful Tactic

When stateful parsing gets hairy, write the simple version first (`split_once`,
`split(',')`) and let the easy tests pass. Then upgrade to the state-machine
version for the harder cases. Failing tests give you concrete examples to think
against, instead of trying to imagine every edge case up front.

> [!NOTE]
>
> This parser is practice for the state-machine loop, not a complete CSV
> implementation. For production code, use the [`csv`
> crate](https://docs.rs/csv), which handles cases we leave out, such as quoted
> fields containing newlines.
