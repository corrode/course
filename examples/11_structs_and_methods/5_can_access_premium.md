# Predicates over multiple fields

Methods are a natural place to encode business rules that span
several fields. Rather than scattering `user.is_verified && ..`
checks across the codebase, name the rule once on the type so the
intent is obvious at every call site.

`can_access_premium` combines two conditions into a single `bool`.
In Rust, the body of a function is an expression, so you can just
write the boolean expression with no `return` and no semicolon.

## Useful from the standard library

- The `&&` operator short-circuits: if the left side is `false`,
  the right side isn't evaluated. Cheap and matches what you'd
  write in any other language.
- The body is a single expression, so leave off the trailing
  semicolon. `self.is_verified && self.login_count >= 5` is a
  complete function body.
- The expression is idempotent for the caller (`&self`), so it's
  safe to call as many times as you want without worrying about
  accidental mutation.
