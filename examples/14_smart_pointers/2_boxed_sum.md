# Putting a value on the heap with `Box`

`Box::new(value)` allocates space on the heap, moves `value` into it,
and hands you back a `Box<T>` that owns that allocation. When the
box goes out of scope, Rust drops the inner value and frees the
memory. No `free`, no leaks.

A `Box<T>` *acts* like the value it holds. The `*` operator
dereferences it, and most method calls work through automatic
dereferencing without you writing `*` at all.

```rust
let boxed: Box<i32> = Box::new(7);
let n: i32 = *boxed;            // explicit deref
assert_eq!(n + 1, 8);
```

Why bother boxing a tiny `i32`? You usually wouldn't. `Box` earns
its keep when:

- The value is large and you'd rather not copy it around on the stack.
- The type would otherwise be infinitely sized (next step).
- You need a trait object (the step after that).

For this exercise we keep it simple: take two boxed integers, add
them, return the sum.

## Useful from the standard library

- [`Box::new`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new)
  is the only constructor you need here.
- Deref with `*a` and `*b`, or rely on auto-deref and just write
  `*a + *b`. Both work because `i32` is `Copy`, so reading through
  the box doesn't move anything out.
