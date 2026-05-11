# Parsing a single line

Before tackling whole files, get one line right. The `.env` format is
`KEY=value`, but real-world files have surrounding whitespace too.
`KEY = value` should be accepted and produce `("KEY", "value")`.

`str::split_once('=')` is the right tool: it gives back
`Option<(&str, &str)>` containing the part before and after the first
`=`. From there it's `trim` plus a couple of validity checks.

We also introduce a small `ParseError` enum that the rest of the
chapter will reuse.
