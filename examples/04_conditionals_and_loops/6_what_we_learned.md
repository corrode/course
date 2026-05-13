# Wrapping up conditionals and loops

You wrote a three-way classifier with `if`/`else if`/`else`, two
accumulator-style loops (a `for` over a range and a `for` over a
slice with `continue`), and a `while` loop where the iteration count
isn't known up front.

## What we learned

- `if`/`else` is an *expression*, not just a statement. It can sit
  on the right of `let`, be returned from a function, or appear
  anywhere a value is expected. Both branches must have the same
  type.
- Conditions are bare `bool` expressions. No parentheses required,
  no implicit conversion from integers or strings.
- `for x in iter` is the default loop. Ranges (`0..n`, `0..=n`),
  slices, vectors, and most other collections all produce iterators
  you can put on the right.
- `while cond` runs as long as the condition is true. Reach for it
  when the iteration count depends on values computed inside the
  loop (like "divide until zero").
- `loop` runs forever until you `break`. It can also produce a
  value: `let x = loop { ...; break value; };`.
- `break` exits the innermost loop; `continue` skips to the next
  iteration. A `continue` to early-out the boring case usually
  reads better than nesting the work inside an `if`.
- The accumulator pattern (`let mut acc = ...; for ... { acc = ...; }`)
  is the standard shape for "compute one value from many". Once you
  meet iterators in chapter 13, methods like `sum`, `count`, and
  `fold` will replace many of these by-hand loops.
