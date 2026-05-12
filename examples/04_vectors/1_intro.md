# Vectors

If you stare at a problem for long enough, it starts turn into a vector.
`Vec<T>` is the workhorse of Rust's collection types.

Vectors in Rust are growable, heap-allocated arrays. The `<T>` in `Vec<T>` is a generic
parameter: it works with any type, but a single `Vec` only holds one
type at a time. So `Vec<i32>` is a vector of 32-bit integers, `Vec<String>`
is a vector of owned strings.

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

- Take a slice (`&[T]`) as input when the function only needs to *read*
  the data. It accepts both `&Vec<T>` and arrays.
- Take `&mut Vec<T>` when you need to add or remove items.
- Take `Vec<T>` (no reference) when you actually want to consume the
  vector and take ownership.

Index access (`list[0]`) panics if out of bounds. `list.get(0)` returns
`Option<&T>` instead, which is the safer default.
Use this unless you like panics.

## Useful from the standard library

- [`Vec::new`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
  and the [`vec!`](https://doc.rust-lang.org/std/macro.vec.html) macro
  create vectors. `vec!` is more common in code.
- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
  appends one item. [`Vec::pop`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop)
  removes the last one and returns `Option<T>`.
- [`Vec::len`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len)
  is the item count. [`Vec::is_empty`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty)
  reads better than `len() == 0`.
- [`<[T]>::contains`](https://doc.rust-lang.org/std/primitive.slice.html#method.contains)
  checks for membership: `list.contains(&"apple")`. The `&` matters; it
  compares by reference.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
  turns any iterator back into a `Vec` (or other collection). You'll use
  it constantly once chapter 11 introduces iterators.
