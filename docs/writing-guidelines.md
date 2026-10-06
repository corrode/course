# Writing Guidelines

The house style for chapter prose and exercise doc-comments. The goal is a
course that respects the reader's time and existing experience. These rules
exist to keep 20+ chapters consistent as we restructure; apply them to every
chapter you touch.

## 1. Lead with the Surprise

Open each topic with something that makes the reader curious: a Rust-specific
difference, a bug it prevents, or a useful choice it gives the programmer.
Enthusiasm, opinions, and playful comparisons are welcome. Keep the technical
claims accurate without turning the course into a reference manual.

Per-chapter prompt: *"What's interesting here, and why would I want to use it?"*

- `Option` represents a value that may be absent.
- `Result` represents success or failure in the return type.
- Ownership determines when values are moved and dropped.
- Enum variants can carry different data.
- Iterators process items without manually indexing a collection.
- Integer overflow behavior depends on the operation and build settings.

Where it fits, start with a concrete problem, then show how Rust handles it.
Name the relevant limits: a compile error, a runtime panic, and an explicit
error return are different outcomes. Don't imply that a feature prevents every
bug of a given kind. A per-language interactive comparison is a future idea,
not something to author by hand now.

Make the bug concrete. Don't just say a number overflows; walk through a
specific scenario and keep the numbers consistent between the prose and the
code. If the example sets health to 200 and adds a 100-point bonus, the prose
should talk about that same 200 dropping to 44, not an abstract "large value."
The reader should be able to trace the failure with the exact numbers they just
read.

## 2. Audience Axiom

The reader already knows at least one other language. **Never explain what a
variable, loop, function, return value, boolean, or integer _is_.** Do explain
Rust-specific intuition, the *why*, and the gotchas a newcomer to Rust (not to
programming) will hit.

## 3. Voice

Peer-to-peer, warm, opinionated, and playful. Write like a competent colleague
pairing with you, not a teacher addressing a class.

Preserve the author's humor, analogies, encouragement, and enthusiasm in both
headlines and body prose. "Enough syntax for a moment!" and "Dad is right,
enums are the best" are examples to keep, not AI tics to remove. Don't replace
a distinctive passage with a bland factual summary just because it is exuberant.

Watch for generic filler rather than banning enthusiasm:

- emoji-laden self-deprecation (e.g. "😬👉👈")
- "how exciting", "your first exercise"
- "the lightbulb moment", "the elephant in the room"
- "you might not have noticed, but…"
- repetitive reassurance that adds nothing to the explanation

Humor belongs throughout the course, not just in epigraphs. Jokes, personal
opinions, and invitations to try things can make a lesson memorable. Joke with
the reader, not at their expense.

Write warm and human, the way you'd explain something to the person at the next
desk. A few specifics that keep it that way:

- No em-dashes. Use a period, a comma, or parentheses for an aside instead.
- Go easy on colons. The "term: explanation" pattern, stacked sentence after
  sentence, reads like clipped notes rather than writing. Prefer full sentences.
- Keep it simple, not compressed. A slightly longer plain sentence beats a
  terse, abbreviated one. Vary the length so it has a natural rhythm.
- Remove formulaic emphasis and repetitive AI phrasing such as "names the
  exact" or "stops you right there at compile time." Evaluate each occurrence
  in context. This is not a ban on lively language, compiler personification,
  or the author's opinions. When in doubt, preserve the original voice.
