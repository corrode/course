# Cap at a maximum

Write `cap_at(value, max)` so it returns `value` if it's at or below `max`, and `max` otherwise.
Both arguments are `i32`.
The logic is one `if` away.

Write the function the most natural way you can think of.
The most natural way doesn't compile, because function parameters are immutable bindings by default (like `let`), so reassigning `value` is rejected.
The fix is one keyword in one place.
Read the error, it points right at it.

Once it compiles, look at the second test.
The caller's variable is untouched even though the function reassigned its parameter.
That's because `i32` is `Copy`, so the function received its own copy to mutate.
You'll see what changes for non-`Copy` types like `String` when ownership and borrowing come up later.
