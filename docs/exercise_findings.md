# Exercise findings

Notes gathered while writing the reference solutions in `solutions/`.

This branch is intentionally **additive**: it does not modify anything
under `examples/`. Instead, anything misleading, subtly wrong, or worth
improving in an exercise is recorded here, so it can become a separate,
focused exercise-fixing pass later.

Severity legend:

- **blocker** - no correct solution can pass the exercise as written
  (e.g. a wrong test). These are raised immediately rather than worked
  around.
- **misleading** - the exercise compiles and is solvable, but the prose,
  naming, or tests point a learner the wrong way.
- **nit** - cosmetic or stylistic; harmless but worth a glance.

## Solution conventions worth knowing

- A solution mirrors its exercise file by path and keeps the `#[test]`
  blocks verbatim; only the `todo!()` bodies are filled in.
- Solutions stay within the concepts a learner has met by that chapter
  (e.g. chapter 6 uses a `for` loop instead of `iter().map().collect()`,
  which arrives in chapter 15) and use the standard-library methods the
  paired `.md` points at.
- **Chapter 03** (`stray_semicolon`, `countdown`, `sum_to`, `cap_at`)
  and the teaching files in **chapter 19** intentionally do not compile
  as exercises. Their solutions are the corrected, compiling versions
  (drop the stray semicolon; declare `mut value`; add the missing
  `pub`). The chapter-03 files carry their task description in a `//!`
  block and ship with no function stub, so each solution replaces that
  block with a short `///` doc on the function it adds, matching every
  other chapter's shape.

## Findings by chapter

Chapters not listed below had no issues: each step is clear, its tests
match the prose, and the standard-library methods the notes point at are
exactly what's needed.

### 16_word_frequencies
- **nit** `5_text_stats.rs`: `average_word_length` is undefined for empty
  input. With no words, `total == 0` and the average divides by zero,
  yielding `NaN`. The tests never exercise the empty case, and the return
  type (a bare `f64`) has no natural "no data" value, so the solution
  computes the average directly. If the exercise wants a defined answer,
  either guard `total == 0` and return `0.0`, or document that the input
  is assumed non-empty.
- **nit** `5_text_stats.rs`: "average word length" is ambiguous between
  "per word occurrence" (counting repeats) and "per unique word." The
  only test uses all-unique words, so both readings pass. The solution
  averages per occurrence (`sum(len * count) / total`), which matches the
  `total_words` framing of the tuple.

## Reviewed
All 21 exercise chapters (00-21) have solutions; chapters 22 (quiz) and
23 (appendix) contain no `.rs` exercises. Every solution compiles and its
tests pass under `scripts/check-solutions.sh`.
