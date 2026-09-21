# Hints

## parse_file

A blank line or comment is allowed; a malformed entry is not. Decide which kind
of line you have before asking the line parser to handle it. Leading spaces
shouldn't hide a comment marker.

The line parser and file parser return the same error type. What does that let
you do with an error from the helper? A loop is fine here; you don't need to fit
everything into one iterator chain.
