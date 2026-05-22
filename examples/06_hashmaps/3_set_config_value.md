# Setting a value

Updating a `HashMap` is the same operation as adding to it: one method
covers both cases, and it doesn't care whether the key was already
there. If the key existed, the old value is replaced (and returned);
if not, it's inserted fresh.

## Useful from the standard library

- [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)
  is the only call you need here. Returns `Option<V>`: `Some(old)` if
  the key was already present, `None` otherwise. You can ignore the
  return value when you don't care.
- [`HashMap::contains_key`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.contains_key)
  is a quick "is this key present?" check that doesn't retrieve the
  value. Not needed for this exercise, but worth knowing.
