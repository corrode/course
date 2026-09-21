# Keep a Username

In the [`Option<T>` chapter](11_option_when_a_value_might_be_missing),
`find_user_by_id` returned a borrowed username. Now the caller needs to keep that
name after the user records are dropped.

Run the supplied code first: three tests pass, and
`test_username_survives_records` stops at `todo!()`.

1. Read the commented code in `test_username_survives_records`. Before
   uncommenting it, predict whether it will compile. Which value owns the text,
   and when is that value dropped?
2. Remove the `todo!()` and the `/*` and `*/`, then run the tests. Read the
   compiler error before editing the function.
3. Change the return type and body of `find_user_by_id` so the caller can use
   the username after the records are dropped. Keep the borrowed input parameter
   and all tests unchanged. The records must still be dropped at the end of
   their block.

The lookup must still return the first matching username, or `None` for a
missing ID. The caller must still be able to use its records after the call. You
don't need lifetime annotations for this task.

All four tests should pass when you're done. To set the exercise aside, restore
the last test's comment markers and `todo!()` rather than removing the test.

When is the borrowed return from the `Option<T>` chapter a better choice? What
extra work does your implementation do to keep a separate username?
