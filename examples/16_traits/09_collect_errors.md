# Collect Every Error

A failed check should not hide the failures that come after it. For `"abc"`, a
minimum length of eight and a required `@` both fail, and the caller needs both
messages.

The mixed shelf exercise used trait objects to borrow values of different
concrete types through one trait. Here, `&dyn Validator` plays the same role for
validation rules. The supplied `&[&dyn Validator]` parameter borrows a slice of
those references, so length and substring rules can appear together. Calling
`check` through a trait object selects that rule's implementation at runtime.
The collector does not need to know the concrete rule type.

Implement only the body of `collect_errors`. The rule implementations are
supplied so this exercise stands on its own.

- Check every rule against the supplied input, even after a failure.
- Return only the failure messages, in the order the rules appear.
- Return an empty vector when every rule passes or when there are no rules.
- Accept any implementation of `Validator`, not just the two supplied types.

A passing rule between two failing rules must not interrupt collection or add a
message. How does this behavior differ from propagating the first error with
`?`?
