# Keep a username after checkout

In the `Option<T>` chapter, `find_user_by_id` returned a borrowed username. That worked while the user records stayed alive.
Now imagine saving a username for a goodbye message after those records have been dropped.

This step includes a working copy of the lookup, so you don't need code from another editor.
Run its starting tests first: three pass, and the survival checkpoint fails at `todo!()` because the task is unfinished. The starter compiles without a lifetime error. Then:

1. Find the commented survival check inside `test_username_survives_records`. Before enabling it, predict whether its borrowed result can survive the end of the inner block. Which value owns the text?
2. Remove the `todo!()` and the `/*` and `*/` around the survival check, then run the tests. Read the compiler error before editing the function.
3. Choose a return type for `find_user_by_id` that meets the new requirement, then implement it. Keep the borrowed input parameter and all tests unchanged. Don't leak memory, hardcode names, or move the records outside their block.

The lookup must still return the first matching username, or `None` for a missing ID. The caller must keep its records usable too.
You don't need lifetime annotations for this task.

The lifetime error is opt-in. To return to the starter, comment out the survival check inside the test and restore its `todo!()`. Keep the test itself active: it should still fail until you complete the repair.
Finishing the exercise means all four tests pass with the survival check uncommented and the `todo!()` removed.

After your repair, explain when you'd prefer the `Option<T>` chapter's borrowed return and what keeping an independent username costs here.
