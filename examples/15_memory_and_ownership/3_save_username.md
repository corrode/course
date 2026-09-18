# Keep a username after its records are dropped

In the `Option<T>` chapter, `find_user_by_id` returned a borrowed username.
That worked while the user records stayed alive.
What if you need to use the username after those records have been dropped?

The lookup is already implemented here.
Run the tests first.
Three pass, and `test_username_survives_records` fails at `todo!()`.
The code that checks the username after the records are dropped is commented out for now.

1. Read the commented code in `test_username_survives_records`.
   Before uncommenting it, predict whether it will compile.
   Which value owns the text, and when is that value dropped?
2. Remove the `todo!()` and the `/*` and `*/`, then run the tests.
   Read the compiler error before editing the function.
3. Change the return type and body of `find_user_by_id` so the caller can use the username after the records are dropped.
   Keep the borrowed input parameter and all tests unchanged.
   Don't leak memory, hardcode names, or move the records outside their block.

The lookup must still return the first matching username, or `None` for a missing ID.
The caller must still be able to use its records after the call.
You don't need lifetime annotations for this task.

To return to the starting code, comment out the code inside `test_username_survives_records` and restore its `todo!()`.
Don't comment out the test itself; it should fail until you finish the exercise.
You're done when all four tests pass with that code uncommented and the `todo!()` removed.

When is the borrowed return from the `Option<T>` chapter a better choice?
What extra work does your implementation do to keep a separate username?
