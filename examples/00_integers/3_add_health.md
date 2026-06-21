# Health that doesn't wrap around

A `u8` holds 0–255. Add past 255 and a debug build panics, while a release build
wraps to a tiny number — so a fully-buffed character could read 3 HP instead of
a capped 255. Neither is what you want from a health bar.

`saturating_add` is the fix: it does the addition but clamps at the type's
maximum instead of overflowing. Use it so stacking buffs tops out at 255 rather
than wrapping around.

## Useful from the standard library

- [`u8::saturating_add`](https://doc.rust-lang.org/std/primitive.u8.html#method.saturating_add)
  returns the sum, or the type's maximum if the true sum wouldn't fit.
