# Moves and Copy

Most languages let you keep using a variable after you've assigned it somewhere else.
Rust usually doesn't, and it stops you at compile time.

```rust
let s = String::from("hello");
let t = s;          // ownership moves from s to t
println!("{s}");    // ERROR: borrow of moved value: `s`
```

Assigning `s` to `t` *moves* the string.
There's now one owner, `t`, and `s` no longer names a value you can use.
Reach for `s` again and the compiler points to the exact move that made it unavailable.
With manual memory management, keeping two pointers to the same buffer can cause a use-after-free if one frees the buffer while the other still uses it.

Why move instead of copy?
A `String` owns a buffer on the heap.
Copying it on every assignment would mean duplicating that buffer over and over, silently.
Rust makes the cheap thing the default: move the `String` without copying its heap buffer.
To duplicate the string's buffer, call `.clone()` explicitly.

## Copy types

Types that can safely be duplicated bit-for-bit can implement `Copy`.
Integers, `bool`, `char`, and fixed-size arrays of them implement the `Copy` trait, so assigning one duplicates the bits instead of moving:

```rust
let a = 5;
let b = a;           // a is copied, not moved
println!("{a} {b}"); // both fine
```

Integers and `bool` are `Copy`, while heap-owning values such as `String` move by default.
A type's documentation tells you whether it implements `Copy` when the distinction is not obvious.

`String` makes moves visible because it owns a heap allocation.
These exercises trace how ownership changes hands.
Borrowing lets a function use a value while its caller keeps ownership.
