# Quiz module — plan

A short post-chapter quiz turns the "I followed along" feeling into "I
can recall this without the editor in front of me." This document
sketches how to add one quiz per chapter.

## Design goals

- **Same look and feel as the rest of the course.** No external quiz
  framework like quizdown; the questions live in the repo as plain
  markdown so they're easy to edit, version, and review.
- **Authored once, rendered server-side.** No JavaScript dependency for
  display; the only client code is the answer-checking logic.
- **Per-question instant feedback.** Tell the learner immediately when
  they get one wrong, with a short explanation pointing back to the
  relevant chapter section. No score-only "you got 7/10" page.
- **Progress tracked alongside exercises.** Reuse the existing
  `submissions` table. Each chapter's quiz counts as one extra
  "exercise" with a special completion criterion.
- **Skippable.** A chapter shouldn't be locked behind its quiz. Quizzes
  are a self-test, not a gate.

## Data model

Each chapter directory grows an optional `quiz.md` file alongside the
existing `1_intro.md` / `main.rs`:

```
examples/02_strings_and_chars/
  1_intro.md
  main.rs
  quiz.md          ← new
```

The format is a small front-matter-free markdown dialect:

```markdown
# Strings and chars — quiz

## Question
What does `"café".len()` return?

- [ ] 4
- [x] 5
- [ ] 6

> `len()` returns the number of *bytes*. `é` is two bytes in UTF-8, so
> `"café"` is 5 bytes long. Use `.chars().count()` for character count.

## Question
Which type owns its string data?

- [ ] `&str`
- [x] `String`
- [ ] `&'static str`

> `String` is heap-allocated and owned. `&str` is a borrowed view; the
> data is owned by someone else.
```

Conventions:
- Top-level `# Title` becomes the quiz heading.
- Each `## Question` block introduces one question.
- The block ends at the next `## Question` or end-of-file.
- A bullet list with `[ ]` / `[x]` checkboxes provides options. Multiple
  `[x]` boxes mean "select all that apply." Exactly zero correct
  answers is a parse error.
- An optional `> blockquote` immediately after the options is the
  per-question explanation, shown after the learner submits the answer.
- Anything else inside a `## Question` block is rendered as the question
  prose (so questions can include code blocks, tables, etc.).

## Parser

Add `pub mod quizzes` next to `exercises` in `src/lib.rs`.

```rust
pub struct Quiz {
    pub slug: String,                 // chapter slug, e.g. "strings_and_chars"
    pub title: String,
    pub questions: Vec<Question>,
}

pub struct Question {
    pub id: String,                   // stable hash of the prompt, used by the JS
    pub prompt_html: String,          // rendered markdown
    pub options: Vec<Option_>,
    pub explanation_html: Option<String>,
}

pub struct Option_ {
    pub label_html: String,
    pub correct: bool,
}

pub fn scan_quizzes(examples_dir: &Path) -> anyhow::Result<Vec<Quiz>>;
```

The parser walks the existing `examples/NN_slug/` directories, looking
for `quiz.md`. It reuses `exercises::render_markdown` for prose and
options (so code blocks get the same syntax highlighting).

## Server

Two new routes on `server.rs`:

| Route | Purpose |
|---|---|
| `GET /quiz/{slug}` | Public render of a chapter's quiz. |
| `GET /quiz/{ulid}/{slug}` | Same, with participant context for tracking. |
| `POST /api/quiz-submit` | Persist that a participant completed a quiz, plus their score. |

`AppState` gains a `quizzes: Arc<Vec<Quiz>>` field, populated at startup
the same way `exercises` already is.

The submit endpoint stores one row per quiz attempt:

