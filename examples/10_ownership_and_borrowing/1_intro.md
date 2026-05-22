# Ownership and borrowing

*Why are Rust developers so frugal? They prefer to borrow.*

Ownership is the rule that makes Rust feel different from other
languages. The short version:

1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. You can borrow a value (without taking ownership) by reference.

That's it. The compiler enforces these rules, which is why Rust catches
use-after-free, double-free, and data races at compile time, with no
garbage collector at runtime.

## Move semantics

Assigning or passing a value transfers ownership ("moves" it). After a
move, the original binding is no longer usable:

```rust
let s = String::from("hello");
let t = s;          // ownership moves from s to t
// println!("{s}"); // ERROR: borrow of moved value
println!("{t}");    // fine
```

Types that are cheap to copy (integers, bools, chars, fixed-size arrays
of those) implement `Copy` and don't move. They're duplicated bit-for-bit:

```rust
let a = 5;
let b = a;      // a is copied, not moved
println!("{a} {b}");
```

## Borrowing

Most of the time you don't want to transfer ownership; you just want to
read or modify the value briefly. That's a borrow, written `&value` (or
`&mut value` for a mutable borrow).

```rust
fn length(s: &String) -> usize { s.len() }   // borrow, doesn't take it

let s = String::from("rust");
let n = length(&s);    // borrow s
println!("{s}");       // s still owns the data
```

The borrow checker enforces one rule that takes a moment to internalize:

> At any time, you can have **either** any number of immutable references
> **or** exactly one mutable reference. Never both.

This is what prevents data races at compile time. If the compiler ever
yells at you about borrowing, that rule is the first thing to look at.


