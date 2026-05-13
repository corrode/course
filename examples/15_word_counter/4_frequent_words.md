# Filtering frequent words

A different shape of result: instead of one winning word, return
every word whose count meets some threshold. The natural pipeline is
`count_words(text).into_iter().filter(...).map(...).collect()`, and
the `collect` infers `Vec<String>` from the return type.

As before, `count_words` is stubbed with `todo!()` so this step
compiles standalone. Your work is in `frequent_words`.
