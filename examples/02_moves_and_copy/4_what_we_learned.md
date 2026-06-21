# Wrapping up moves and Copy

You moved a `String` into a function and back out, and saw that an `i32` copies instead of moving.

## What we learned

- Assigning or passing a non-`Copy` value moves it.
  The old binding is dead, and using it again is a compile error, not a runtime surprise.
- A move hands over ownership cheaply (just the pointer).
  Rust makes deep copies explicit through `.clone()`.
- `Copy` types (integers, `bool`, `char`, fixed-size arrays of those) duplicate bit-for-bit instead of moving, so the original stays usable.
- The owner is responsible for the value: when it goes out of scope, the value is dropped, with no garbage collector involved.
- Borrowing, coming up next, lets you hand a value to a function without giving up ownership at all.
