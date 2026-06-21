# Factorial with a `for` loop

`n!` is `1 * 2 * 3 * ... * n`.
By convention, `0! == 1`.
Build it up with a running accumulator and a `for` loop over an inclusive range.

The accumulator pattern shows up everywhere once you start writing loops: `let mut acc = ...; for x in ... { acc = ... }; acc`.
Note the `mut`: bindings are immutable by default, and the loop body needs to update `acc`, so you have to opt in.
