# Text statistics

The orchestrator step. `text_stats` returns three numbers about a
piece of text: total word count, number of unique words, and the
average word length as an `f64`. You can compute all three from a
single pass over `count_words`'s result, or split the work; either
is fine.

`count_words` is stubbed with `todo!()` again so this file compiles
on its own. Wire `text_stats` up however you like. The test only
cares about the returned tuple.
