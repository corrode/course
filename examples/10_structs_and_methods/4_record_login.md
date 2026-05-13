# Methods that mutate via `&mut self`

When a method needs to change the struct's data, it takes
`&mut self`. The caller must hold the value in a `mut` binding for
this to compile. Mutability is opt-in at every layer.

`record_login` bumps a counter and flips a flag. The flag write is
idempotent: setting `is_verified = true` on an already-verified
user is harmless, which keeps the code simpler than a branch.
