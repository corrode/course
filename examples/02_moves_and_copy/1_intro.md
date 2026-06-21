# Moves and Copy

Most languages let you keep using a variable after you've assigned it somewhere else.
Rust usually doesn't, and it stops you at compile time.

```rust
let s = String::from("hello");
let t = s;          // ownership moves from s to t
println!("{s}");    // ERROR: borrow of moved value: `s`
```

Assigning `s` to `t` *moves* the string.
There's now one owner, `t`, and `s` is dead.
Reach for `s` again and the compiler refuses, naming the exact line.
In a language with shared mutable pointers this would be a silent bug (two variables aliasing one buffer, one of them freeing it first), and Rust turns it into a compile error.

Why move instead of copy?
A `String` owns a buffer on the heap.
Copying it on every assignment would mean duplicating that buffer over and over, silently.
Rust makes the cheap thing (a move: hand over the pointer) the default and the expensive thing (a deep copy with `.clone()`) something you ask for out loud.

## Copy types

Small values that live entirely on the stack don't have this problem.
Integers, `bool`, `char`, and fixed-size arrays of them implement the `Copy` trait, so assigning one duplicates the bits instead of moving:

```rust
let a = 5;
let b = a;           // a is copied, not moved
println!("{a} {b}"); // both fine
```

The rule of thumb: a type that owns nothing on the heap and is cheap to duplicate is `Copy`, and the move rules never bite it.
Everything else moves.

This is the first chapter where moves matter, because `String` is the first heap-owning type you've met.
Borrowing, using a value without taking it, is the next chapter.
Here we just get comfortable with ownership changing hands.
