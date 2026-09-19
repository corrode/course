# Optional Challenge: Longer Words

Write `count_long_words(text)` to count whitespace-separated words containing more than three Unicode scalar values (`char`s).
Use what you've learned so far, without an implementation recipe.

Exactly three does not qualify.
Count scalar values, not bytes or visible characters: `"été"` has three, while `"café"` has four.
Whitespace separates words, including tabs, newlines, and Unicode whitespace.
Punctuation stays part of a word, so `"cat!"` qualifies.
An empty or whitespace-only input has no qualifying words.
