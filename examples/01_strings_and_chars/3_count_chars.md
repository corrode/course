# Counting Characters

In many languages, asking for the "length" of a string gives you back the number
of **characters**. In Rust, `str::len` returns the number of **bytes** in the
underlying UTF-8 buffer. For "hello" the byte count and char count both happen
to be 5, but "café" is 5 bytes and 4 chars. That's because the `é` is two bytes
in UTF-8.

Use `chars()` to count Unicode scalar values rather than bytes.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  iterates over the `char`s of a string.
- [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)
  consumes an iterator and returns how many items it produced.
