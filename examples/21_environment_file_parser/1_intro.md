# Parsing structured text and generics

*You have a problem. You decide to use generics. Now you have a `Problem<T> where T: Clone + Send + Sync + 'static`.*

The project is a parser for `.env`-style configuration files.
It splits a string only once and uses a generic function to parse the type requested by the caller.

## Splitting once

`split` returns an iterator of *all* parts.
For "key=value" you usually want to split *once* and keep the rest of the line intact (in case the value itself contains the separator):

```rust
let line = "DATABASE_URL=postgres://user:pass@host/db";
match line.split_once('=') {
    Some((key, value)) => println!("{key} -> {value}"),
    None => println!("no '='"),
}
```

`split_once` returns `Option<(&str, &str)>`.
The two halves are slices of the original string; no allocation.

## Generic functions

Sometimes you want one function that works for many types.
Here, "parse this string into whatever the caller asks for" is a perfect fit:

```rust
fn get<T>(env: &HashMap<String, String>, key: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    env.get(key)?.parse().ok()
}

let port: Option<u16> = get(&env, "PORT");
let debug: Option<bool> = get(&env, "DEBUG");
```

`<T>` declares a type parameter.
The `where T: FromStr` clause says "T must implement the `FromStr` trait", which is what makes `.parse()` work.
`.parse()` returns `Result<T, T::Err>`; `.ok()` discards the error type and gives back `Option<T>`, which combines nicely with the `?` on the preceding line.

## Trim and skip

Real config files have empty lines, comments, and trailing whitespace.
A small loop handles all three cases:

```rust
for line in content.lines() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        continue;
    }
    // ...parse the line
}
```

`continue` skips the rest of the current loop iteration and jumps to the next one.
Its sibling, `break`, exits the loop entirely.

## When errors mix: `Box<dyn Error>`

The parser uses one error type, the custom `ParseError` enum, so `?` propagates it cleanly.
Real programs often mix error types: read the file from disk and you get a `std::io::Error`; parse its contents and you get your own `ParseError`.
A function using `?` insists on one error type, so you need something both can turn into.

`Box<T>` puts a value on the heap and owns it.
`Box<dyn std::error::Error>` is a return type that accepts almost any error, because nearly every error type converts into it automatically:

```rust
fn load(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;   // io::Error
    let env = parse_env_file(&content)?;            // ParseError
    Ok(env)
}
```

The two `?` lines return different error types, and both convert into `Box<dyn Error>` on the way out.
That's the quick, lossy option: you give up the specific type and keep only "some error happened."
A custom enum (like the `ParseError` you're about to write, but with a variant per source) keeps the type information at the cost of more code, and the [`thiserror`](https://docs.rs/thiserror) crate generates most of that boilerplate for you.
For quick programs, [`anyhow`](https://docs.rs/anyhow) wraps the `Box<dyn Error>` approach with nicer ergonomics; for libraries where callers need to match on the error, prefer an enum.

## A note on raw strings: `r#"..."#`

The tests use raw string literals so you can embed a multi-line `.env` snippet without escaping anything:

```rust
let content = r#"
HOST=localhost
PORT=5432
"#;
```

A raw string starts with `r` and zero or more `#`s, then a quote.
It ends with the matching closing quote and `#`s.
Inside, backslashes and quotes are literal: no escape sequences.
Use more `#`s on each side if the content itself contains `"#`.


