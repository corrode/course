# Counting with `entry`

Counting occurrences is the canonical "look up; if missing, insert a
default; then update" workflow. Doing it by hand with `contains_key`
and `get_mut` works but does two lookups and fights the borrow
checker. The `entry` API does it in one step.

## Useful from the standard library

- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
  reserves the slot for a key in one lookup and hands you back an
  `Entry` you can operate on.
- [`Entry::or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert)
  returns a `&mut V`: either to the existing value, or to the default
  it just inserted. The full pattern reads
  `*map.entry(key).or_insert(0) += 1`.
- The `*` is the dereference operator from the chapter intro: it
  reaches through the `&mut V` so the `+= 1` updates the count
  inside the map.
