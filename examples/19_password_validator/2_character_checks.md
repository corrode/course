# Check the Characters

Start with four small questions the validator will need to ask.
Does the input contain an uppercase letter, a lowercase letter, a digit, or one of our chosen special characters?
Keeping these checks separate lets you test each rule before combining them.

Implement `has_uppercase`, `has_lowercase`, `has_digit`, and `has_special`.
Each takes a borrowed `&str` and returns whether at least one character belongs to its class:

| Function | Characters That Count |
| --- | --- |
| `has_uppercase` | `A` through `Z` |
| `has_lowercase` | `a` through `z` |
| `has_digit` | `0` through `9` |
| `has_special` | Exactly `!@#$%^&*` |

An empty input satisfies none of the checks.
A matching character can appear anywhere, not just at the start.
Other characters are allowed in the input, but don't satisfy these rules: `É` is not an ASCII uppercase letter, and `?` is not in our special-character set.
Don't trim or change the input.

For example, `"café7?"` contains lowercase ASCII letters and an ASCII digit, but neither uppercase ASCII letters nor a special character from our set.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) visits Unicode scalar values without changing the string.
- [`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) answers whether any item satisfies a predicate.
- [`char::is_ascii_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase), [`is_ascii_lowercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_lowercase), and [`is_ascii_digit`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_digit) check the ASCII classes.
- [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains) can check whether a string contains a particular `char`.
