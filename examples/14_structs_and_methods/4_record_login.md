# Methods that mutate via `&mut self`

When a method needs to change the struct's data, it takes `&mut self`.
Declare `user` with `let mut` so the method can borrow it mutably.
At `user.record_login()`, Rust supplies the `&mut` borrow automatically.

`record_login` bumps a counter and flips a flag.
You can set `is_verified = true` even if the user is already verified.
That assignment is idempotent, so you don't need a branch to check the flag first.

## Useful from the standard library

- The `+= 1` operator updates a numeric field in place; the same works through `self`.
  You don't need `Cell` or any special handling.
- Plain assignment (`self.is_verified = true`) is enough for the bool.
  There's no separate "setter" syntax in Rust; methods on `&mut self` just assign.
- [`u32::checked_add`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add) is the safe-overflow alternative if you're worried about wrapping.
  You don't need it for this exercise.
