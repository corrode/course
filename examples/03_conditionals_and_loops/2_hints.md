# Hints

## ferris_mood

1. An `if`/`else if` chain checks conditions from top to bottom; the first match
   wins. Follow the order in the rule table.
2. The `"Grumpy"` rule needs both conditions to be true. Combine them with `&&`
   (logical AND).

## factorial

1. Use a mutable accumulator.
2. What starting value works for multiplication and also makes `0!` correct?

## count_evens

1. `let mut count = 0u32;` plus a `for n in numbers` loop. The suffix `0u32`
   pins the integer type so you don't need a separate annotation.
2. `for n in numbers` over a `&[i32]` yields `&i32`. The `%` operator accepts
   the reference, so you can write `n % 2`. `continue` skips the rest of the
   current iteration.

## digit_count

1. What happens to the decimal digits when you divide an integer by 10? Trace
   it with `100` and decide what each iteration should count.
2. You can make a mutable working copy with `let mut remaining = n;` without
   changing the signature. What should happen for a single-digit input,
   including zero?
