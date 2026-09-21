# Sum to N

This exercise is about recursion: a function that calls itself. Each call
*returns* a value built up from the answer to a smaller version of the same
problem.

Write `sum_to(n)` so it returns `1 + 2 + ... + n`, with `sum_to(0) == 0`.

A recursive call uses ordinary function-call syntax, even inside the function
being defined. It needs a base case that returns without calling itself again.
Each other call should move closer to that case.

How does the answer for `n` relate to the answer for `n - 1`? Use that relation
to decide what each call should return.
