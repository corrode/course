//! Startup-time scan and parse of `examples/NN_slug/` chapter directories.
//!
//! Each chapter is a directory `examples/NN_slug/` containing one or more
//! exercise files (`.rs`) and zero or more notes (`.md`), interleaved by a
//! leading `<n>_` ordering prefix. The chapter is exposed to the rest of
//! the application as an ordered list of [`Step`]s, where each step is
//! either a [`Note`] (prose) or a [`CodeStep`] (an exercise with its own
//! editor and tests).
//!
//! Two shapes are supported:
//!
//! * **Single-step (legacy):** only a `main.rs` and (optionally) one or
//!   more `<n>_<slug>.md` notes. The chapter is treated as a single code
//!   step whose source is `main.rs`; notes render before its prose.
//! * **Multi-step:** sibling `<n>_<slug>.rs` files alongside (or instead
//!   of) `main.rs`. Each `.rs` file becomes a `CodeStep`, ordered with
//!   the notes by its leading number. A generated `main.rs` (built by
//!   `build.rs`) aggregates the step files as `mod _N_slug;` so
//!   `cargo test --example <chapter>` still runs every step's tests.
//!
//! See `docs/multi_step_chapters_plan.md` for the rollout plan.

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A short prose note that lives next to an exercise. Notes give us a place
/// to put preliminary information ("solve with std only", section
/// preambles, etc.) without bloating the exercise's own `//!` block.
#[derive(Debug, Clone)]
pub struct Note {
    /// In-chapter ordering taken from the leading `<n>_` of the filename.
    pub order: u8,
    /// Slug after the leading number, e.g. `intro` for `1_intro.md`.
    pub slug: String,
    /// First H1 of the note, falling back to the slug.
    pub title: String,
    /// Rendered HTML body (everything after the H1).
    pub html: String,
}

/// An exercise file: prose + starter code + tests, rendered as one
/// editable section on the web.
#[derive(Debug, Clone)]
pub struct CodeStep {
    /// In-chapter ordering taken from the leading `<n>_` of the filename.
    /// Single-step (legacy) chapters use order `0`.
    pub order: u8,
    /// Slug after the leading number, e.g. `unwrap` for `2_unwrap.rs`.
    /// For the legacy single-step shape, this is the chapter slug itself.
    pub slug: String,
    /// Step title. Source priority: paired `<N>_<slug>.md` H1 (if it
    /// exists) > first H1 in the file's `//!` block > slug. Used as
    /// the section heading above the editor when there is no paired
    /// note rendered immediately above.
    pub title: String,
    /// File contents with the `//!` block stripped, used as the editor's
    /// starter content.
    pub starter_code: String,
    /// Rendered HTML for this step's slice of `hints.md`, if the chapter
    /// has a hints note with a `## <slug>` subsection matching this
    /// step. Wired up in [`parse_chapter`] after both the steps and the
    /// hints note have been parsed. `None` means "no per-step hint":
    /// either the chapter has no hints at all, or the hints file has no
    /// section matching this step's slug."
    pub hints_html: Option<String>,
}

impl CodeStep {
    /// Stable identifier for a step within its chapter, used in URLs and
    /// in `submissions.exercise_name` as `<chapter>/<step_key>`.
    ///
    /// For multi-step chapters this is `<order>_<slug>` (e.g. `2_unwrap`)
    /// so the on-disk filename is recoverable from the key. For the
    /// legacy single-step shape the key is the empty string, so the
    /// chapter slug alone identifies the step.
    #[must_use]
    pub fn key(&self) -> String {
        if self.order == 0 {
            String::new()
        } else {
            format!("{}_{}", self.order, self.slug)
        }
    }
}

/// One position in a chapter's ordered sequence of content: either a
/// prose note or a code step.
#[derive(Debug, Clone)]
pub enum Step {
    /// A markdown note rendered as-is.
    Prose(Note),
    /// An exercise file with its own editor and tests.
    Code(CodeStep),
}

impl Step {
    /// Ordering key (the leading `<n>_` prefix of the source filename).
    #[must_use]
    pub const fn order(&self) -> u8 {
        match self {
            Self::Prose(n) => n.order,
            Self::Code(c) => c.order,
        }
    }
}

