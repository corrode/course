//! Startup-time scan and parse of `examples/*.rs` exercise files.
//!
//! See `docs/in_browser_exercises.md` for the design.

use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A single exercise, parsed from one `examples/NN_slug.rs` file.
#[derive(Debug, Clone)]
pub struct Exercise {
    /// Numeric prefix from the filename, e.g. `2` for `02_strings_and_chars.rs`.
    pub number: u8,
    /// Slug without the numeric prefix, e.g. `strings_and_chars`.
    pub slug: String,
    /// File stem including the prefix, e.g. `02_strings_and_chars`.
    pub file_stem: String,
    /// Title, taken from the first H1 of the `//!` block. Falls back to
    /// `file_stem` if no H1 is present.
    pub title: String,
    /// Remainder of the `//!` block (everything after the first H1),
    /// rendered to HTML at startup.
    pub intro_html: String,
    /// Verbatim file contents — what the editor is seeded with.
    pub starter_code: String,
}

impl Exercise {
    /// True for files whose slug contains "quiz".
    #[must_use]
    pub fn is_quiz(&self) -> bool {
        self.slug.contains("quiz")
    }
}

/// Scan a directory for `NN_*.rs` files and parse each one.
///
/// Files whose stem does not match `^\d+_` are skipped. Files that fail
/// to parse are logged and skipped, so a single bad file does not take
/// down the whole server.
pub fn scan_dir(dir: &Path) -> Result<Vec<Exercise>> {
    if !dir.exists() {
        return Err(anyhow!("Examples directory not found: {}", dir.display()));
    }

    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"))
        .filter(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.chars().next().is_some_and(|c| c.is_ascii_digit()))
        })
        .collect();

    paths.sort();

    let mut out = Vec::with_capacity(paths.len());
    for path in paths {
        match parse_file(&path) {
            Ok(ex) => out.push(ex),
            Err(e) => {
                log::warn!("Skipping {}: {e:#}", path.display());
            }
        }
    }
    Ok(out)
}

/// Convenience: scan and wrap in `Arc` for `AppState`.
pub fn load(dir: &Path) -> Result<Arc<Vec<Exercise>>> {
    Ok(Arc::new(scan_dir(dir)?))
}

fn parse_file(path: &Path) -> Result<Exercise> {
    let starter_code = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;

    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("invalid filename"))?
        .to_string();

    let (number, slug) = split_numeric_prefix(&file_stem)
        .ok_or_else(|| anyhow!("filename does not start with NN_: {file_stem}"))?;

    let intro_md = extract_inner_doc(&starter_code)?;
    let (title, body_md) = split_title(&intro_md);
    let title = title.unwrap_or_else(|| file_stem.clone());
    let intro_html = render_markdown(&body_md);

    Ok(Exercise {
        number,
        slug,
        file_stem,
        title,
        intro_html,
        starter_code,
    })
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

fn render_markdown(md: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(md, opts);
    let mut out = String::with_capacity(md.len() * 2);
    html::push_html(&mut out, parser);
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
        assert_eq!(strings.title, "Strings, &str, and chars");
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
