# Word Count Challenge

A word can have more than three bytes without having more than three Unicode
scalar values. These optional challenges build on Word Count with strings,
loops, conditionals, and borrowed text. First count all qualifying words, then
find the longest run of consecutive qualifying words. Neither exercise counts
toward course completion.

Both use the same rules. Words are separated by whitespace, including tabs,
newlines, and Unicode whitespace. Punctuation stays part of a word, so `"cat!"`
has four scalar values and qualifies. Count Unicode scalar values (`char`s), not
bytes or visible characters.

Each exercise has its own function and tests and can run without completing the
other.
