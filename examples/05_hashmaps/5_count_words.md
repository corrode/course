# Counting with `entry`

Counting occurrences is the canonical "look up; if missing, insert a
default; then update" workflow. Doing it by hand with `contains_key`
and `get_mut` works but does two lookups and fights the borrow
checker. The `entry` API does it in one step.