/// Optional per-chapter UI customisation, loaded from
/// `examples/<chapter>/.chapter.toml` if that file exists. The struct
/// is intentionally small and additive: unknown keys are rejected (so
/// typos surface), and every field has a sensible default so omitting
/// the file is the same as having an empty one.
///
/// The whole struct is serialised to JSON and emitted on the exercise
/// page root as `data-corrode-config`. Adding a new knob therefore
/// requires only:
///   1. a field here (with `#[serde(default)]`),
///   2. a handler in the JS `applyChapterDirectives()` registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct ChapterDirectives {
    /// Allow-list of buttons to keep visible in every code section.
    /// `None` (the field is omitted) means "show every button" — the
    /// default for any chapter without a `.chapter.toml`. An explicit
    /// list (including the empty list) hides any button not named in
    /// it. See the `Buttons` map in `exercise.html` for valid names
    /// (`run`, `submit`, `format`, `reset`, `copy`, `vim`, `vscode`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<Vec<String>>,
    /// Whether the page-wide table of contents (the chapter list at
    /// the bottom of the page) is rendered. `None` = default visible.
    /// `Some(false)` hides it; `Some(true)` is identical to `None` and
    /// exists only for explicitness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_toc: Option<bool>,
    /// Names of effects to fire when a run finishes successfully.
    /// See the `PassHooks` map in `exercise.html` for valid names
    /// (`confetti`, ...).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub on_pass: Vec<String>,
    /// When set on a chapter rendered anonymously, the inline signup
    /// card embedded in `exercise.html` is revealed the moment a code
    /// section passes its tests, and the next-chapter button is gated
    /// behind it. Used on the welcome chapter so a brand-new visitor
    /// signs up after their very first successful run, before moving
    /// on. `None` / `Some(false)` is the default (no inline signup).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_on_pass: Option<bool>,
}

/// A single chapter, parsed from one `examples/NN_slug/` directory.
#[derive(Debug, Clone)]
pub struct Exercise {
    /// Numeric prefix from the directory name, e.g. `2` for `02_strings_and_chars`.
    pub number: u8,
    /// Slug without the numeric prefix, e.g. `strings_and_chars`.
    pub slug: String,
    /// Directory name including the prefix, e.g. `02_strings_and_chars`.
    /// Doubles as the chapter half of `submissions.exercise_name`.
    pub file_stem: String,
    /// Chapter title, taken from the first code step's `//!` H1 (legacy
    /// shape) or from a top-level `0_chapter.md` (future). Falls back to
    /// `file_stem`.
    pub title: String,
    /// Ordered prose + code steps as they should render top-to-bottom.
    pub steps: Vec<Step>,
    /// Optional spoiler-protected hints, surfaced once per chapter as a
    /// closed `<details>` below the last step. Sourced from a sibling
    /// note whose slug is exactly `hints`.
    pub hints: Option<Note>,
    /// Per-chapter UI tweaks loaded from `.chapter.toml` (if present).
    /// Empty/default when the file is absent.
    pub directives: ChapterDirectives,
}

impl Exercise {
    /// True for chapters whose slug contains "quiz".
    #[must_use]
    pub fn is_quiz(&self) -> bool {
        self.slug.contains("quiz")
    }

    /// All code steps in render order. Useful for progress calculations.
    #[must_use]
    pub fn code_steps(&self) -> Vec<&CodeStep> {
        self.steps
            .iter()
            .filter_map(|s| match s {
                Step::Code(c) => Some(c),
                Step::Prose(_) => None,
            })
            .collect()
    }

    /// Prose-only notes, in render order. Convenience for legacy callers.
    #[must_use]
    pub fn notes(&self) -> Vec<&Note> {
        self.steps
            .iter()
            .filter_map(|s| match s {
                Step::Prose(n) => Some(n),
                Step::Code(_) => None,
            })
            .collect()
    }

    /// The single code step for legacy single-step chapters. Returns
    /// the first code step for multi-step chapters; callers that need
    /// to render every step should use [`Self::code_steps`] instead.
    #[must_use]
    pub fn primary_step(&self) -> Option<&CodeStep> {
        self.code_steps().into_iter().next()
    }

    /// True if the chapter has more than one code step.
    #[must_use]
    pub fn is_multi_step(&self) -> bool {
        self.code_steps().len() > 1
    }

    /// Serialise this chapter's directives as a compact JSON string,
    /// suitable for embedding in a `data-corrode-config` attribute.
    /// Falls back to `"{}"` if serialisation somehow fails so the
    /// template never panics.
    #[must_use]
    pub fn directives_json(&self) -> String {
        serde_json::to_string(&self.directives).unwrap_or_else(|_| "{}".into())
    }

    /// Convenience for templates: did this chapter opt into the inline
    /// signup-on-pass card? Treats `Some(true)` as enabled and
    /// everything else (missing field, `Some(false)`) as disabled.
    #[must_use]
    pub fn wants_signup_on_pass(&self) -> bool {
        self.directives.signup_on_pass.unwrap_or(false)
    }
}

/// One position in a chapter's rendered output: prose or an editable
/// code section. Built per-request by the server from a chapter's
/// `steps` plus the participant's progress; rendered by the
/// `exercise.html` template.
///
/// Lives in this module (rather than in the server binary) so that
/// `templates/exercise.html` can pattern-match on it through Askama's
/// fully-qualified path syntax.
#[derive(Debug, Clone)]
pub struct RenderItem {
    pub kind: RenderKind,
}

