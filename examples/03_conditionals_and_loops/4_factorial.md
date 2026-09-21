# Factorial with a `for` Loop

`n!` is `1 * 2 * 3 * ... * n`. By convention, `0! == 1`. Compute it with a
`for` loop.

Rust's `start..end` excludes the end, while `start..=end` includes it. Choose
the bounds for the factors you need. Any binding you update during the loop
needs `mut`; Rust bindings are immutable by default.
