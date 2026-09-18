# Sum to N

This exercise is about recursion: a function that calls itself.
Each call *returns* a value built up from the answer to a smaller version of the same problem.

Write `sum_to(n)` so it returns `1 + 2 + ... + n`, with `sum_to(0) == 0`.

The base case and recursive case look like this:

```text
sum_to(0) = 0                    // base case
sum_to(n) = n + sum_to(n - 1)    // for n > 0
```

Once you reach the base case, every pending call finishes its addition and returns the total to its caller.
