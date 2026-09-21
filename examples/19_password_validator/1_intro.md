# Password Validator

A password form that says only "invalid" leaves the user guessing. Let's build
one that explains which checks failed. The result will be a Rust value a caller
can inspect, rather than a message printed inside the validator.

This optional project brings together borrowing, enums, structs, iterators, and
`Result`. Try it after the [iterators chapter](17_iterators). There are
four steps:

1. Check which character classes an input contains.
2. Represent a score and its feedback in a report.
3. Apply the rules and return that report.
4. Generate sample inputs for testing, including handling an impossible request.

Each editor runs on its own. Later steps supply the helpers you've already
practiced, so you can focus on the new task instead of copying earlier answers.
Read the contract above each editor, try the tests, and open its hints if you
get stuck.

> [!WARNING]
> This is a programming exercise, not a password-security tool. The scores and
> the label `Strong` describe our made-up rules; they do not measure how hard a
> password is to guess. Use invented inputs only: **Run** sends the code,
> including its test strings, to the Rust Playground.

> [!TIP]
> The files get longer here because each editor includes its own support code
> and tests. **Open in Web Editor** opens the file on
> [github.dev](https://github.dev/corrode/course) if you'd like more room. For
> local execution and `rust-analyzer`, clone [the
> repo](https://github.com/corrode/course) and run
> `cargo test --example 19_password_validator`. The unfinished steps will fail
> until you implement them.
