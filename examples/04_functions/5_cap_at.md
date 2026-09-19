# Cap at a Maximum

Write `cap_at(value, max)` so it returns `value` if it's at or below `max`, and
`max` otherwise. Both arguments are `i32`. Any implementation that passes the
tests is valid. You don't have to reassign a parameter to solve this.

Then try this experiment: use the signature
`fn cap_at(value: i32, max: i32) -> i32`, assign `max` to `value` when it
exceeds the limit, and return `value`. Run the tests and read the compiler's
explanation for rejecting the assignment. Change the parameter binding so this
version compiles. The chapter hints can help if you're stuck.

Look at the second test. The caller's variable is untouched even when the
function reassigns its parameter. That's because `i32` is `Copy`, so the
function received its own copy to mutate. You saw the other half with moves: a
non-`Copy` type such as `String` is moved in instead of copied. Borrowing lets a
function use a value without taking ownership.
