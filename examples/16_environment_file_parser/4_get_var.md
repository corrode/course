# Typed lookup with generics

Configuration values are stored as strings, but consumers want
`u16` ports, `bool` flags, and so on. Rather than write one helper
per type, declare a generic function bounded by `FromStr` and let the
caller pick the type at the call site with a turbofish or a type
annotation.

Inside the body, `env.get(key)?` short-circuits on a missing key and
`.parse().ok()` collapses the parse `Result` into an `Option`. Don't
try to `?` the parse: `T::Err` is unconstrained here and would need
an extra `From` bound.
