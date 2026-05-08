# Design: In-Browser Exercises

## Goal

Today, doing an exercise requires cloning the repo and running
`cargo test --example NN_name`. We want a learner to be able to click an
exercise link and land on a page where they can read the prose, edit the
code, run the tests, and submit — all in the browser. The CLI workflow
remains supported but is no longer the entry point.

A non-goal: building a general Rust playground or sandbox. We delegate
execution to `play.rust-lang.org`.

## Page model

One page, single column, top to bottom:

1. **Header.** Breadcrumb, prev/next links, progress dots.
2. **Prose.** The exercise's `//!` doc comment, rendered as Markdown.
3. **Editor.** A CodeMirror 6 instance seeded with the entire
   `examples/NN_name.rs` file — function stubs, `///` doc comments, and
   `mod tests` all live here. The user is free to edit any of it.
4. **Buttons.** `Run tests`, and (when logged in) `Submit`. A small
   `Reset to starter` button that restores the original file contents.
5. **Output panel.** Empty until first run. Then renders test results
   (per-test pass/fail) and any compiler errors.

This is the "literate programming" shape: the file *is* the page.
Doc-comment becomes prose, code becomes the editable artifact, results
appear inline.

There is no separate "use your IDE" panel on each page. The course
intro (README / landing page) explains the CLI workflow once for
learners who prefer it.

## Routes

| Method | Path                          | Purpose                              |
| ------ | ----------------------------- | ------------------------------------ |
| GET    | `/exercise/:slug`             | Public exercise page (no progress).  |
| GET    | `/exercise/:ulid/:slug`       | Participant exercise page.           |
| POST   | `/api/run`                    | Proxy to Playground `/execute`.      |
| POST   | `/api/submit`                 | Existing endpoint, reused as-is.     |

The ULID-in-URL pattern matches the existing `/dashboard/:ulid`
convention. No cookies or sessions are introduced.

### Anonymous behaviour

On `/exercise/:slug`:

- `Run tests` works exactly the same as on the participant route.
- `Submit` is hidden. In its place, a small banner: *"Register to track
  your progress"* linking to `/`.

There is no localStorage persistence in v1. Refreshing the page returns
to starter code. `Reset to starter` is still useful for when a learner
has typed garbage and wants the original back without losing their
scroll position.

## Server-side data model

Exercises are scanned once at server startup. The dashboard's existing
ad-hoc doc-comment parser is replaced by this single source of truth.

```rust
struct Exercise {
    number: u8,            // 02
    slug: String,          // "strings_and_chars"
    file_stem: String,     // "02_strings_and_chars"
    title: String,         // first H1 of the //! block
    intro_html: String,    // rest of //! block, rendered to HTML at startup
    starter_code: String,  // verbatim file contents
}
```

The scan parses each `examples/*.rs` with `syn` to reliably extract
inner doc attributes (`//!`). The Markdown is rendered to HTML once,
at startup, with `pulldown-cmark` (default features only — no GFM
extensions until we actually use them).

`AppState` gains an `exercises: Arc<Vec<Exercise>>` field. No reload at
runtime; restart the server to pick up edits.

## Execution: `/api/run`

A thin proxy. Request:

```json
{ "slug": "02_strings_and_chars", "code": "..." }
```

The handler POSTs to `https://play.rust-lang.org/execute` with:

```json
{
  "channel": "stable",
  "mode": "debug",
  "edition": "2024",
  "crateType": "bin",
  "tests": true,
  "code": "<user's full source>"
}
```

It returns the upstream JSON to the browser, plus a parsed
`test_results: [{ name, passed }]` array derived from a small regex
over the `test … ok` / `test … FAILED` lines in stdout. If the
Playground returns a non-2xx (including 429), we forward the status
code so the browser can surface it directly.

We do not add our own rate limiting. The Playground enforces its own,
and that is sufficient for the expected traffic.

If the Playground is unreachable, the page still renders prose and
code; the user falls back to the CLI workflow (or simply waits).

## Submission

The browser's `Submit` button calls the existing `/api/submit` endpoint
with `{ ulid, exercise_name, source_code, tests_passed, clippy_passed,
fmt_passed }`. The browser parses the Playground output to determine
the flags. This is the same trust model as the CLI — the client reports
its own results — and it is acceptable on a self-directed learning
platform.

The existing `api_submit` already de-duplicates submissions by content
hash, so a learner mashing `Submit` twice on identical code is a no-op.

