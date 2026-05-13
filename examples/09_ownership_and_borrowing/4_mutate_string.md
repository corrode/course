# Mutable borrows

Sometimes you want to modify a value in place without taking
ownership of it. That's a mutable borrow: `&mut T`. The caller
still owns the value, but the callee gets exclusive write access
for the duration of the call.

Two things to notice in the signature:

1. The parameter is `&mut String`, not `&mut str`. We need the
   *owned* `String` because growing it (with `push_str`) may
   reallocate; a bare string slice has a fixed length.
2. There's no return value. The mutation happens through the
   reference and is visible to the caller after the call returns.

On the call site (see the test): the caller has to write
`&mut s` explicitly, and `s` itself has to have been declared
`let mut s = ...`. Mutability is opt-in at every layer.

## Useful from the standard library

- [`String::push_str`](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str)
  works on `&mut String` exactly the same way as on an owned
  `String`. The compiler reaches through the reference for you.
- [`String::push`](https://doc.rust-lang.org/std/string/struct.String.html#method.push)
  is the single-`char` version, in case you want to append one
  character at a time.
