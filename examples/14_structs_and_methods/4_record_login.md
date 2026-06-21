# Methods that mutate via `&mut self`

When a method needs to change the struct's data, it takes `&mut self`.
The caller must hold the value in a `mut` binding for this to compile.
Mutability is opt-in at every layer.

`record_login` bumps a counter and flips a flag.
The flag write is idempotent: setting `is_verified = true` on an already-verified user is harmless, which keeps the code simpler than a branch.

## Useful from the standard library

- The `+= 1` operator updates a numeric field in place; the same works through `self`.
  No `Cell` or special-casing required.
- Plain assignment (`self.is_verified = true`) is enough for the bool.
  There's no separate "setter" syntax in Rust; methods on `&mut self` just assign.
- [`u32::checked_add`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add) is the safe-overflow alternative if you're worried about wrapping.
  Not needed here.
