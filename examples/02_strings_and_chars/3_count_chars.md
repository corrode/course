# Counting characters

Your first encounter with `&str`. In many languages, asking for the
"length" of a string gives you back the number of characters. In
Rust, `str::len` returns the number of *bytes* in the underlying
UTF-8 buffer, which only matches the character count for plain ASCII.

For "hello" the byte count and char count both happen to be 5, but
"café" is 5 bytes and 4 chars. Reach for `chars()` when you want the
character count.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  iterates over the `char`s of a string. The starting point for almost
  any character-level work.
- [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)
  consumes an iterator and returns how many items it produced.
- [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)
  is *byte* length, not character count. Useful, but not what you want
  here.
