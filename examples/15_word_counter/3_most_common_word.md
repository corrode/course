# The most common word

Now that you can count, finding the maximum is a one-liner — almost.
The borrow checker has an opinion about returning data out of a
`HashMap`, and that's the real lesson of this step.

`count_words` is duplicated below as a `todo!()` stub so this step
compiles in isolation; you don't need to fill it in again — focus on
`most_common_word`. Once you have it, the test will drive both
through `unwrap()`.
