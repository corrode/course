# Cap at a maximum

Write `cap_at(value, max)` so it returns `value` if it's at or below `max`, and `max` otherwise.
Both arguments are `i32`.
The logic is one `if` away.

Write the function the most natural way you can think of.
Your first version may not compile, and that failure is part of the exercise.
Read the error before changing anything because it tells you why the assignment is rejected.

Once it compiles, look at the second test.
The caller's variable is untouched even though the function reassigned its parameter.
That's because `i32` is `Copy`, so the function received its own copy to mutate.
You saw the other half with moves: a non-`Copy` type such as `String` is moved in instead of copied.
Next we'll borrow a value so a function can use it without taking ownership.
