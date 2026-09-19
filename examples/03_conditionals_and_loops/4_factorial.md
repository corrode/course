# Factorial with a `for` Loop

`n!` is `1 * 2 * 3 * ... * n`. By convention, `0! == 1`. Build it up with a
running accumulator and a `for` loop over the inclusive range `1..=n`. Start the
accumulator at `1`, then multiply each number into it as the loop goes along.

When `n` is `0`, the range `1..=n` is empty, so the loop does not run and the
initial `1` comes back unchanged. That gives you `0! == 1` without a special
case.

This is an accumulator: begin with one value, update it for every item, then
return the result. The binding needs `mut` because Rust bindings are immutable
unless you explicitly make them mutable.
