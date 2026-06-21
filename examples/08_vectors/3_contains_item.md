# Searching the list

Back to a read-only operation, but now we have to compare each element against the item we're looking for.
This is where the borrowed-vs-owned distinction starts to bite: the `Vec` holds `String`s, but we're searching with a `&str`.

## Useful from the standard library

- [`<[T]>::contains`](https://doc.rust-lang.org/std/primitive.slice.html#method.contains)
  is the obvious tool, but its signature is `fn contains(&self, x: &T) -> bool`.
  Here that's `&String`, while the parameter is `&str`.
  The mismatch is real, hence the loop.
- A `for item in list` loop yields `&String` on each iteration.
  Comparing `item == search` works because `String` and `&str` know how to compare against each other.
- [`str::eq`](https://doc.rust-lang.org/std/primitive.str.html#impl-PartialEq%3Cstr%3E-for-String)
  via `==` is the cleanest way to compare two strings regardless of ownership; no manual `.as_str()` needed.
