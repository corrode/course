# Keep a username after checkout

Chapter 11's `find_user_by_id` returned a borrowed username. That worked while the user records stayed alive.
Now imagine saving a username for a goodbye message after those records have been dropped.

This step includes a working copy of the lookup, so you don't need code from another editor.
Run its starting tests first. Then:

1. Find the commented `test_username_survives_records` test. Before enabling it, predict whether its borrowed result can survive the end of the inner block. Which value owns the text?
2. Remove the `/*` and `*/` around that test and run it. Read the compiler error before editing the function.
3. Choose a return type for `find_user_by_id` that meets the new requirement, then implement it. Keep the borrowed input parameter and all tests unchanged. Don't leak memory, hardcode names, or move the records outside their block.

The lookup must still return the first matching username, or `None` for a missing ID. The caller must keep its records usable too.
You don't need lifetime annotations for this task.

The lifetime error is opt-in. To get back to the passing starting state before attempting a repair, comment out the survival test again.
Finishing the exercise means all four tests pass with that test enabled.

After your repair, explain when you'd prefer chapter 11's borrowed return and what keeping an independent username costs here.
