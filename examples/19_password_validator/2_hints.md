# Hints

## character_checks

Ask whether any character matches each rule; you don't need to count matches or allocate a new string.
Check the difference between the ASCII predicates and their Unicode counterparts.
For the special rule, treat the allowed characters as a small set, not as a substring that must appear in full.

## is_strong

Separate the two jobs.
`from_score` chooses a label from a number; `is_strong` reads a label from an existing report.
Walk through the values immediately below and at each boundary before you run the tests.
If you use range patterns, `..=` includes its upper endpoint.

## validate

Keep a score and an initially empty feedback vector.
For each base rule, either award its points or add its message, then continue to the next rule.
Check the two length bonuses independently: reaching 16 characters must not skip the reward for reaching 12.
Classify the finished score with the supplied method and construct the report last.

If only the non-ASCII tests fail, check how you measure length.
If a long input fails, check whether you converted its length to `u8` before comparing it with the thresholds.

## generate

Deal with an impossible length before constructing the string.
Guaranteeing each class appears is easier than generating arbitrary characters and hoping all four classes turn up.
Once those requirements are satisfied, extend the string to the requested length using allowed characters.
Since every allowed character is ASCII, its byte length and scalar count are the same.
No clock, random-number generator, or external crate is needed.