`Submit with formatting + lint` is a tickbox next to the button. When
checked, the browser additionally calls Playground's `/format` and
`/clippy` endpoints (via two more thin server proxies, mirroring
`/api/run`) and includes the results in the submission flags.

On successful submission the page reflects the new state (e.g. the
progress dot in the header turns filled).

## Editor

CodeMirror 6, loaded as ESM from a CDN, with the Rust language pack
for syntax highlighting. No completion, no LSP, no rust-analyzer-WASM.

No JS build pipeline is introduced. A single `<script type="module">`
in `templates/exercise.html` pulls CM6 and wires it to a `<textarea>`
fallback. Rationale: the exercises are short, errors come from the
Playground (not the editor), and a build step is significant new
maintenance burden for marginal UX gains.

If CDN reliability becomes a concern later, we vendor a pre-built CM6
bundle into `static/`. That is a one-day migration, deferred until
needed.

## Dashboard changes

Each exercise row becomes a link to `/exercise/:ulid/:slug`. The
existing expand-in-place `<details>` (with description text and
submission history) is removed. The exercise page itself is now the
home for that information; submission history may be added there in a
later PR.

## Out of scope

- Multi-file exercises.
- Self-hosted compile fallback.
- rust-analyzer-WASM / in-editor completion.
- Cookie/session authentication.
- A public exercise index page.
- Submission history on the exercise page (deferred).
- Server-side re-execution on submit (we trust the client).
- A general literate-Markdown authoring tool. The `.rs` file remains
  the source of truth. A future, separate project may build a Markdown-
  first authoring tool reusable across this course and the blog; the
  exercise page will keep working unchanged whichever side authors live
  on, since it consumes a parsed `Exercise` struct.

## Risks

- **Playground availability.** If `play.rust-lang.org` is down,
  `Run tests` and `Submit` are down. The page still renders the prose
  and the editor; the CLI fallback is always available. Degraded, not
  broken.
- **Playground API stability.** Not a versioned public API. We pin the
  request shape and centralise it in one handler, so changes are a
  one-place fix.
- **CM6 via CDN.** A third-party network dependency for a learner-
  facing page. Acceptable for v1; vendoring the bundle is the escape
  hatch.

## Delivery plan

Each step leaves `main` deployable.

### PR 1 — Exercise scan and parser ✅

- Add `src/exercises.rs` with the `Exercise` struct.
- Parse `examples/*.rs` with `syn` at startup. Render `intro_html` with
  `pulldown-cmark`.
- Store `Arc<Vec<Exercise>>` in `AppState`.
- Replace the dashboard's existing parser (`parse_exercise_docs`,
  `get_exercise_progress`'s filesystem scan) with lookups against
  `AppState::exercises`.
- Unit-test the parser against `02_strings_and_chars.rs`.

No new routes yet. The dashboard renders identically to today, just
sourced from the new model.

### PR 2 — Read-only exercise page + dashboard links ✅

- Add `templates/exercise.html` extending `base.html`.
- Add `GET /exercise/:slug` and `GET /exercise/:ulid/:slug`. Both
  render prose + a read-only `<pre><code>` of the starter code, with a
  Copy button. No editor, no buttons yet.
- Update `dashboard.html`: each exercise row becomes a link to
  `/exercise/:ulid/:slug`. The `<details>` expand-in-place is removed.

### PR 3 — CodeMirror 6 editor ✅

- Replace the `<pre>` with a CM6 editor seeded with `starter_code`.
- Add the `Reset to starter` button.
- Rust language pack, theme matching the site palette, lazy-loaded
  after first paint.

### PR 4 — `Run tests` + output panel ✅

- Add `POST /api/run` proxy to Playground `/execute`.
- Wire the `Run tests` button to it.
- Render the output panel: per-test pass/fail with stderr collapsed by
  default.

### PR 5 — `Submit` and `Submit with formatting + lint`

- Show `Submit` only on the participant route.
- On click: parse the run results, then POST to existing `/api/submit`.
- Add the `with formatting + lint` tickbox; when set, also proxy
  `/format` and `/clippy` and include their results.
- Update header progress dots on success.

### PR 6 — Header navigation

- Prev/next links in the header.
- Progress dots reflecting the current participant's state.

### Polish (separate, not blocking)

- `Cmd/Ctrl+Enter` runs tests; `Cmd/Ctrl+S` submits.
- Public route gets a "register to track progress" banner.
- Dark/light theme parity for the editor.
- Quiz page wrapped in the same layout for visual consistency.
- Submission history surfaced on the exercise page.