/// Either a prose block or a code section with all the per-step state
/// the editor JS needs.
#[derive(Debug, Clone)]
pub enum RenderKind {
    /// A markdown note rendered as raw HTML.
    Prose { html: String },
    /// An editable code step. Carries every per-step value the template
    /// needs so the editor JS can stay scoped to one `<section>`.
    Code {
        /// Stable per-step id used for DOM ids and the draft
        /// localStorage key. Equal to the chapter `file_stem` for
        /// legacy single-step chapters, or
        /// `<chapter_file_stem>__<n>_<step_slug>` for multi-step.
        dom_id: String,
        /// Full `submissions.exercise_name` value the editor will
        /// post back. `<chapter>` for legacy, `<chapter>/<step_key>`
        /// for multi-step.
        exercise_key: String,
        /// Section eyebrow text, e.g. `"Exercise"` or
        /// `"Exercise 2 of 4"`.
        eyebrow: String,
        /// Step title shown above the editor. Suppressed by the
        /// template when `show_title` is false, which happens when an
        /// immediately-preceding note already shows the same heading.
        title: String,
        /// Whether the template should render `title` above the editor.
        /// False whenever a paired note is rendered directly above this
        /// step (its H2 acts as the section heading instead, avoiding a
        /// duplicate).
        show_title: bool,
        /// Editor starter content (file with `//!` stripped).
        starter_code: String,
        /// True if at least one submission exists for this step.
        attempted: bool,
        /// True if a submission has `tests_passed`.
        completed: bool,
        /// True if a submission has tests + fmt + clippy all green.
        perfected: bool,
        /// github.dev URL pointing at the right file in this chapter.
        github_dev_url: String,
        /// Optional rendered HTML for this step's slice of `hints.md`,
        /// surfaced as an inline `<details>` immediately under the
        /// editor section.
        hints_html: Option<String>,
    },
}

/// Scan a directory for `NN_slug/` chapter dirs and parse each one.
///
/// Directories whose name does not match `^\d+_` are skipped. Chapters
/// missing a `main.rs` are logged and skipped, so a single malformed
/// chapter does not take down the whole server.
pub fn scan_dir(dir: &Path) -> Result<Vec<Exercise>> {
    if !dir.exists() {
        return Err(anyhow!("Examples directory not found: {}", dir.display()));
    }

    let mut chapter_dirs: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.chars().next().is_some_and(|c| c.is_ascii_digit()))
        })
        .collect();

    chapter_dirs.sort();

    let mut out = Vec::with_capacity(chapter_dirs.len());
    for chapter_dir in chapter_dirs {
        match parse_chapter(&chapter_dir) {
            Ok(ex) => out.push(ex),
            Err(e) => {
                log::warn!("Skipping {}: {e:#}", chapter_dir.display());
            }
        }
    }
    Ok(out)
}

/// Convenience: scan and wrap in `Arc` for `AppState`.
pub fn load(dir: &Path) -> Result<Arc<Vec<Exercise>>> {
    Ok(Arc::new(scan_dir(dir)?))
}

fn parse_chapter(dir: &Path) -> Result<Exercise> {
    let file_stem = dir
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("invalid directory name"))?
        .to_string();

    let (number, slug) = split_numeric_prefix(&file_stem)
        .ok_or_else(|| anyhow!("directory does not start with NN_: {file_stem}"))?;

    // Discover sibling step files (`<n>_<slug>.rs`) before deciding the
    // shape. The presence of any such file flips us into multi-step mode,
    // even if there's also a `main.rs` (which in that case is generated).
    let step_files = scan_step_files(dir)?;
    let mut notes = scan_notes(dir)?;
    let hints = notes
        .iter()
        .position(|n| n.slug == "hints")
        .map(|i| notes.remove(i));
    let hints_path = hints.as_ref().and_then(|_| find_hints_path(dir));

    let (steps, title) = if step_files.is_empty() {
        // No step files. Two cases:
        //   * Legacy single-step chapter with `main.rs`. Parse it.
        //   * Notes-only chapter (appendix): no `main.rs`, just prose.
        let main_rs = dir.join("main.rs");
        if main_rs.exists() {
            let (step, step_title) = parse_code_file(&main_rs, 0, &slug, &file_stem)?;
            let chapter_title = notes.first().map(|n| n.title.clone()).unwrap_or(step_title);
            let mut steps: Vec<Step> = notes.into_iter().map(Step::Prose).collect();
            steps.push(Step::Code(step));
            (steps, chapter_title)
        } else if !notes.is_empty() {
            let chapter_title = notes
                .first()
                .map(|n| n.title.clone())
                .unwrap_or_else(|| file_stem.clone());
            let steps: Vec<Step> = notes.into_iter().map(Step::Prose).collect();
            (steps, chapter_title)
        } else {
            return Err(anyhow!(
                "chapter {} has neither code steps nor notes",
                dir.display()
            ));
        }
    } else {
        // Multi-step chapter: interleave prose and code by order.
        let mut code_steps = Vec::with_capacity(step_files.len());
        let mut step_title_fallback: Option<String> = None;
        for (order, path, step_slug) in step_files {
            let (step, step_title) = parse_code_file(&path, order, &step_slug, &file_stem)?;
            if step_title_fallback.is_none() {
                step_title_fallback = Some(step_title);
            }
            code_steps.push(step);
        }

        let mut combined: Vec<Step> = Vec::with_capacity(notes.len() + code_steps.len());
        combined.extend(notes.iter().cloned().map(Step::Prose));
        combined.extend(code_steps.into_iter().map(Step::Code));
        combined.sort_by_key(Step::order);

        // Title priority for multi-step: first note H1 > first step H1
        // > file_stem.
        let chapter_title = notes
            .first()
            .map(|n| n.title.clone())
            .or(step_title_fallback)
            .unwrap_or_else(|| file_stem.clone());

        (combined, chapter_title)
    };

    let (steps, hints) = apply_hints(steps, hints, hints_path.as_deref());

    let directives = load_chapter_directives(dir);

    Ok(Exercise {
        number,
        slug,
        file_stem,
        title,
        steps,
        hints,
        directives,
    })
}

