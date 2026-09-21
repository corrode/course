# Vectors

A `[u8; 4]` has room for exactly four bytes; that's it.

When the number of items can vary, use a `Vec<T>` instead. It stores a sequence
of values that can grow or shrink, without asking you to predict the final
length.

## Arrays First: Where Vectors Come From

To see why a `Vec` can grow, compare it with an *array*. An array `[T; N]` is a
fixed-size, contiguous chunk of values whose length is part of the type:

```rust
let bytes: [u8; 4] = [10, 20, 30, 40];   // exactly four u8s, forever
```

Because this array is a local variable with a compile-time length, its elements
can live directly **on the stack** alongside the function's other local data.
Setting aside that stack space is cheap, and Rust reclaims it automatically when
the function returns. The catch is that you can't grow it. `bytes.push(50)`
doesn't compile because arrays have no `push` method: `[u8; 4]` holds exactly
four elements, wherever it is stored.

`Vec<T>` solves that by storing the elements **on the heap** instead. The local
`Vec` value is a small header containing a pointer, a length, and a capacity,
while the allocator provides the buffer it points to. When you `push` and the
buffer fills up, `Vec` requests more space and may move the elements to a new
allocation. The header stays the same size; the buffer behind it grows.

A quick mental model:

| Type        | Where the data lives | Size known at | Can grow? |
|-------------|----------------------|---------------|-----------|
| `[T; N]`    | Inline in its owner (often on the stack) | Compile time | No |
| `Vec<T>`    | Heap                 | Run time      | Yes       |
| `&[T]`      | Wherever the owner put it (just a pointer + length) | n/a | n/a |

If you're coming from Python or Java, `Vec<T>` is the closer match for the lists
you use every day. In C, the same choice is closer to picking a fixed-size array
or managing an allocation yourself. Rust gives you both choices, and its
ownership rules apply to either one.

## Vectors: Growable, Heap-Allocated

The `<T>` is a generic parameter: it works with any type, but a single `Vec`
only holds one type at a time. So `Vec<i32>` is a vector of 32-bit integers,
`Vec<String>` is a vector of owned strings.

You can start with an empty vector or with its initial items:

```rust
let mut empty: Vec<i32> = Vec::new();
let with_items = vec![1, 2, 3]; // vec! starts with these three items
```

Changing a vector requires mutable access, while reading it only needs a shared
borrow:

```rust
let mut list = vec!["bread"];
list.push("milk");          // requires `mut`
let count = list.len();     // borrow without mut
```

When you choose a parameter type, start from what the function needs to do:

- Take a slice (`&[T]`) as input when the function only needs to *read* the
  data. This follows the same rule as `&str` in function parameters. A `&[i32]`
  parameter accepts a borrow of a `Vec` (`&my_vec` coerces to `&[i32]`), a
  borrow of an array (`&[1, 2, 3]`), or a sub-slice of either, without an
  explicit conversion. A parameter typed `&Vec<i32>` would only accept the first
  one and would offer nothing in return.
- Take `&mut Vec<T>` when you need to add or remove items.
- Take `Vec<T>` (no reference) when you actually want to consume the vector and
  take ownership.

Index access (`list[0]`) panics if out of bounds. `list.get(0)` returns
`Option<&T>` instead, so you can handle a missing element without panicking.
