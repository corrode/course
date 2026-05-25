# Wrapping up vectors

You worked through every form a `Vec` parameter can take: a shared
borrow for reading, a mutable borrow for changing, and a fresh
`Vec<String>` produced from borrowed `&str` inputs.

## What we learned

- `Vec<T>` is a growable, heap-allocated array. The `<T>` is generic,
  but a single `Vec` only holds one type at a time.
- Build them with `Vec::new()` for an empty one, or the `vec![...]`
  macro when you already have the contents.
- The parameter version says what you intend to do: `&[T]` or
  `&Vec<T>` to read, `&mut Vec<T>` to add or remove, plain `Vec<T>`
  to consume the whole thing.
- `push` appends, `pop` removes the last item and returns
  `Option<T>`, `len` and `is_empty` answer the obvious questions.
- Index access (`list[i]`) panics on out-of-bounds; `list.get(i)`
  returns `Option<&T>` and is the safer default.
- A `for item in &list` loop yields `&T`. That's usually what you
  want; iterating `&mut list` gives `&mut T`, and iterating `list`
  by value moves the items out.
- A `Vec<String>` is not the same as a `Vec<&str>`. Converting
  between them needs `.to_string()` / `String::from` (one direction)
  or `.as_str()` (the other).
