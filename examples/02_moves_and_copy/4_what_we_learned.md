# Wrapping Up Moves and Copy

You moved a `String` into a function and back out, and saw that an `i32` copies
instead of moving.

## What We Learned

- Assigning or passing a non-`Copy` value moves it. The old binding no longer
  holds the value; using it again before reassigning it is a compile error.
- Moving a `String` transfers its pointer, length, and capacity without copying
  the heap buffer. To copy the buffer too, call `.clone()`.
- `Copy` types (integers, `bool`, `char`, fixed-size arrays of those) duplicate
  bit-for-bit instead of moving, so the original stays usable.
- The owner is responsible for the value: when it goes out of scope, the value
  is dropped, with no garbage collector involved.
- Borrowing lets a function use a value without taking ownership from its
  caller.
