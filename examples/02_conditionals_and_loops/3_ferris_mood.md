# Ferris's mood

Ferris the crab is a creature of simple needs. Two things determine
his mood on any given day: how hungry he is (on a `0..=10` scale)
and how many naps he's managed to fit in.

Implement `ferris_mood(hunger, naps)` returning a `&'static str`,
following these rules:

| Condition                                           | Mood        |
|-----------------------------------------------------|-------------|
| `hunger >= 8`                                       | `"Hangry"`  |
| `hunger >= 5` **and** `naps == 0`                   | `"Grumpy"`  |
| `naps >= 3`                                         | `"Sleepy"`  |
| anything else                                       | `"Content"` |

`&'static str` just means "a borrowed string slice that lives for
the whole program". String literals like `"Hangry"` automatically
have this type.

## Two things to watch

**Combining conditions.** The `"Grumpy"` rule needs *both* parts to
be true. Rust spells this `&&` (logical AND). Its sibling `||` is
logical OR. Both short-circuit: if the left side already decides
the answer, the right side isn't evaluated.

**Order matters.** An `if`/`else if`/`else` chain is checked
top-to-bottom and stops at the first match. If you put the `naps`
check before the `hunger` check, a hungry crab who happens to have
napped a lot will get classified as `"Sleepy"` instead of
`"Hangry"`. The tests deliberately include cases (like
`ferris_mood(9, 5)`) that only pass with the right ordering.
