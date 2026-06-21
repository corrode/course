# Writing Guidelines

The house style for chapter prose and exercise doc-comments. The goal is a course
that respects the reader's time and existing experience. These rules exist to keep
20+ chapters consistent as we restructure; apply them to every chapter you touch.

## 1. Lead with the surprise (top principle)

For every topic, open with the angle that **differs from other languages**, the
**bug Rust prevents** at compile/runtime that would have shipped elsewhere, or the
way it **hands power back to the programmer**. Never spend the reader's attention
explaining what they already know; spend it on what makes Rust feel different and
worth it.

Per-chapter prompt: *"What's the empowering / bug-preventing Rust angle here?"*

- `Option` → no null, no NullPointerException
- `Result` → errors you can't forget to handle
- ownership → memory safety without a garbage collector
- enums → make illegal states unrepresentable
- iterators → no off-by-one, no manual bounds checks
- integers → overflow caught instead of silently wrapping

Where it fits, go **problem-first**: show the bug that would compile-and-crash in
many other languages (kept language-neutral, *"in most languages this throws at
runtime"*), then show Rust's fix. (A per-language interactive comparison is a
future idea, not something to author by hand now.)

## 2. Audience axiom

The reader already knows at least one other language. **Never explain what a
variable, loop, function, return value, boolean, or integer _is_.** Do explain
Rust-specific intuition, the *why*, and the gotchas a newcomer to Rust (not to
programming) will hit.

## 3. Voice

Peer-to-peer, warm, dry wit allowed. Write like a competent colleague pairing with
you, not a teacher addressing a class.

**Banned tics** (cut on sight):

- emoji-laden self-deprecation (e.g. "😬👉👈")
- "how exciting", "your first exercise"
- "the lightbulb moment", "the elephant in the room"
- "you might not have noticed, but…"
- any sentence that narrates the reader's emotional state *for* them

Dry epigraph jokes (like the integers chapter's "it's pointless") are fine. That's
wit, not condescension. Joke with the reader, never at them, and never about how
they're supposed to feel.

Write warm and human, the way you'd explain something to the person at the next
desk. A few specifics that keep it that way:

- No em-dashes. Use a period, a comma, or parentheses for an aside instead.
- Go easy on colons. The "term: explanation" pattern, stacked sentence after
  sentence, reads like clipped notes rather than writing. Prefer full sentences.
- Keep it simple, not compressed. A slightly longer plain sentence beats a terse,
  abbreviated one. Vary the length so it has a natural rhythm.

## 4. Density guard

Cutting a sentence that explains the obvious is good. Compressing two clear
sentences into one dense one is not. **Cut, don't crush.** Removing condescension
should make the prose lighter, not denser.

## 5. Cross-references

Refer to chapters by **name** ("the borrowing chapter", "the `Result` chapter"),
not by number, so renumbering doesn't rot the prose.

## 6. Pacing floor

Don't shrink a core chapter below ~2 hands-on exercises. It should feel like
practice, not a reading. (Pure "why" chapters like the ownership consolidation and
the appendix are exempt.)

## 7. Difficulty honesty

When a concept is genuinely hard (the borrow checker, the `?`/error-type story, the
CSV state machine), say so in a short, reassuring inline note, e.g. *"This is a
known hard spot. If it takes a few tries, that's the concept being hard, not
you."* Normalize the struggle instead of pretending everything is easy.
