# Turning feedback into suggestions

The validator (next step) produces a `PasswordReport` with a list of
short complaints in `feedback`, like `"too short"` or `"missing digit"`.
This step turns those complaints into actionable, human-readable
suggestions.

Decide on your own format. A reasonable approach is to scan each
feedback string for keywords ("short", "uppercase", "digit", ...) and
emit a matching suggestion ("Add at least 4 more characters", "Mix in
an uppercase letter like A-Z", ...).

The shared types are duplicated here so this step compiles on its own.
