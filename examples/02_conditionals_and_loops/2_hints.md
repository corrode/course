# Hints

## ferris_mood

1. The order of an `if`/`else if` chain decides everything: top to
   bottom, first match wins. Translate the rule table line by line
   and the order falls out for you.
2. The `"Grumpy"` rule needs both conditions to be true. Combine
   them with `&&` (logical AND).

## factorial

1. `let mut acc: u32 = 1;` outside the loop, `for i in 1..=n { ... }`
   inside. Return `acc` at the end.
2. The body of the loop is `acc *= i;`. Both `mut` on the binding
   and `*=` for the compound assignment are needed.

## count_evens

1. `let mut count = 0u32;` plus a `for n in numbers` loop. The
   suffix `0u32` pins the integer type so you don't need a separate
   annotation.
2. `for n in numbers` over a `&[i32]` yields `&i32`. The `%`
   operator works through the reference, so `n % 2` Just Works.
   `continue` skips the rest of the current iteration.

## digit_count

1. Special-case `n == 0` returning `1`. Otherwise, divide by 10 in
   a `while` loop and count the iterations.
2. Shadow the parameter with `let mut n = n;` so you can mutate it
   without changing the signature. Loop while `n > 0`, dividing by
   `10` and bumping a counter.