/// Read `.chapter.toml` from a chapter directory if present.
///
/// A missing file is not an error — it just yields the default
/// (empty) directives. A malformed file is logged and ignored so a
/// typo in one chapter doesn't take down the whole server.
fn load_chapter_directives(dir: &Path) -> ChapterDirectives {
    let path = dir.join(".chapter.toml");
    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return ChapterDirectives::default(),
        Err(e) => {
            log::warn!("reading {}: {e}", path.display());
            return ChapterDirectives::default();
        }
    };
    match toml::from_str::<ChapterDirectives>(&raw) {
        Ok(d) => d,
        Err(e) => {
            log::warn!("parsing {}: {e}", path.display());
            ChapterDirectives::default()
        }
    }
}

/// Distribute the chapter's `hints.md` over its code steps and rebuild
/// the chapter-wide leftover block in one pass.
///
/// Sections whose heading contains a backticked token matching a code
/// step's slug are attached to that step. For example,
/// `` ## `quoted_line`, the state machine `` matches the `quoted_line`
/// step. Anything that
/// doesn't match stays in the chapter-wide leftover, which falls back
/// to the old "all hints in one block at the bottom" behaviour.
///
/// On any I/O failure we log and pass through unchanged.
fn apply_hints(
    mut steps: Vec<Step>,
    original: Option<Note>,
    hints_path: Option<&Path>,
) -> (Vec<Step>, Option<Note>) {
    let Some(path) = hints_path else {
        return (steps, original);
    };
    let Some(mut note) = original else {
        return (steps, None);
    };
    let md = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            log::warn!(
                "reading {} for per-step split failed: {e:#}",
                path.display()
            );
            return (steps, Some(note));
        }
    };
    // Drop the H1 the same way `parse_note` does so titles aren't doubled.
    let (_title, body_md) = split_title(&md);
    let (intro, sections) = split_hints_markdown(&body_md);
    if sections.is_empty() {
        return (steps, Some(note));
    }

    let mut consumed = vec![false; sections.len()];
    for step in &mut steps {
        let Step::Code(code) = step else { continue };
        if let Some((sec_idx, (_, body))) = sections
            .iter()
            .enumerate()
            .find(|(i, (heading, _))| !consumed[*i] && heading_matches_slug(heading, &code.slug))
        {
            code.hints_html = Some(render_markdown(body));
            consumed[sec_idx] = true;
        }
    }

    // Rebuild the leftover block from the intro plus any unconsumed
    // sections. If everything got consumed and the intro is just
    // whitespace, drop the bottom block entirely.
    let mut leftover = intro.trim_end().to_string();
    for (i, (heading, body)) in sections.iter().enumerate() {
        if consumed[i] {
            continue;
        }
        if !leftover.is_empty() && !leftover.ends_with("\n\n") {
            leftover.push_str("\n\n");
        }
        leftover.push_str("## ");
        leftover.push_str(heading);
        leftover.push('\n');
        leftover.push_str(body);
    }

    if leftover.trim().is_empty() {
        (steps, None)
    } else {
        note.html = render_markdown(&leftover);
        (steps, Some(note))
    }
}

/// Locate the `<n>_hints.md` file in a chapter dir, if present.
fn find_hints_path(chapter_dir: &Path) -> Option<PathBuf> {
    std::fs::read_dir(chapter_dir)
        .ok()?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .find(|p| {
            p.extension().and_then(|s| s.to_str()) == Some("md")
                && p.file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|s| split_numeric_prefix(s).map(|(_, slug)| slug))
                    .as_deref()
                    == Some("hints")
        })
}

