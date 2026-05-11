# Character-class helpers

The validator's scoring rules all boil down to questions like "does this
password contain an uppercase letter?" Before assembling the orchestrator,
write the small predicates that answer each one.

All four functions take a `&str` and return `bool`. The "special"
character set for this exercise is `!@#$%^&*` — feel free to expand it if
you want a stricter validator later.

Hint: `str::chars()` plus `Iterator::any` is the natural shape here.
