# Searching the list

You only need to read the list here, comparing each element against the item you're looking for.
The `Vec` holds `String`s, but we're searching with a `&str`.

## Useful from the standard library

- [`<[T]>::contains`](https://doc.rust-lang.org/std/primitive.slice.html#method.contains) takes a reference to the element type: its signature is `fn contains(&self, x: &T) -> bool`.
  Here that's `&String`, while the parameter is `&str`.
  A loop lets you compare the strings without allocating a temporary `String`.
- A `for entry in list` loop yields `&String` on each iteration.
  Compare `entry == item` directly; no temporary `String` or `.as_str()` call is needed.
