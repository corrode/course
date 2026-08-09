# Counting words

Start by turning a string of text into a `HashMap<String, usize>` that records how many times each word appears.
Words are separated by whitespace and the count should be case-insensitive: `"Hello"` and `"hello"` are the same word.

Build the map by splitting on whitespace, lowercasing each piece, and bumping its counter.
The `entry` API handles the lookup and default insertion together, then gives you the counter to update.

## Useful from the standard library

- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) splits on any whitespace and skips empty pieces.
  That makes it a better default for natural text than splitting on one literal space.
- [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase) returns a fresh `String`.
  Use it as the map key so `Hello` and `hello` collapse together.
- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) + [`Entry::or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert) is the "look up; insert default; mutate" pattern you used with hashmaps.
