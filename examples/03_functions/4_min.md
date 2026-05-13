# Returning the smaller value

Two parameters this time, and the body has to *decide* which one to
return. The natural shape is `if a < b { a } else { b }`. That's a
single `if`/`else` expression, and the value of the chosen branch
becomes the function's return value.

`if` is fully covered next chapter. The short version: the
condition needs no parentheses, the branches are blocks, and the
whole `if`/`else` is itself an expression that produces a value.

## Useful from the standard library

- [`std::cmp::min`](https://doc.rust-lang.org/std/cmp/fn.min.html)
  already does this for any type that implements `Ord`. Worth
  knowing about, but for the exercise stick with `if`/`else` so the
  expression-as-value lesson lands.
- The body is one expression. Both branches must produce the same
  type (`i32` here), and there's no semicolon at the end of the
  block.
