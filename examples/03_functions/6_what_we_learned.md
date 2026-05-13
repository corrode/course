# Wrapping up functions

You wrote four small functions: a `String`-returning greeter, a
floating-point converter, a comparison that picks the smaller of two
numbers, and a boolean predicate. Same shape every time: declared
parameters, declared return, one expression in the body.

## What we learned

- `fn name(param: Type, ...) -> ReturnType { body }` is the universal
  function shape. Parameter types and the return type are always
  spelled out; Rust does not infer them.
- The body is a block. The block's value is its final expression
  (no trailing semicolon), and that becomes the return value.
- `return expr;` works too, but is almost always reserved for early
  exits. The no-semicolon form reads better at the end.
- Adding a stray semicolon to the last line turns the function into
  one that returns `()`. The compiler error is loud and points at
  the right line.
- Functions without a return value have type `()` and the `-> Type`
  part is omitted from the signature.
- `if`/`else` is itself an expression in Rust, so it can be the body
  of a function. Both branches must produce the same type.
- The `%` operator is the integer remainder, and `==` produces a
  `bool`. Together they cover most "is this number special?" checks.
