# Wrapping up conditionals and loops

You used each kind of control flow for a slightly different job.
The mood classifier chose one branch, the two `for` loops walked through values you already had, and the `while` loop kept going until there was nothing left to divide.

## What we learned

- `if`/`else` is an *expression*, not just a statement.
  It can sit on the right of `let`, be returned from a function, or appear anywhere a value is expected.
  Both branches must have the same type.
- Conditions are `bool` expressions written without surrounding parentheses.
  Rust does not quietly treat an integer or a string as true or false.
- `for x in iter` is the type of loop to use when you already have something to walk through.
  Ranges (`0..n`, `0..=n`), slices, vectors, and most other collections all work here.
- `while cond` runs as long as the condition is true.
  Reach for it when the iteration count depends on values computed inside the loop (like "divide until zero").
- `loop` runs forever until you `break`.
  It can also produce a value: `let x = loop { ...; break value; };`.
- `break` exits the innermost loop; `continue` skips to the next iteration.
  In `count_evens`, `continue` handled the odd numbers first and kept the counter outside a nested `if`.
- An accumulator lets you compute one value from many inputs.
  You start with a value, update it once per loop iteration, and return it when the loop is done.
  Iterator methods such as `sum`, `count`, and `fold` cover many of the same jobs.
