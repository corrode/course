# Parsing structured text and generics

*You have a problem. You decide to use generics. Now you have a `Problem<T> where T: Clone + Send + Sync + 'static`.*

This chapter parses `.env`-style configuration files. Two new things show
up:

1. **Splitting a string at the first occurrence of a separator.**
2. **A generic function** that works for any type the caller wants to
   parse into.

## Splitting once

`split` returns an iterator of *all* parts. For "key=value" you usually
want to split *once* and keep the rest of the line intact (in case the
value itself contains the separator):

```rust
let line = "DATABASE_URL=postgres://user:pass@host/db";
match line.split_once('=') {
    Some((key, value)) => println!("{key} -> {value}"),
    None => println!("no '='"),
}
```

`split_once` returns `Option<(&str, &str)>`. The two halves are slices of
the original string; no allocation.

## Generic functions

Sometimes you want one function that works for many types. Here, "parse
this string into whatever the caller asks for" is a perfect fit:

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

`<T>` declares a type parameter. The `where T: FromStr` clause says "T
must implement the `FromStr` trait", which is what makes `.parse()` work.
`.parse()` returns `Result<T, T::Err>`; `.ok()` discards the error type
and gives back `Option<T>`, which combines nicely with the `?` on the
preceding line.

## Trim and skip

Real config files have empty lines, comments, and trailing whitespace.
The usual handling chain is:

```rust
for line in content.lines() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        continue;
    }
    // ...parse the line
}
```

`continue` skips the rest of the current loop iteration and jumps to the
next one. Its sibling, `break`, exits the loop entirely.

## A note on raw strings: `r#"..."#`

The tests in this chapter use raw string literals to embed a multi-line
`.env` snippet without escaping anything:

```rust
let content = r#"
HOST=localhost
PORT=5432
"#;
```

A raw string starts with `r` and zero or more `#`s, then a quote. It
ends with the matching closing quote and `#`s. Inside, backslashes and
quotes are literal — no escape sequences. Use more `#`s on each side if
the content itself contains `"#`.

## Useful from the standard library

- [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once)
  returns the first split as `Option<(&str, &str)>`. Cleaner than
  `splitn(2, ...)`.
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
  iterates over the lines of a string, handling `\n` and `\r\n`.
- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
  removes leading and trailing whitespace.
- [`str::is_empty`](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty)
  reads better than `len() == 0`.
- [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
  for prefix checks (skipping `#` comments here).
- [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
  is the trait `.parse::<T>()` uses. Most numeric types, `bool`, and
  many others implement it.
- [`Result::ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok)
  converts a `Result` to an `Option`, dropping the error.
