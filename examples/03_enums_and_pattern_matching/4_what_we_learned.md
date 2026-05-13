# Wrapping up enums and pattern matching

You defined an enum with a fixed set of variants, mapped each variant
to a value with a `match`, and used `matches!` to ask a yes/no
question about a single variant.

## What we learned

- An `enum` is a "this or that or that" type. Each value is exactly
  one of its variants, and the compiler tracks which one.
- `match` checks a value against patterns top to bottom and runs the
  first arm that fits. Every arm is `pattern => expression`, and the
  whole `match` is itself an expression that produces a value.
- `match` is exhaustive: leave a variant unhandled and the compiler
  refuses to build. Add a new variant later and every `match` that
  needs updating tells you exactly where.
- `|` lets multiple patterns share an arm (`200 | 201 | 204 => ...`),
  and `_` is the catch-all when you want to ignore the rest.
- `#[derive(Debug, PartialEq)]` is the usual pair on a plain enum:
  one for `{:?}` printing, one for `==`. Add `Clone, Copy` when the
  variants carry no heap data so values can be passed around freely.
- For a single-variant check, `matches!(value, Variant)` is the
  compact form; `value == Variant` works equally well when
  `PartialEq` is derived.