/// Split a `hints.md` markdown source into an intro chunk (everything
/// before the first `## ` heading) and a sequence of `(heading_text,
/// body)` sections. Heading text is returned verbatim minus the
/// trailing newline so it can be inspected by [`heading_matches_slug`].
///
/// This is a deliberately tiny line-based scan rather than a full
/// markdown re-parse: the hints files only ever use H2 dividers, and
/// keeping the splitter dumb means we can hand each section back to
/// [`render_markdown`] unchanged.
fn split_hints_markdown(md: &str) -> (String, Vec<(String, String)>) {
    let mut intro = String::new();
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current: Option<(String, String)> = None;
    for line in md.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("## ") {
            if let Some(prev) = current.take() {
                sections.push(prev);
            }
            let heading = rest.trim_end_matches(['\n', '\r']).to_string();
            current = Some((heading, String::new()));
        } else if let Some((_, body)) = current.as_mut() {
            body.push_str(line);
        } else {
            intro.push_str(line);
        }
    }
    if let Some(prev) = current.take() {
        sections.push(prev);
    }
    (intro, sections)
}

/// Match a hints H2 heading against a step slug. The first backticked
/// token in the heading (if any) is treated as the key, otherwise the
/// whole heading text is used. This lets authors write descriptive
/// headings like `` `quoted_line`, the state machine `` while still
/// keying off the file slug.
fn heading_matches_slug(heading: &str, step_slug: &str) -> bool {
    let key = if let Some(start) = heading.find('`') {
        let after = &heading[start + 1..];
        match after.find('`') {
            Some(end) => &after[..end],
            None => heading.trim(),
        }
    } else {
        heading.trim()
    };
    key.eq_ignore_ascii_case(step_slug)
}

/// Parse a single `.rs` file (`main.rs` or a step file) into a `CodeStep`.
/// Returns the step plus the title we parsed from any leading `//!`
/// block (used as a fallback when there is no paired note).
fn parse_code_file(
    path: &Path,
    order: u8,
    slug: &str,
    fallback_title: &str,
) -> Result<(CodeStep, String)> {
    let starter_code_full =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let intro_md = extract_inner_doc(&starter_code_full)?;
    let (title_opt, _body_md) = split_title(&intro_md);
    let title = title_opt.unwrap_or_else(|| fallback_title.to_string());
    let starter_code = trim_trailing_blank_lines(&strip_inner_doc(&starter_code_full));

    Ok((
        CodeStep {
            order,
            slug: slug.to_string(),
            title: title.clone(),
            starter_code,
            hints_html: None,
        },
        title,
    ))
}

/// Collapse trailing whitespace-only lines down to a single newline so
/// the editor doesn't show empty padding below short step files.
fn trim_trailing_blank_lines(s: &str) -> String {
    let trimmed_end = s.trim_end_matches(|c: char| c == '\n' || c == '\r' || c == ' ' || c == '\t');
    let mut out = String::with_capacity(trimmed_end.len() + 1);
    out.push_str(trimmed_end);
    out.push('\n');
    out
}

/// Discover `<n>_<slug>.rs` step files in a chapter dir.
///
/// `main.rs` is excluded: in multi-step chapters it's an auto-generated
/// aggregator (`build.rs`), and in single-step chapters it's the only
/// file and is handled separately by [`parse_chapter`].
fn scan_step_files(chapter_dir: &Path) -> Result<Vec<(u8, PathBuf, String)>> {
    let mut out: Vec<(u8, PathBuf, String)> = std::fs::read_dir(chapter_dir)
        .with_context(|| format!("reading {}", chapter_dir.display()))?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"))
        .filter(|p| p.file_name().and_then(|s| s.to_str()) != Some("main.rs"))
        .filter_map(|p| {
            let stem = p.file_stem().and_then(|s| s.to_str())?.to_string();
            let (n, slug) = split_numeric_prefix(&stem)?;
            Some((n, p, slug))
        })
        .collect();
    out.sort_by_key(|(n, _, _)| *n);
    Ok(out)
}

/// Find `<n>_<slug>.md` notes in a chapter dir and parse each into a
/// `Note`. Sorted by the leading number so chapter authors can
/// interleave notes with code steps in any order they like.
fn scan_notes(chapter_dir: &Path) -> Result<Vec<Note>> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(chapter_dir)
        .with_context(|| format!("reading {}", chapter_dir.display()))?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .filter(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.chars().next().is_some_and(|c| c.is_ascii_digit()))
        })
        .collect();
    paths.sort();

    let mut notes = Vec::with_capacity(paths.len());
    for path in paths {
        match parse_note(&path) {
            Ok(n) => notes.push(n),
            Err(e) => log::warn!("Skipping note {}: {e:#}", path.display()),
        }
    }
    Ok(notes)
}

fn parse_note(path: &Path) -> Result<Note> {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("invalid note filename"))?
        .to_string();
    let (order, slug) = split_numeric_prefix(&stem)
        .ok_or_else(|| anyhow!("note does not start with N_: {stem}"))?;

    let md =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    // The leading H1 is consumed as the note's `title`; the rendered
    // HTML body excludes it. Chapter pages render the title separately
    // (as the chapter heading for the first note, or as a section
    // heading for subsequent notes) so the H1 doesn't appear twice.
    let (title_opt, body_md) = split_title(&md);
    let title = title_opt.unwrap_or_else(|| slug.clone());
    let html = render_markdown(&body_md);

    Ok(Note {
        order,
        slug,
        title,
        html,
    })
}

