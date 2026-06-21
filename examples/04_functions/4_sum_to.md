# Sum to N

Now do recursion the other way around.
Instead of printing as you go, *return* a value built up from smaller answers.

Write `sum_to(n)` so it returns `1 + 2 + ... + n`, with `sum_to(0) == 0`.

The base case and recursive case look like this:

```text
sum_to(0) = 0                    // base case
sum_to(n) = n + sum_to(n - 1)    // for n > 0
```

That's the whole idea of recursion: a function's answer is defined in terms of its own answer to a smaller version of the same problem.
Once the base case is reached, every pending call finishes its addition and the final total bubbles back up.
