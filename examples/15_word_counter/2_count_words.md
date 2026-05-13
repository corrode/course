# Counting words

The foundation for everything else in this chapter: take a string of
text and produce a `HashMap<String, usize>` that maps each word to
how many times it appears. Words are separated by whitespace and the
count should be case-insensitive: `"Hello"` and `"hello"` are the
same word.

The classic recipe is: split on whitespace, lowercase each piece,
then walk the resulting iterator and bump a counter in the map. The
`entry` API on `HashMap` is the idiomatic way to do that last step:
`*map.entry(key).or_insert(0) += 1`.

## Useful from the standard library

- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
  splits on any whitespace and skips empty pieces. Almost always
  what you want for word splitting.
- [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase)
  returns a fresh `String`. Use it as the map key so `Hello` and
  `hello` collapse together.
- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
  + [`Entry::or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert)
  is the "look up; insert default; mutate" pattern from chapter 5.