/// Drop the leading run of inner doc-comment lines (`//!` or `/*! */`) plus
/// any blank lines that follow, leaving the rest of the source intact.
fn strip_inner_doc(source: &str) -> String {
    let mut out: Vec<&str> = Vec::with_capacity(source.lines().count());
    let mut header = true;
    let mut in_block = false;
    for line in source.lines() {
        if header {
            let trimmed = line.trim_start();
            if in_block {
                if trimmed.contains("*/") {
                    in_block = false;
                }
                continue;
            }
            if trimmed.starts_with("//!") {
                continue;
            }
            if trimmed.starts_with("/*!") {
                if !trimmed.contains("*/") {
                    in_block = true;
                }
                continue;
            }
            if trimmed.is_empty() {
                // Skip blank lines that sat between or after the doc block.
                continue;
            }
            header = false;
        }
        out.push(line);
    }
    let mut joined = out.join("\n");
    if source.ends_with('\n') && !joined.ends_with('\n') {
        joined.push('\n');
    }
    joined
}

/// Split `02_strings_and_chars` into `(2, "strings_and_chars")`.
fn split_numeric_prefix(stem: &str) -> Option<(u8, String)> {
    let (num, rest) = stem.split_once('_')?;
    let n: u8 = num.parse().ok()?;
    Some((n, rest.to_string()))
}

/// Use `syn` to extract the file's inner doc attributes (`//!` and
/// `/*! */`) joined into a single Markdown string.
fn extract_inner_doc(source: &str) -> Result<String> {
    let file: syn::File = syn::parse_file(source).context("parsing file with syn")?;

    let mut lines: Vec<String> = Vec::new();
    for attr in &file.attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        if let syn::Meta::NameValue(nv) = &attr.meta {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &nv.value
            {
                // syn gives us the doc string with the leading space stripped
                // by rustc, but keep it verbatim. Markdown handles whitespace.
                let raw = s.value();
                // Each `//! foo` becomes a single line; strip a single leading
                // space if present (matches the convention in our examples).
                let trimmed = raw.strip_prefix(' ').unwrap_or(&raw);
                lines.push(trimmed.to_string());
            }
        }
    }

    Ok(lines.join("\n"))
}

/// If the first non-empty line is a `# Heading`, peel it off as the title
/// and return the rest as the body Markdown.
fn split_title(md: &str) -> (Option<String>, String) {
    let mut lines = md.lines();
    let mut title = None;
    let mut consumed = 0usize;

    for line in lines.by_ref() {
        consumed += line.len() + 1;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(rest) = line.trim_start().strip_prefix("# ") {
            title = Some(rest.trim().to_string());
        }
        break;
    }

    let body = if title.is_some() && consumed <= md.len() {
        md[consumed.min(md.len())..].to_string()
    } else {
        md.to_string()
    };
    (title, body)
}

pub fn render_markdown(md: &str) -> String {
    use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, html};
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    // GFM unlocks GitHub-style alerts: `> [!NOTE]`, `> [!TIP]`, etc.
    // pulldown-cmark recognizes them as `BlockQuote(Some(BlockQuoteKind))`,
    // and the default HTML renderer wraps them in `<blockquote class="markdown-alert-note">`.
    opts.insert(Options::ENABLE_GFM);

    // Rewrite outbound links so they open in a new tab. Anything with an
    // `http(s)://` scheme is treated as external; relative links and
    // intra-doc anchors are left alone. CommonMark forbids nested links,
    // so a single bool is enough state to pair Start ↔ End.
    let mut in_external = false;
    let parser = Parser::new_ext(md, opts).map(move |ev| match ev {
        Event::Start(Tag::Link {
            ref dest_url,
            ref title,
            ..
        }) if is_external(dest_url) => {
            in_external = true;
            Event::Html(
                format!(
                    "<a href=\"{href}\" title=\"{title}\" class=\"external-link\" target=\"_blank\" rel=\"noopener noreferrer\">",
                    href = escape_attr(dest_url),
                    title = escape_attr(title),
                )
                .into(),
            )
        }
        Event::End(TagEnd::Link) if in_external => {
            in_external = false;
            Event::Html("</a>".into())
        }
        other => other,
    });

    let mut out = String::with_capacity(md.len() * 2);
    html::push_html(&mut out, parser);
    wrap_alert_sections(&out)
}

/// Headings whose body (the immediately-following `<ul>...</ul>`) should
/// be wrapped in a NOTE-style blockquote. The H2 stays inside the
/// blockquote and acts as the label, so all three render with the same
/// alert chrome that pulldown-cmark emits for `> [!NOTE]`.
const ALERT_HEADINGS: &[&str] = &[
    "<h2>Useful from the standard library</h2>",
    "<h2>Where to look things up</h2>",
    "<h2>What we learned</h2>",
];

