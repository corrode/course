# Validating required variables

Most apps need *some* configuration to be present at startup:
a database URL, a port, an API key. This last helper takes a list of
required keys and reports the first one that's missing.

`Iterator::find` is a good fit: scan the slice, return the first
key that isn't in the map, and turn that into an `Err`. If `find`
returns `None`, every required key was present and the result is
`Ok(())`.