```sql
CREATE TABLE IF NOT EXISTS quiz_submissions (
    id TEXT PRIMARY KEY,
    participant_id TEXT NOT NULL,
    quiz_slug TEXT NOT NULL,
    score INTEGER NOT NULL,            -- 0..=questions.len()
    total INTEGER NOT NULL,
    submitted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

A participant has "completed" a quiz when they submit any attempt with
`score == total`. This mirrors the existing `tests_passed` semantics for
exercises.

## Templates

Add `templates/quiz.html`. Key markup:

```html
{% for q in quiz.questions %}
<section class="quiz-question" data-id="{{ q.id }}">
    <div class="quiz-prompt">{{ q.prompt_html|safe }}</div>
    <ul class="quiz-options">
        {% for opt in q.options %}
        <li>
            <label>
                <input type="checkbox" data-correct="{{ opt.correct }}" />
                <span>{{ opt.label_html|safe }}</span>
            </label>
        </li>
        {% endfor %}
    </ul>
    <button class="quiz-check" type="button">Check</button>
    <div class="quiz-feedback" hidden></div>
    {% if let Some(html) = q.explanation_html %}
    <div class="quiz-explanation" hidden>{{ html|safe }}</div>
    {% endif %}
</section>
{% endfor %}
```

Client-side script (one ~50-line JS block, no framework):
1. On `Check`, mark each option as `correct` / `incorrect` / `missed`.
2. Reveal the explanation regardless of correctness.
3. Tally a running score. When all questions are answered, show a
   per-chapter summary card and POST it to `/api/quiz-submit`.

## Dashboard / chapter list

The bottom-of-exercise chapter list (the "dots" UI in `exercise.html`)
already reserves space for a special `is_quiz` row. Two changes:

1. Insert one quiz dot *after* every chapter that has a `quiz.md`,
   labelled "Chapter N quiz."
2. The dot reads its completion state from the new
   `quiz_submissions` table.

The dashboard's "completed exercises" stat keeps counting only real
exercises (the perfectionists shouldn't feel the goal post move). A
separate "quizzes passed" stat goes next to it.

## Authoring workflow

For each chapter, write a quiz of **3-7 questions** that:
- Tests recall of one specific concept the chapter introduced.
- Avoids trick questions and ambiguous wording.
- Includes at least one "this is the wrong way to do it" option (for
  sharpness) when it's natural.
- Provides an explanation that points back to the chapter prose, not
  just "the right answer is B."

A starter set:

| Chapter | Questions to ask about |
|---|---|
| 02 Strings | `len()` vs `.chars().count()`, `&str` vs `String`, what `to_string()` does |
| 03 Enums | exhaustiveness, `_` arms, when to add a new variant |
| 04 Vectors | `Vec` vs `&[T]`, `push` vs `insert`, indexing pitfalls |
| 05 HashMap | what `get` returns, when to reach for `entry` |
| 07 Option | `unwrap_or` vs `unwrap_or_else`, `if let` vs `match` |
| 08 Result | when `?` works, `'static` lifetimes for error messages |
| 09 Ownership | move vs borrow, `&` vs `&mut` borrowing rules |
| 10 Structs | `&self` / `&mut self` / `self`, what `Self` means |
| 11 Iterators | laziness, `iter` vs `into_iter`, `collect` type-hints |
| 13 `?` | when `?` requires a `From` impl, `Box<dyn Error>` |
| 14 Modules | `pub`, `pub(crate)`, `use` glob behaviour |

Chapters 01, 06, 12, 15-17 either have no compelling quiz material
(they're mostly project-shaped) or are the project chapters themselves.
Skip them; not every chapter needs one.

## Implementation order

1. Add `quizzes` module + `quiz.md` parser + tests on a single example
   chapter (write `examples/02_strings_and_chars/quiz.md` first).
2. Render `/quiz/{slug}` with no JS interactivity to verify the parse.
3. Layer on the answer-checking JS.
4. Add the migration + submit endpoint.
5. Wire the chapter-list dots and dashboard counters.
6. Author the remaining 10 quizzes.

Steps 1-5 are roughly a day of work. Step 6 is the bulk of the
remaining effort and can be parallelised across reviewers.
