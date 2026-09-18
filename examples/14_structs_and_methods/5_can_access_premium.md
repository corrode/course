# Predicates over multiple fields

If you keep writing `user.is_verified && ..` in different places, give that rule a name.
A method lets you keep the checks together and tells callers what they're asking: can this user access premium features?

`can_access_premium` combines two conditions into a single `bool`.
In Rust, the body of a function is an expression, so you can just write the boolean expression with no `return` and no semicolon.

## Useful from the standard library

- The `&&` operator short-circuits, so a `false` condition on the left skips the condition on the right.
  Here an unverified user doesn't need a login-count check.
- Since the method returns the value of its final expression, leave off the trailing semicolon.
  You don't need an explicit `return` for the boolean.
- Taking `&self` means callers can ask the question without giving up ownership or providing a mutable borrow.
