# Parsing Structured Text and Generics

*You have a problem. You decide to use generics. Now you have a
`Problem<T> where T: Clone + Send + Sync + 'static`.*

You'll build a parser for `.env`-style configuration files. You'll split each
entry only once, then use a generic function to read values as the types your
callers need.

## Splitting Once

`split` returns an iterator of *all* parts. For "key=value" you usually want to
split *once* and keep the rest of the line intact (in case the value itself
contains the separator):

```rust
let line = "DATABASE_URL=postgres://user:pass@host/db";
match line.split_once('=') {
    Some((key, value)) => println!("{key} -> {value}"),
    None => println!("no '='"),
}
```

`split_once` returns `Option<(&str, &str)>`. The two halves are slices of the
original string, so you don't allocate anything.

## Generic Functions

You could write separate functions to read a port as a `u16` and a flag as a
`bool`. I'd rather write the lookup once and let the caller choose the type.
First, here's a smaller generic helper with no map lookup yet:

```rust
fn parse_value<T: std::str::FromStr>(text: &str) -> Result<T, T::Err> {
    text.parse()
}

assert_eq!(parse_value::<u16>("8080"), Ok(8080));
assert_eq!(parse_value::<bool>("true"), Ok(true));
```

`<T>` declares a type parameter. The `FromStr` bound makes `.parse()` available.
The equivalent `where T: std::str::FromStr` spelling puts the bound after the
parameters. `T::Err` is the error type chosen by that implementation of
`FromStr`. Your lookup exercise will combine this conversion with `HashMap::get`
and decide how to represent a missing key or a failed conversion.

## Trim and Skip

Real config files have empty lines, comments, and trailing whitespace. A small
loop handles all three cases:

```rust
for line in content.lines() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        continue;
    }
    // ...parse the line
}
```

`continue` skips the rest of the current loop iteration and jumps to the next
one. Its sibling, `break`, exits the loop entirely.

## When Errors Mix: `Box<dyn Error>`

The parser uses one error type, the custom `ParseError` enum, so `?` propagates
it cleanly. Real programs often mix error types: read the file from disk and you
get a `std::io::Error`; parse its contents and you get your own `ParseError`. A
function using `?` insists on one error type, so you need something both can
turn into. This is a fiddly part of `?`: when it won't compile, check the error
types as well as the success types.

`Box<T>` puts a value on the heap and owns it. `Box<dyn std::error::Error>` can
hold different error types through a shared interface. An owned error
implementing `std::error::Error + 'static` converts into it automatically. For
this example to compile, you'd first need to implement `Display` and
`std::error::Error` for the exercise's `ParseError`:

```rust
fn load(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;   // io::Error
    let env = parse_env_file(&content)?;            // ParseError
    Ok(env)
}
```

The two `?` lines return different error types, and both convert into
`Box<dyn Error>` on the way out. The box preserves the error's message and
source chain, but callers no longer get a concrete error type to match on
directly. A custom enum (like `ParseError`, but with a variant per source) lets
callers match on each case, and the [`thiserror`](https://docs.rs/thiserror)
crate generates the trait implementations for you. For application code,
[`anyhow`](https://docs.rs/anyhow) offers a similar approach with convenient
error context; for libraries where callers need to match on the error, prefer an
enum.

## A Note on Raw Strings: `r#"..."#`

The tests use raw string literals so you can embed a multi-line `.env` snippet
without escaping anything:

```rust
let content = r#"
HOST=localhost
PORT=5432
"#;
```

A raw string starts with `r` and zero or more `#`s, then a quote. It ends with
the matching closing quote and `#`s. Inside, backslashes and quotes are literal:
no escape sequences. Use more `#`s on each side if the content itself contains
`"#`.