/// Wrap every recognized H2-plus-following-`<ul>` block in the same
/// `<blockquote class="markdown-alert-note">` markup that
/// pulldown-cmark emits for `> [!NOTE]`. The H2 stays inside the
/// blockquote and acts as the label (the alert CSS hides its
/// auto-injected "Note" cap when an H2 is the first child).
fn wrap_alert_sections(html: &str) -> String {
    let mut current = html.to_string();
    for heading in ALERT_HEADINGS {
        current = wrap_one_alert_section(&current, heading);
    }
    current
}

fn wrap_one_alert_section(html: &str, heading: &str) -> String {
    let Some(h_start) = html.find(heading) else {
        return html.to_string();
    };
    // Find the matching closing `</ul>` after the heading. The list may
    // contain nested HTML, but pulldown-cmark doesn't emit nested `<ul>`
    // here (each item is its own `<li>` with no sub-list), so the next
    // `</ul>` is the one we want.
    let after_h = h_start + heading.len();
    let rest = &html[after_h..];
    // Skip whitespace, expect `<ul>`.
    let ul_offset = rest
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| i);
    let Some(ul_off) = ul_offset else {
        return html.to_string();
    };
    if !rest[ul_off..].starts_with("<ul>") {
        return html.to_string();
    }
    let Some(ul_end_rel) = rest[ul_off..].find("</ul>") else {
        return html.to_string();
    };
    let ul_end_abs = after_h + ul_off + ul_end_rel + "</ul>".len();

    let mut out = String::with_capacity(html.len() + 64);
    out.push_str(&html[..h_start]);
    out.push_str("<blockquote class=\"markdown-alert-note\">\n");
    out.push_str(&html[h_start..ul_end_abs]);
    out.push_str("\n</blockquote>");
    out.push_str(&html[ul_end_abs..]);
    out
}

