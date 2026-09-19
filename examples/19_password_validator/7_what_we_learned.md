# Wrapping Up the Password Validator

You started with small borrowed-string checks, gave their summary a type, and combined the rules into a report.
The generator reversed the problem: it constructed an input with known properties and rejected requests it couldn't satisfy.

## What We Learned

- Small predicates let you test each rule separately from the scoring policy.
- A report can own its feedback without owning or retaining the input password.
- An enum gives callers a fixed set of labels to match on.
- Collecting all missing requirements is a different error-handling choice from returning after the first failure.
- Precise boundary tests catch mistakes that one "weak" and one "strong" example would miss.
- Passing a set of formatting checks does not establish security.

If you want to take the project further, add a `PasswordPolicy` struct with a configurable minimum length.
Keep the feedback and score consistent with that setting, and write boundary tests before changing the validator.
Another useful experiment is to replace feedback strings with an enum of missing requirements, leaving human-readable wording to a separate display function.
