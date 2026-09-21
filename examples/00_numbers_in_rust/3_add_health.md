# Health That Doesn't Wrap Around

A `u8` holds 0 to 255. Add 100 to 200 and a debug build panics, while a release
build wraps to 44. Neither is what you want from a health bar.

`saturating_add` is the fix. It does the addition but clamps at the type's
maximum instead of overflowing, so stacking buffs tops out at 255 rather than
wrapping around.

> [!TIP]
> Vim user? Open Settings in the top bar and turn on **Vim Keybindings**
> to use Vim in the exercise editors. Your choice is saved in this browser.

## Useful from the Standard Library

- [`u8::saturating_add`](https://doc.rust-lang.org/std/primitive.u8.html#method.saturating_add)
  returns the sum, or the type's maximum if the true sum wouldn't fit.
