# Borrowing and references

*Why are Rust developers so frugal? They prefer to borrow.*

The functions chapter had a nuisance baked in.
Pass a `String` to a function and the move rules say you've handed it over.
Want to use it afterwards? Too bad, it's gone.
Returning it back out works but gets tedious fast, especially when the function only needed to *read* the value.

Borrowing is the fix.
A borrow lets a function use a value without taking ownership of it: `&value` for a shared, read-only borrow, and `&mut value` for an exclusive, writable one.

```rust
fn length(s: &String) -> usize { s.len() }   // borrows, doesn't take

let s = String::from("rust");
let n = length(&s);    // lend s out for the call
println!("{s}");       // s still owns the data
```

The caller keeps ownership the whole time.
The function gets temporary access and gives it back when it returns.

## The one rule

Borrows come with a single rule, and the borrow checker enforces it everywhere:

> At any moment you can have either any number of shared `&` references, or exactly one `&mut` reference. Never both at once.

A shared reference promises the data won't change while you're looking at it.
A mutable reference promises nobody else is looking while you write to it.
Allow both at once and you'd have someone reading a value halfway through someone else's change, which is the classic data race.
Rust rules that out at compile time instead of trusting you to get the locking right.

When the compiler rejects your code with a borrowing error, this rule is the first thing to check.
