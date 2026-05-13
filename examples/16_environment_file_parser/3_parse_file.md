# Parsing a whole file

With single-line parsing solved, the file-level function is mostly
plumbing: iterate over `content.lines()`, skip blank lines and `#`
comments, and accumulate the rest into a `HashMap`. Stop at the first
malformed line and return an error. Strict parsing makes
configuration bugs obvious instead of silently dropping values.

Each step is self-contained, so the previous step's `parse_env_line`
and `ParseError` are re-declared here with `todo!()` bodies. Re-implement
them (or paste your earlier solution) so this step compiles on its
own.
