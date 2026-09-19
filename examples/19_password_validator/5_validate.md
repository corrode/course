# Return a Useful Report

Now put the pieces together in `PasswordValidator::validate(password)`. A caller
should get both a summary and every missing base requirement from a single call.
Unlike a parser that stops at its first error, this function must keep checking
after it finds a problem.

The character helpers and report methods are already implemented in this editor.
Your task is only the body of `validate`. Borrow the input, apply the following
scoring rules, and return a `PasswordReport`.

| Rule | Points | Feedback When Missing |
| --- | --- | --- |
| At least 8 characters | 20 | `Use at least 8 characters.` |
| An uppercase ASCII letter | 15 | `Add an uppercase ASCII letter.` |
| A lowercase ASCII letter | 15 | `Add a lowercase ASCII letter.` |
| An ASCII digit | 15 | `Add an ASCII digit.` |
| A character from `!@#$%^&*` | 15 | `Add one of !@#$%^&*.` |
| At least 12 characters | 10 more | None |
| At least 16 characters | 10 more | None |

Here, "characters" means Unicode scalar values, not bytes or visible symbols.
Keep whitespace and punctuation as they are; they count toward length even when
they don't satisfy a character-class rule. The length rewards accumulate, so an
input of at least 16 characters earns all 40 length points. Each character class
earns its points once, regardless of how many matching characters appear.

Use the exact feedback strings in the table, in table order, and only for failed
base rules. The two extra length rewards do not create complaints. Use
`PasswordStrength::from_score` for the label rather than writing the
classification rules again.

For example, `"Rust1234"` has eight characters and three of the four classes.
Its report has a score of 65, the label `Medium`, and one feedback message:
`"Add one of !@#$%^&*."`. An empty input scores zero and receives all five
base-rule messages.

The fixed rules make the tests precise. Once they pass, you can experiment with
another policy, but change its tests too.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
  with
  [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)
  counts scalar values; `str::len` counts bytes.
- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
  adds a message while preserving the order of the checks.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  creates an owned message for the report's `Vec<String>`.
