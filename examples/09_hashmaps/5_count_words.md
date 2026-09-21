# Counting with `entry`

Count how often each word occurs. Unlike the configuration exercises, repeated
keys shouldn't replace a value: every occurrence contributes to the same count.
Use the `entry` API to handle both new and repeated words.

## Useful from the Standard Library

- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
  looks up a key and returns an `Entry` representing either an occupied or a
  vacant entry.
- [`Entry::or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert)
  returns a `&mut V`: either to the existing value, or to the default it just
  inserted.
- The `*` is the dereference operator from the chapter intro: it reaches through
  the `&mut V` to let you change the value inside the map.
