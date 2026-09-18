# Methods That Mutate via `&mut self`

When a method needs to change the struct's data, it takes `&mut self`.
Declare `user` with `let mut` so the method can borrow it mutably.
At `user.record_login()`, Rust supplies the `&mut` borrow automatically.

`record_login` bumps a counter and flips a flag.
You can set `is_verified = true` even if the user is already verified.
That assignment is idempotent, so you don't need a branch to check the flag first.

## Updating Fields

- The `+= 1` operator updates a numeric field in place; the same works through `self`.
- Plain assignment (`self.is_verified = true`) is enough for the bool.
  There's no separate "setter" syntax in Rust; methods on `&mut self` just assign.
