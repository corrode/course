//! Startup-time scan and parse of `examples/NN_slug/` chapter directories.
//!
//! Each chapter is a directory `examples/NN_slug/` containing exactly one
//! `main.rs` (the exercise) and zero or more `<n>_<slug>.md` notes that
//! render before the exercise prose on the web. The chapter directory name
//! matches the historical `file_stem`, so database keys (`exercise_name`)
//! are preserved across the migration.
//!
//! See `docs/in_browser_exercises.md` for the original design.

use anyhow::{Context, Result, anyhow};
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

/// A single exercise, parsed from one `examples/NN_slug/` chapter directory.
#[derive(Debug, Clone)]
pub struct Exercise {
    /// Numeric prefix from the directory name, e.g. `2` for `02_strings_and_chars`.
    pub number: u8,
    /// Slug without the numeric prefix, e.g. `strings_and_chars`.
    pub slug: String,
    /// Directory name including the prefix, e.g. `02_strings_and_chars`.
    /// Doubles as the key used in the `submissions.exercise_name` column.
    pub file_stem: String,
    /// Title, taken from the first H1 of the `//!` block. Falls back to
    /// `file_stem` if no H1 is present.
    pub title: String,
    /// Remainder of the `//!` block (everything after the first H1),
    /// rendered to HTML at startup.
    pub intro_html: String,
    /// Verbatim file contents — what the editor is seeded with.
    pub starter_code: String,
    /// Optional notes that render before `intro_html` on the web, in
    /// `order` order. Empty for chapters without any `.md` files.
    pub notes: Vec<Note>,
    /// Optional spoiler-protected hints. Sourced from a sibling note
    /// whose slug is exactly `hints` (e.g. `2_hints.md`). Renders as a
    /// closed `<details>` so learners only see them on demand.
    pub hints: Option<Note>,
}

impl Exercise {
    /// True for files whose slug contains "quiz".
    #[must_use]
    pub fn is_quiz(&self) -> bool {
        self.slug.contains("quiz")
    }
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

    let main_rs = dir.join("main.rs");
    if !main_rs.exists() {
        return Err(anyhow!("missing main.rs in {}", dir.display()));
    }
    let starter_code_full = std::fs::read_to_string(&main_rs)
        .with_context(|| format!("reading {}", main_rs.display()))?;

    let intro_md = extract_inner_doc(&starter_code_full)?;
    let (title, body_md) = split_title(&intro_md);
    let title = title.unwrap_or_else(|| file_stem.clone());
    let intro_html = render_markdown(&body_md);
    // Editor sees the file *without* the inner doc-comment; the prose above
    // already covers it, no need to repeat it inside the code area.
    let starter_code = strip_inner_doc(&starter_code_full);

    let mut notes = scan_notes(dir)?;
    // A note named `<n>_hints.md` is special: it gets its own slot on the
    // exercise page, hidden behind a click. Pull it out of the regular
    // notes list so it doesn't render inline above the intro prose.
    let hints_idx = notes.iter().position(|n| n.slug == "hints");
    let hints = hints_idx.map(|i| notes.remove(i));

    Ok(Exercise {
        number,
        slug,
        file_stem,
        title,
        intro_html,
        starter_code,
        notes,
        hints,
    })
}

/// Find `<n>_<slug>.md` notes alongside the chapter's `main.rs` and parse
/// each into a `Note`. Sorted by the leading number so chapter authors
/// can interleave notes with code in any order they like.
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
    // Notes render as ordinary chapter prose. Keep the leading H1 so it
    // appears as a real heading instead of being demoted into metadata.
    let title = split_title(&md).0.unwrap_or_else(|| slug.clone());
    let html = render_markdown(&md);

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
                // by rustc, but keep it verbatim — Markdown handles whitespace.
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
    // and the default HTML renderer wraps them in `<blockquote class="markdown-alert markdown-alert-note">`.
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
        assert_eq!(strings.title, "Strings and chars");
        assert!(
            strings.intro_html.contains("<p>"),
            "intro should be rendered HTML, got: {}",
            strings.intro_html
        );
        assert!(
            strings.intro_html.contains("<table>"),
            "intro should include the borrowed/owned table, got: {}",
            strings.intro_html
        );
        assert!(
            strings.starter_code.contains("fn first_char"),
            "starter code should contain function stubs"
        );
        assert!(
            !strings.starter_code.contains("//!"),
            "editor starter code should have the inner doc comment stripped"
        );
        assert!(
            strings.starter_code.starts_with("///")
                || strings.starter_code.starts_with("fn")
                || strings.starter_code.starts_with("use ")
                || strings.starter_code.starts_with("#["),
            "starter code should begin with code, not blank lines: {:?}",
            &strings.starter_code[..40.min(strings.starter_code.len())]
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
        assert!(
            !chapter.notes.is_empty(),
            "expected at least one note in 00_hello_rust"
        );
        let first = &chapter.notes[0];
        assert_eq!(first.order, 1);
        assert!(
            first.html.contains("<p>"),
            "note should render to HTML, got: {}",
            first.html
        );
        // External links should be rewritten to open in a new tab.
        assert!(
            first.html.contains("target=\"_blank\""),
            "external links should open in a new tab, got: {}",
            first.html
        );
        assert!(
            first.html.contains("class=\"external-link\""),
            "external links should be tagged for the icon affordance, got: {}",
            first.html
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
}
