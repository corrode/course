# Vectors

If you stare at a problem for long enough, it starts turn into a vector.
`Vec<T>` is the workhorse of Rust's collection types.

## Arrays first: where vectors come from

Before we get to `Vec`, it's worth a minute on its older sibling, the
*array*. An array `[T; N]` is a fixed-size, contiguous chunk of
values whose length is part of the type:

```rust
let bytes: [u8; 4] = [10, 20, 30, 40];   // exactly four u8s, forever
```

Because the length is known at compile time, the whole array lives
**on the stack**, the same place your local variables and function
parameters live. Stack storage is essentially free: allocation is
"move the stack pointer by `4 * size_of::<u8>()` bytes," and cleanup
happens automatically when the function returns. The catch is that
you can't grow it. `bytes.push(50)` doesn't compile, because there's
nowhere to grow *into*: the next bytes on the stack already belong
to somebody else.

`Vec<T>` solves that by storing the elements **on the heap** instead.
A `Vec` value is a tiny header on the stack (pointer + length +
capacity) that points at a buffer the allocator hands you. When you
`push` and the buffer fills up, `Vec` asks for a bigger one and
copies the elements over. The header stays the same size; the
buffer behind it grows.

A quick mental model:

| Type        | Where the data lives | Size known at | Can grow? |
|-------------|----------------------|---------------|-----------|
| `[T; N]`    | Stack                | Compile time  | No        |
| `Vec<T>`    | Heap                 | Run time      | Yes       |
| `&[T]`      | Wherever the owner put it (just a pointer + length) | n/a | n/a |

This distinction is one of the things Rust makes you confront that
many languages hide. In Python or Java, *every* list is heap-backed
and you don't get a choice; in C you'd reach for either a fixed-size
array or `malloc` by hand. Rust gives you both, with the same
ownership rules applied to either.

## Vectors: growable, heap-allocated

`Vec<T>` is what you reach for most of the time. The `<T>` is a
generic parameter: it works with any type, but a single `Vec` only
holds one type at a time. So `Vec<i32>` is a vector of 32-bit
integers, `Vec<String>` is a vector of owned strings.

Two ways to create one:

```rust
let mut empty: Vec<i32> = Vec::new();
let with_items = vec![1, 2, 3]; // the vec! macro is the usual way
```

Most operations need a mutable reference. Note the `&mut`:

```rust
let mut list = vec!["bread"];
list.push("milk");          // requires `mut`
let count = list.len();     // borrow without mut
```

A few rules of thumb that will save you trouble:

- Take a slice (`&[T]`) as input when the function only needs to
  *read* the data. This is the vector chapter's version of the
  `&str` rule from chapter 4: `&[i32]` accepts a borrow of a `Vec`
  (`&my_vec` coerces to `&[i32]`), a borrow of an array (`&[1, 2,
  3]`), or a sub-slice of either, all without conversion. A
  parameter typed `&Vec<i32>` would only accept the first one and
  would offer nothing in return.
- Take `&mut Vec<T>` when you need to add or remove items.
- Take `Vec<T>` (no reference) when you actually want to consume the
  vector and take ownership.

Index access (`list[0]`) panics if out of bounds. `list.get(0)` returns
`Option<&T>` instead, which is the safer default.
Use this unless you like panics.


