# Wrapping Up Vectors

You read a vector through a shared borrow and changed one through a mutable
borrow. You also built a fresh `Vec<String>` from borrowed `&str` inputs.

## What We Learned

- `Vec<T>` is a growable, heap-allocated array. The `<T>` is generic, but a
  single `Vec` only holds one type at a time.
- Build them with `Vec::new()` for an empty one, or the `vec![...]` macro when
  you already have the contents.
- Choose the parameter from the operation: `&[T]` to read, `&mut Vec<T>` to add
  or remove, and plain `Vec<T>` to consume the whole vector.
- `push` appends, `pop` removes the last item and returns `Option<T>`, `len`
  returns the number of items, and `is_empty` tells you whether there are any.
- Index access (`list[i]`) panics on out-of-bounds; `list.get(i)` returns
  `Option<&T>` and is the safer default.
- A `for item in &list` loop yields `&T`. That's usually what you want;
  iterating `&mut list` gives `&mut T`, and iterating `list` by value moves the
  items out.
- A `Vec<String>` is not the same as a `Vec<&str>`. Converting between them
  needs `.to_string()` / `String::from` (one direction) or `.as_str()` (the
  other).
