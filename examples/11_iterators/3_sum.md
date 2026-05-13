# Summing with an iterator

Iterators were popularised by functional languages like Lisp (created by
John McCarthy in 1958), and today they're a core building block in most
modern languages. Rust's iterators are lazy: they don't do any work
until you ask for a result.

The simplest pattern is to take a sequence and collapse it down to a
single value. You could write a `for` loop with a running total, but
the standard library can do this for you in one call.

## Useful from the standard library

- [`<[T]>::iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter)
  produces an iterator of shared references over the slice.
- [`Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)
  reduces a numeric iterator to a single total. It's generic over the
  output type, so the compiler needs a hint: either annotate the
  binding (`let total: i32 = ...`) or use the turbofish
  (`.sum::<i32>()`).
- [`Iterator::product`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)
  is the multiplicative cousin if you ever need a running product.
