# Producing an `Option<char>`

Now you have to *produce* an `Option`, not consume one. Asking a
string for its first character is the canonical example: if the
string is empty, there is no first character, and returning some
"default" `char` would be a lie. `Option<char>` is the honest type.

You could pattern-match by hand, but the standard library has
already done the work for you: `text.chars()` returns an iterator,
and every iterator's `.next()` already hands you `Option<Item>`.
Compose the two.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  returns an iterator over the `char`s of the string.
- [`Iterator::next`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.next)
  pulls one item off the iterator and returns it as `Option<Item>`.
  For the first character, that's `Option<char>` and exactly the
  return type.
