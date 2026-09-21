# Producing an `Option<char>`

Now you have to *produce* an `Option`, not consume one. If the string is empty,
there is no first character; `Option<char>` represents that case with `None`.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  returns an iterator over the `char`s of the string.
- [`Iterator::next`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.next)
  pulls one item off an iterator and returns it as `Option<Item>`. An exhausted
  iterator returns `None`.
