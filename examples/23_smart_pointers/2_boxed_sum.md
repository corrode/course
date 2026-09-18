# Putting a value on the heap with `Box`

[`Box::new(value)`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new) allocates space on the heap, moves `value` into it, and hands you back a `Box<T>` that owns that allocation.
When the box goes out of scope, Rust drops the inner value and frees the memory.

A `Box<T>` *acts* like the value it holds.
The `*` operator dereferences it, and most method calls work through automatic dereferencing without you writing `*` at all.

```rust
let boxed: Box<i32> = Box::new(7);
let n: i32 = *boxed;            // explicit deref
assert_eq!(n + 1, 8);
```

Why bother boxing a tiny `i32`?
You usually wouldn't.
`Box` earns its keep when:

- The value is large and you'd rather not copy it around on the stack.
- The type would otherwise be infinitely sized (next step).
- You need a trait object (the step after that).

For this exercise, take two boxed integers, add them, and return the sum.
The small values let you focus on how to read through a box before you use one in a recursive type.

The tests construct the boxes for you.
Use `*a` and `*b` to read their values: `i32` is `Copy`, so these expressions copy the integers out of the boxes.
