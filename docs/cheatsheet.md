# Rust cheatsheet

A short, opinionated quick-reference of the syntax this course uses.
Inspired by [cheats.rs](https://cheats.rs/), trimmed to the essentials.
Press `?` from any page to bring this up; `Esc` to close.

## Variables and types

| Syntax | Meaning |
| --- | --- |
| `let x = 42;` | Immutable binding, type inferred. |
| `let mut n: u32 = 0;` | Mutable binding with explicit type. |
| `const PI: f64 = 3.1415;` | Compile-time constant. Must have a type. |
| `let (a, b) = (1, 2);` | Destructure a tuple. |
| `i32`, `u32`, `usize`, `f64`, `bool`, `char` | Common scalar types. `usize` for indices/lengths. |

## Strings

| Syntax | Meaning |
| --- | --- |
| `let s: &str = "hi";` | Borrowed string slice. Cheap to pass around. |
| `let o: String = s.to_string();` | Owned, growable string on the heap. |
| `format!("{s}!")` | Build a `String` from a template (no I/O). |
| `println!("{s} {n}");` | Print to stdout, with a newline. |
| Take `&str` in, return `String` out | Rule of thumb for function signatures. |

## Control flow

| Syntax | Meaning |
| --- | --- |
| `if x > 0 { ... } else { ... }` | `if` is an expression: it returns a value. |
| `for i in 0..10 { ... }` | Half-open range. `0..=10` includes 10. |
| `while cond { ... }` | Loop while the condition holds. |
| `loop { if done { break v; } }` | Infinite loop; `break` can return a value. |
| `match x { 0 => "zero", _ => "n" }` | Pattern match; must be exhaustive. |
| `if let Some(v) = opt { ... }` | Match a single pattern, ignore the rest. |

## Option and Result

| Syntax | Meaning |
| --- | --- |
| `Option<T>` = `Some(T)` \| `None` | A value that may be absent. |
| `Result<T, E>` = `Ok(T)` \| `Err(E)` | A value that may have failed. |
| `x.unwrap_or(0)` | Value if `Some`/`Ok`, otherwise `0`. |
| `x.map(\|v\| v + 1)` | Transform the inner value, leave `None`/`Err` alone. |
| `x?` | Propagate `Err`/`None` to the caller; unwrap on success. |
| `x.unwrap()` | Panic on `None`/`Err`. Use sparingly. |

## Collections

| Syntax | Meaning |
| --- | --- |
| `let v: Vec<i32> = vec![1, 2, 3];` | Heap-allocated, growable array. |
| `v.push(4);` `v.pop();` | Add to / remove from the end. |
| `v.len()`, `v.first()`, `v.contains(&2)` | Length, optional first, membership. |
| `for x in &v { ... }` | Iterate by reference (doesn't consume `v`). |
| `HashMap<K, V>` | Hash map. `use std::collections::HashMap;` |
| `m.insert(k, v);` `m.get(&k);` | Insert; lookup returns `Option<&V>`. |

## Functions and closures

| Syntax | Meaning |
| --- | --- |
| `fn add(a: i32, b: i32) -> i32 { a + b }` | Function. Last expression (no `;`) is the return. |
| `\|x\| x + 1` | Closure with one argument, return inferred. |
| `\|x: i32\| -> i32 { x + 1 }` | Closure with explicit types and a body block. |
| `Fn`, `FnMut`, `FnOnce` | Closure traits, in order of how much they capture. |

## Ownership and borrowing

| Syntax | Meaning |
| --- | --- |
| `let s = String::from("hi");` | `s` owns the heap data. |
| `fn read(s: &String) { ... }` | Shared borrow. Many allowed at once, read-only. |
| `fn modify(s: &mut String) { ... }` | Exclusive borrow. Only one at a time. |
| `fn consume(s: String) { ... }` | Takes ownership; original binding becomes invalid. |
| Rule | At any time: many `&` *or* one `&mut`, never both. |

## Structs, enums, methods

| Syntax | Meaning |
| --- | --- |
| `struct User { name: String, age: u32 }` | Named-field struct. |
| `struct Pair(i32, i32);` | Tuple struct; access with `.0`, `.1`. |
| `impl User { fn name(&self) -> &str { ... } }` | Methods via `impl`. `&self` borrows. |
| `enum Event { Login, Click { x: i32, y: i32 } }` | Sum type with mixed-shape variants. |
| `#[derive(Debug, Clone, PartialEq)]` | Auto-implement common traits. |

## Iterators

| Syntax | Meaning |
| --- | --- |
| `v.iter()` | Iterate by `&T`. Most common. |
| `v.iter_mut()` | Iterate by `&mut T`. |
| `v.into_iter()` | Iterate by `T`, consuming the collection. |
| `.map(\|x\| x * 2)` | Lazy: transform each item. |
| `.filter(\|x\| **x > 0)` | Lazy: keep items where the predicate is true. |
| `.collect::<Vec<_>>()` | Drive a lazy chain into a concrete collection. |
| `.sum()`, `.count()`, `.find(...)`, `.any(...)` | Common eager terminators. |

### What does my closure receive?

`filter` always hands its closure a *reference* to whatever the iterator
yields, which is why `|x|` is so often actually `&T` or `&&T`. Quick
lookup table:

| Source | `.iter()` yields | `.filter` closure sees |
| --- | --- | --- |
| `Vec<i32>` | `&i32` | `&&i32` (use `**x > 0` or `x > &&0`) |
| `Vec<String>` | `&String` | `&&String` (auto-derefs for `.starts_with("a")`) |
| `&[&str]` | `&&str` | `&&&str` (auto-derefs for `.ends_with(".rs")`) |
| `Vec<i32>` after `.into_iter()` | `i32` | `&i32` |

Method calls like `.starts_with`, `.len`, `.contains` auto-dereference,
so `|s| s.starts_with("a")` works no matter how many `&`s are in front.
Direct comparisons (`==`, `<`, `>`) don't, which is why you sometimes
need `**` or `&&` to make the types meet.


## Error handling with `?`

| Syntax | Meaning |
| --- | --- |
| `let n: i32 = s.trim().parse()?;` | On `Err`, return early from the function. |
| `Result<T, Box<dyn Error>>` | "Any error type", common in `main`. |
| `?` requires a matching return type | Function must return `Result` or `Option`. |

## Modules and visibility

| Syntax | Meaning |
| --- | --- |
| `mod math { ... }` | Inline module. Or `mod math;` to load `math.rs`. |
| `use math::add;` | Bring a name into scope. |
| `pub fn ...` | Public to anyone who can see this module. |
| `pub(crate) fn ...` | Public within this crate only. |
| (no keyword) | Private to the current module. |

## Traits and generics (just enough)

| Syntax | Meaning |
| --- | --- |
| `fn parse<T: FromStr>(s: &str) -> Option<T>` | Generic function with a trait bound. |
| `impl Display for User { ... }` | Implement a trait for your type. |
| `Box<dyn Trait>` | Heap-allocated trait object (dynamic dispatch). |

## Cargo commands

| Command | Purpose |
| --- | --- |
| `cargo check` | Fast type-check, no codegen. |
| `cargo build` | Compile in debug mode. |
| `cargo run` | Build and run the default binary. |
| `cargo test` | Run all tests. |
| `cargo test --example NAME` | Run one course exercise's tests. |
| `cargo fmt` | Auto-format with rustfmt. |
| `cargo clippy -- -D warnings` | Lint, fail on warnings. |

## Where to look next

[`std` docs](https://doc.rust-lang.org/std/) ·
[Rust by Example](https://doc.rust-lang.org/rust-by-example/) ·
[The Rust Book](https://doc.rust-lang.org/book/) ·
[cheats.rs](https://cheats.rs/)