fn is_external(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Minimal HTML attribute escaper for the small set of values we emit.
fn escape_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_real_examples_dir() {
        let exercises =
            scan_dir(Path::new("examples")).expect("examples dir should exist when running tests");
        assert!(!exercises.is_empty(), "expected at least one exercise");
        let strings = exercises
            .iter()
            .find(|e| e.slug == "strings_and_chars")
            .expect("expected 02_strings_and_chars to be present");
        assert_eq!(strings.number, 2);
        assert_eq!(strings.file_stem, "02_strings_and_chars");
        // Title now comes from the first code step's `# Title` (multi-step
        // chapter). The chapter intro lives in `1_intro.md` (a Note).
        let primary = strings
            .primary_step()
            .expect("chapter should have at least one code step");
        // The borrowed/owned table now lives in the chapter-level note.
        let notes = strings.notes();
        let intro_note = notes
            .iter()
            .find(|n| n.slug == "intro")
            .expect("chapter should have a 1_intro.md note");
        assert!(
            intro_note.html.contains("<p>"),
            "intro note should be rendered HTML"
        );
        assert!(
            primary.starter_code.contains("fn format_welcome_message"),
            "first step's starter code should contain its function stub, got: {}",
            primary.starter_code
        );
        assert!(
            !primary.starter_code.contains("//!"),
            "editor starter code should have the inner doc comment stripped"
        );
        assert!(
            primary.starter_code.starts_with("///")
                || primary.starter_code.starts_with("fn")
                || primary.starter_code.starts_with("use "),
            "starter code should begin with code, not blank lines: {:?}",
            &primary.starter_code[..40.min(primary.starter_code.len())]
        );
    }

    #[test]
    fn discovers_chapter_notes() {
        // 00_hello_rust ships with an introductory note (added in the
        // same migration as the chapter-directory layout).
        let exercises =
            scan_dir(Path::new("examples")).expect("examples dir should exist when running tests");
        let chapter = exercises
            .iter()
            .find(|e| e.slug == "hello_rust")
            .expect("expected 00_hello_rust to be present");
        let notes = chapter.notes();
        assert!(
            !notes.is_empty(),
            "expected at least one note in 00_hello_rust"
        );
        let first = notes[0];
        assert_eq!(first.order, 1);
        assert!(
            first.html.contains("<p>"),
            "note should render to HTML, got: {}",
            first.html
        );
        // Note: chapter 0's intro is intentionally link-free so we no
        // longer assert on external-link rewriting here. That logic is
        // still covered indirectly by the markdown rendering tests in
        // chapters that do link out (e.g. 02_strings_and_chars).
    }

    #[test]
    fn multi_step_chapter_exposes_each_step() {
        // 07_option is the pilot multi-step chapter (four step files).
        let exercises =
            scan_dir(Path::new("examples")).expect("examples dir should exist when running tests");
        let chapter = exercises
            .iter()
            .find(|e| e.slug == "option")
            .expect("expected 07_option to be present");
        let code_steps = chapter.code_steps();
        assert!(
            code_steps.len() >= 2,
            "expected multiple code steps, got {}",
            code_steps.len()
        );
        assert!(
            chapter.is_multi_step(),
            "chapter with sibling .rs files should report multi-step"
        );
        // Step keys must be `<n>_<slug>` so the DB key becomes
        // `07_option/<n>_<slug>`.
        let first = code_steps[0];
        assert_eq!(first.order, 2, "first step should be ordered 2_*");
        assert!(first.key().starts_with("2_"));
    }

    #[test]
    fn legacy_single_step_chapter_has_one_code_step() {
        let exercises =
            scan_dir(Path::new("examples")).expect("examples dir should exist when running tests");
        // Any chapter with only `main.rs` should produce exactly one code
        // step ordered at 0 (which renders as the chapter's primary step).
        let chapter = exercises
            .iter()
            .find(|e| e.slug == "rust_fundamentals_quiz")
            .expect("expected 18_rust_fundamentals_quiz to be present");
        let code_steps = chapter.code_steps();
        assert_eq!(code_steps.len(), 1, "legacy chapter should have one step");
        assert_eq!(code_steps[0].order, 0);
        assert!(
            !chapter.is_multi_step(),
            "legacy chapter should report as single-step"
        );
    }

    #[test]
    fn strip_inner_doc_removes_leading_block() {
        let src = "//! # Title\n//!\n//! body\n\nfn main() {}\n";
        assert_eq!(strip_inner_doc(src), "fn main() {}\n");
    }

    #[test]
    fn strip_inner_doc_leaves_inner_attrs_alone() {
        // No leading //!; nothing to strip.
        let src = "use std::io;\n\nfn main() {}\n";
        assert_eq!(strip_inner_doc(src), src);
    }

    #[test]
    fn split_numeric_prefix_works() {
        assert_eq!(
            split_numeric_prefix("02_strings_and_chars"),
            Some((2, "strings_and_chars".to_string()))
        );
        assert_eq!(
            split_numeric_prefix("13_question_mark_operator"),
            Some((13, "question_mark_operator".to_string()))
        );
        assert_eq!(split_numeric_prefix("no_prefix"), None);
    }

    #[test]
    fn split_title_peels_first_heading() {
        let md = "# Hello\n\nworld\n";
        let (title, body) = split_title(md);
        assert_eq!(title.as_deref(), Some("Hello"));
        assert!(body.trim_start().starts_with("world"));
    }

    #[test]
    fn split_title_handles_no_heading() {
        let md = "Just a paragraph.\n";
        let (title, body) = split_title(md);
        assert_eq!(title, None);
        assert_eq!(body, md);
    }

    #[test]
    fn split_hints_markdown_separates_h2_sections() {
        let md = "intro line\n\n## `foo`\n\nbody of foo\n\n## bar\n\nbody of bar\n";
        let (intro, sections) = split_hints_markdown(md);
        assert!(intro.contains("intro line"));
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].0, "`foo`");
        assert!(sections[0].1.contains("body of foo"));
        assert_eq!(sections[1].0, "bar");
        assert!(sections[1].1.contains("body of bar"));
    }

    #[test]
    fn heading_matches_slug_uses_first_backticked_token() {
        assert!(heading_matches_slug(
            "`quoted_line`, the state machine",
            "quoted_line"
        ));
        assert!(heading_matches_slug("`sum`", "sum"));
        assert!(heading_matches_slug("plain heading", "plain heading"));
        assert!(!heading_matches_slug("`other`, prose", "sum"));
    }

    #[test]
    fn integers_chapter_distributes_hints_per_step() {
        let exercises =
            scan_dir(Path::new("examples")).expect("examples dir should exist when running tests");
        let chapter = exercises
            .iter()
            .find(|e| e.slug == "integers")
            .expect("expected 01_integers to be present");
        // Every code step in this chapter has a `## <slug>` section in
        // hints.md, so each step should now carry its own rendered HTML.
        for code in chapter.code_steps() {
            assert!(
                code.hints_html.is_some(),
                "step {} should have per-step hints attached",
                code.slug
            );
        }
        // And because every section was claimed by a step, the
        // chapter-wide leftover hints should collapse to None (only the
        // tiny intro paragraph remains, but it's pure prose with no
        // unmatched H2s, so we still expect Some, just shorter).
        if let Some(leftover) = &chapter.hints {
            assert!(
                !leftover.html.contains("number_to_string"),
                "per-step section should no longer appear in the leftover"
            );
        }
    }

    #[test]
    fn renamed_chapters_distribute_hints_per_step() {
        // 11_iterators and 17_csv_parser were renamed so each hints H2
        // is keyed by the file slug (e.g. `` ## `quoted_line`, the
        // state machine ``). Every code step should receive its slice.
        let exercises =
            scan_dir(Path::new("examples")).expect("examples dir should exist when running tests");
        for slug in ["iterators", "csv_parser"] {
            let chapter = exercises
                .iter()
                .find(|e| e.slug == slug)
                .unwrap_or_else(|| panic!("expected chapter `{slug}` to be present"));
            for code in chapter.code_steps() {
                assert!(
                    code.hints_html.is_some(),
                    "{slug}: step {} should have hints attached",
                    code.slug
                );
            }
            assert!(
                chapter.hints.is_none(),
                "{slug}: leftover hints block should collapse when every section is consumed"
            );
        }
    }
}
