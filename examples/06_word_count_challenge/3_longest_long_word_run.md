# Optional Challenge: Consecutive Long Words

Write `longest_long_word_run(text)` to return the largest number of consecutive
words containing more than three Unicode scalar values (`char`s). Count words in
the run, not the scalar values they contain. A word with three or fewer scalar
values breaks the run. Return `0` if no words qualify.

Words are separated by whitespace, including tabs, newlines, and Unicode
whitespace. Repeated whitespace does not introduce empty words or break a run.
Punctuation stays part of a word, so `"cat!"` qualifies. Count scalar values,
not bytes or visible characters: `"été"` has three, `"café"` has four, and
`"abe\u{301}"` has four because the combining accent is a separate scalar value.

Use loops and conditionals with borrowed text, as in Word Count. This file
stands on its own; you do not need your `count_long_words` implementation.

Once the tests pass, explain why counting all qualifying words would answer a
different question.