- Bold sparingly. One short thesis sentence at the top of a section can be bold,
  the single claim you want the reader to remember ("Rust never mixes numeric
  types for you."). Never bold-lead the items of a bulleted list, and don't
  pepper bold through a paragraph.

## 4. Density Guard

Cutting a sentence that explains the obvious is good. Compressing two clear
sentences into one dense one is not. **Cut, don't crush.** Removing
condescension should make the prose lighter, not denser.

## 5. Cross-References

Refer to chapters by **name** ("the borrowing chapter", "the `Result` chapter"),
not by number, so renumbering doesn't rot the prose.

Chapter directory slugs follow the visible title in human-readable snake case,
not a literal transcription of Rust syntax. Omit generic parameters such as
`<T>` and `<T, E>`, keep `str` from `&str`, and spell `?` as `question_mark`.
For example, "Option<T>: When a Value Might Be Missing" maps to
`11_option_when_a_value_might_be_missing`, and "The ? Operator" maps to
`13_the_question_mark_operator`. Keep the numeric prefixes stable when aligning
names with titles, and mirror directory renames under `solutions/`.

Preserve saved learner progress with a database migration when renaming a
chapter. Match whole chapter stems, not prefixes: optional chapters such as
`06_word_count_challenge` and `23_even_more_csv_parsing` keep their names and
bonus status. Update active links and CLI examples, but leave historical
migration references intact.

## 6. Pacing Floor

Don't shrink a core chapter below ~2 hands-on exercises. It should feel like
practice, not a reading. (Pure "why" chapters like the ownership consolidation
and the appendix are exempt.)

Keep the pace relaxed, with something new to try regularly. A short warmup can
introduce syntax; a later exercise should let the reader combine it with
something they already know. Keep requirements and edge cases explicit so
progress comes from learning Rust, not guessing what the task wants.

Worked examples should teach the tools without solving the next exercise.
Check introductions, starter comments, and test fixtures too: could a reader
paste nearby code, change a name or literal, and be done? If so, change the
example's operation or leave a different decision for the exercise. A tiny
compiler repair can still be useful when the reader has to predict, diagnose,
or explain it. Don't add busywork just to make the answer longer.

## 7. Difficulty Honesty

When a concept is genuinely hard (the borrow checker, the `?`/error-type story,
the CSV state machine), say so in a short, reassuring inline note, e.g. *"This
is a known hard spot. If it takes a few tries, that's the concept being hard,
not you."* Normalize the struggle instead of pretending everything is easy.

## 8. Headings

Keep chapter titles, section headings, and app headlines short and in Title
Case. Capitalize the first and last word and the first word after a colon.
Capitalize nouns, pronouns, verbs, adjectives, and adverbs; keep articles,
coordinating conjunctions, and prepositions lowercase elsewhere, including
longer prepositions such as "into", "with", and "without". For example: "Text
into Numbers", "A Beginner's Guide to Rust", "Arrays First: Where Vectors Come
From", and "Static vs. Dynamic Dispatch: A Sneak Preview". Capitalize particles
in phrasal verbs ("Wrapping Up Numbers", "Set Up the CLI") and the main parts of
hyphenated compounds ("Single-Threaded", "Step-by-Step"). Preserve Rust
identifiers and types, inline code, acronyms, brand spelling, filenames, slugs,
and URLs exactly, even at the start or end of a heading: "Strings, &str, and
Chars", "Counting with `entry`", and "A Note from corrode". Hint headings that
match function names stay unchanged, such as `## factorial` or
``## `quoted_line`: The State Machine``. This policy does not apply to body
text, button labels, or code blocks.

Mix descriptive headings with occasional playful or motivational ones.
"Taming CSV" and "Choose Your Separator" are welcome; not every heading needs
to read like a reference manual. Playful explanations are welcome too.
Avoid headlines that imply a false technical guarantee: "No Silent Overflows"
is misleading when release builds can wrap.

## 9. Source Formatting: 80 Columns

Wrap prose and comments to 80 columns, including list and blockquote prefixes.
Reflow without changing wording or meaning. Preserve Markdown structure,
explicit hard breaks, and code fence contents. Code, tables, URLs, and
indivisible inline code may exceed the limit.

## 10. Info Boxes

GitHub-style alerts such as `> [!NOTE]`, `> [!TIP]`, and `> [!WARNING]` render
as callouts in the course. Use them for an aside the reader should notice
without interrupting the exercise: an editor tip, a limitation of a teaching
example, or a safety warning. Keep the task's motivation, contract, and
instructions in the main prose. Don't wrap every section in a box or repeat the
same warning at every step.

Put implementation nudges in the chapter's hints file under `## <step_slug>` so
the app reveals them beside the matching editor. A prompt should explain the
required behavior without giving away the function body. API links and type
explanations can stay in the prompt; a complete sequence of calls belongs in
optional help. Start hints with a useful question or one next step rather than
the whole algorithm. Keep full solutions available as a deliberate reveal, and
don't make using help feel like failing.

## 11. Code Comments in Examples

Do not use bold formatting in code comments, including doc-comments and
comments inside Markdown code blocks. Write the explanation in plain text;
backticks for code identifiers are fine.

Put short instructions for missing code in `todo!("Describe the task here")`
rather than a comment next to a bare `todo!()`. The message marks the place to
edit and appears when the unfinished code runs. Describe the required behavior,
not the implementation. Keep longer contracts, API references, and conceptual
explanations outside the macro; don't repeat the same instruction in both
places. Prediction placeholders should ask a question without giving its answer.

`todo!` takes a format string. Escape literal braces as `{{` and `}}` when a
message describes an output pattern rather than interpolating a variable.
In `const fn`, keep the instruction above a bare `todo!()`: the message-taking
form is not const-compatible on the current toolchain.

Comments in example code should teach, not label. Prefer a full sentence (or
two) on its own line above the line it explains, rather than a terse trailing
comment. Use a blank line to separate the setup from the line you're actually
demonstrating, so the reader's eye lands on the point:

```rust
let hp: u8 = 200;
let bonus: u8 = 100;

// The next line panics in debug mode
// because we attempt to add with overflow
let total = hp + bonus;
```
