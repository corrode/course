# Rust cheatsheet

A short, opinionated quick-reference. Inspired by [cheats.rs](https://cheats.rs/),
trimmed down to the syntax and types this course actually uses. Keep this
tab open while you work through the exercises.

## Variables and types

```rust
let x = 42;             // immutable, type inferred
let mut n: u32 = 0;     // mutable, explicit type
const PI: f64 = 3.1415; // compile-time constant
let (a, b) = (1, 2);    // destructuring
```

Common scalar types: `i32`, `i64`, `u32`, `u64`, `usize`, `f64`, `bool`, `char`.

## Strings

```rust
let s: &str = "hello";          // borrowed string slice
let owned: String = s.to_string();
let combined = format!("{s}!"); // returns String
println!("{s} {n}");             // print to stdout
```

Rule of thumb: take `&str` as input, return `String` when the caller
needs to keep the result.

## Collections

```rust
let v: Vec<i32> = vec![1, 2, 3];
v.len(); v.first(); v.contains(&2);
v.push(4);
for x in &v { /* ... */ }

use std::collections::HashMap;
let mut m: HashMap<String, u32> = HashMap::new();
m.insert("a".into(), 1);
m.get("a"); // Option<&u32>
```

## Control flow

```rust
if x > 0 { ... } else if x == 0 { ... } else { ... }

for i in 0..10 { /* 0..=10 includes 10 */ }
while cond { ... }
loop { if done { break value; } }

let label = match score {
    0..30  => "weak",
    30..70 => "medium",
    _      => "strong",
};
```

## Option and Result

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }

x.unwrap_or(0);          // value or default
x.map(|v| v + 1);        // transform Some / Ok
x.unwrap();              // panic on None / Err — sparingly
x?;                      // propagate Err / None up

if let Some(v) = x { ... }
while let Some(v) = iter.next() { ... }
```

## Functions and closures

```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn shout(s: &str) -> String { s.to_uppercase() }

let inc = |x| x + 1;
let greet = |name: &str| { format!("hi, {name}") };
v.iter().map(|x| x * 2).filter(|x| *x > 0).collect::<Vec<_>>();
```

## Structs, enums, methods

```rust
#[derive(Debug, Clone, PartialEq)]
struct User { name: String, verified: bool }

impl User {
    fn new(name: String) -> Self { Self { name, verified: false } }
    fn verify(&mut self)         { self.verified = true; }
    fn name(&self) -> &str       { &self.name }
}

enum Event { Login, Click { x: i32, y: i32 }, Quit }
```

## Ownership in one paragraph

Each value has one owner. Passing it to a function moves it (the original
binding is gone). To let someone *look* at it without taking it, pass a
reference: `&value` (shared) or `&mut value` (exclusive). At any time,
either many `&` or one `&mut`, never both.

```rust
fn read(s: &String)      { /* borrow */ }
fn modify(s: &mut String){ s.push('!'); }
fn consume(s: String)    { /* takes ownership */ }
```

## Iterators

```rust
v.iter()            // &T
v.iter_mut()        // &mut T
v.into_iter()       // T (consumes the Vec)

v.iter().sum::<i32>();
v.iter().count();
v.iter().any(|x| *x > 0);
v.iter().find(|x| **x == 7);
v.iter().filter(|x| **x > 0).cloned().collect::<Vec<_>>();
```

## Error handling with `?`

```rust
fn read_number(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let n: i32 = text.trim().parse()?;
    Ok(n)
}
```

## Modules and visibility

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}
use math::add;

// pub        — visible everywhere this module is in scope
// pub(crate) — visible inside this crate
// (none)     — visible only inside this module
```

## Traits and generics (just enough)

```rust
fn parse<T: std::str::FromStr>(s: &str) -> Option<T> {
    s.parse().ok()
}

let port: Option<u16> = parse("8080");
```

## Useful command-line incantations

```sh
cargo new my-project
cargo check                 # fast type-check
cargo test                  # run tests
cargo test --example NAME   # run one course exercise
cargo fmt
cargo clippy -- -D warnings
```

## Where to look next

- [`std` documentation](https://doc.rust-lang.org/std/) — the source of truth.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — short, runnable snippets.
- [The Rust Book](https://doc.rust-lang.org/book/) — the long version.
- [cheats.rs](https://cheats.rs/) — the original, much more exhaustive cheat sheet.
