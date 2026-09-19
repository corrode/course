# Reading a Boxed Value

[`Box::new(value)`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new) moves a value into a heap allocation and returns its owner, a `Box<T>`.
When the box is dropped, Rust drops the inner value and frees the allocation.

The `*` operator dereferences a box.
For a `Copy` type such as `i32`, reading this way copies the inner value.
Most method calls work through automatic dereferencing instead.

```rust
let boxed: Box<i32> = Box::new(7);
let n: i32 = *boxed;
assert_eq!(n + 1, 8);
```

Implement `boxed_sum` to take ownership of two boxed integers and return their sum as an `i32`.
The tests supply the boxes.
The function does not need to allocate any new ones.

You usually wouldn't box a tiny integer.
This warmup lets you practice reading through a box before using one to own recursive data or a trait object.
