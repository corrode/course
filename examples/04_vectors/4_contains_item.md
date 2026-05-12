# Searching the list

Back to a read-only operation, but now we have to compare each
element against the item we're looking for. This is where the
borrowed-vs-owned distinction starts to bite: the `Vec` holds
`String`s, but we're searching with a `&str`.
