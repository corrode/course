# Swapping with destructuring

Tuple destructuring makes swapping two values a one-liner: bind the pair to `(a, b)` and return `(b, a)`.
You don't need a temporary variable.

The integers in this exercise are `Copy`, so you can still use `a` and `b` after constructing `(b, a)`.
