# Counting items

The simplest possible operation on a vector is to ask it how many items it currently holds.
Notice the parameter is `&Vec<String>`: a borrow, so the caller keeps
ownership of the list.

## Useful from the standard library

- [`Vec::len`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len)
  returns the number of items as a `usize`. Constant time, no
  allocation.
- [`Vec::is_empty`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty)
  reads better than `len() == 0` when that's all you need to know.
