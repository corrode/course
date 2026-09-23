//! Search metadata is built from trusted configuration and the public catalog only.
use course_server::exercises::Exercise;
use serde_json::json;
use std::fmt::Write as _;

pub const DEFAULT_ORIGIN: &str = "https://course.corrode.dev";

#[derive(Clone, Debug)]
pub struct SiteOrigin(String);

impl SiteOrigin {
    pub fn parse(value: &str) -> anyhow::Result<Self> {
        // Reject URL parser repairs, credentials, and anything beyond an origin.
        anyhow::ensure!(
            !value.chars().any(char::is_whitespace) && !value.contains('\\'),
            "SITE_ORIGIN must be an HTTP(S) origin"
        );
        let url = url::Url::parse(value)?;
        let authority = value.split_once("://").map_or("", |(_, rest)| rest);
        let (host, suffix) = authority.split_once('/').unwrap_or((authority, ""));
        anyhow::ensure!(
            !host.is_empty() && suffix.is_empty(),
            "SITE_ORIGIN cannot contain a path"
        );
        anyhow::ensure!(
            matches!(url.scheme(), "http" | "https")
                && value.starts_with(&format!("{}://", url.scheme()))
                && url.host_str().is_some()
                && url.username().is_empty()
                && url.password().is_none()
                && !authority.contains('@')
                && url.path() == "/"
                && url.query().is_none()
                && url.fragment().is_none(),
            "SITE_ORIGIN must contain only an HTTP(S) scheme, host, and optional port"
        );
        Ok(Self(url.origin().ascii_serialization()))
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{path}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct PageMetadata {
    pub title: String,
    pub description: String,
    pub canonical: Option<String>,
    pub image: String,
    pub json_ld: Option<String>,
}

pub const COURSE_NAME: &str = "A Beginner's Guide to Rust";

impl PageMetadata {
    pub fn private() -> Self {
        Self {
            title: format!("Your learning space | {COURSE_NAME}"),
            description: "Your personal Rust course workspace.".into(),
            canonical: None,
            image: String::new(),
            json_ld: None,
        }
    }

    pub fn public(origin: &SiteOrigin, path: &str, catalog: &[Exercise]) -> Option<Self> {
        let (title, description) = match path {
            "/" => (format!("{COURSE_NAME} | corrode"), "A free, interactive course. Write code in your browser and practice ownership, borrowing, and error handling. No setup needed.".into()),
            "/tour" => (format!("A Quick Tour of Rust | {COURSE_NAME}"), "Explore Rust syntax in an interactive tour: variables, functions, ownership, pattern matching, and more, with runnable code.".into()),
            "/cheatsheet" => (format!("Rust Cheatsheet | {COURSE_NAME}"), "Look up Rust syntax for types, borrowing, collections, error handling, and iterators in a compact reference for beginners.".into()),
            "/playground" => (format!("Rust Playground | {COURSE_NAME}"), "Write, run, and experiment with Rust code in your browser. Use this free playground to practice without installing a toolchain.".into()),
            _ => {
                let lesson = catalog.iter().find(|e| lesson_path(e) == path)?;
                let label = if lesson.title.trim().is_empty() { lesson.slug.replace('_', " ") } else { lesson.title.clone() };
                let description = crate::lesson_seo::description(&lesson.slug).map_or_else(
                    || format!("Learn {label} in this Rust course chapter. Read explanations and practice the concepts in your browser."),
                    str::to_owned,
                );
                (format!("{label} | {COURSE_NAME}"), description)
            }
        };
        let description = format!("{COURSE_NAME}. {description}");
        let canonical = origin.url(path);
        let json_ld = if path == "/" {
            Some(script_json(&json!({
                "@context": "https://schema.org",
                "@type": "Course",
                "@id": format!("{canonical}#course"),
                "name": COURSE_NAME,
                "description": description,
                "url": canonical,
                "isAccessibleForFree": true,
                "inLanguage": "en",
                "educationalLevel": "Beginner",
                "coursePrerequisites": "Familiarity with variables, functions, and loops in another programming language. No previous Rust experience is needed.",
                "teaches": ["ownership and borrowing", "pattern matching", "Option", "Result", "iterators", "structs and methods", "modules"],
                "provider": {
                    "@type": "Organization", "@id": format!("{canonical}#organization"),
                    "name": "corrode", "url": "https://corrode.dev/"
                },
                "author": {
                    "@type": "Person", "@id": format!("{canonical}#person"),
                    "name": "Matthias Endler", "url": "https://endler.dev/about/"
                }
            })))
        } else if path.starts_with("/exercise/") {
            let lesson = catalog.iter().find(|e| path == lesson_path(e))?;
            Some(script_json(&json!({
                "@context": "https://schema.org", "@type": "BreadcrumbList",
                "itemListElement": [
                    {"@type": "ListItem", "position": 1, "name": "Course", "item": origin.url("/")},
                    {"@type": "ListItem", "position": 2, "name": lesson.title, "item": canonical}
                ]
            })))
        } else {
            None
        };
        Some(Self {
            title,
            description,
            canonical: Some(canonical),
            image: origin.url("/static/assets/social.png"),
            json_ld,
        })
    }
}

// JSON serialization alone does not escape an HTML script closing tag.
fn script_json(value: &serde_json::Value) -> String {
    value
        .to_string()
        .replace('&', "\\u0026")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

pub fn lesson_path(lesson: &Exercise) -> String {
    lesson_key_path(&lesson.file_stem)
}

pub fn lesson_key_path(key: &str) -> String {
    // Encode the existing key as one path segment without changing its identity.
    let mut url = url::Url::parse(DEFAULT_ORIGIN).expect("constant origin");
    url.path_segments_mut()
        .expect("HTTP URL")
        .extend(["exercise", key]);
    url.path().to_owned()
}

pub fn public_paths(catalog: &[Exercise]) -> Vec<String> {
    ["/", "/tour", "/cheatsheet", "/playground"]
        .into_iter()
        .map(str::to_owned)
        .chain(catalog.iter().map(lesson_path))
        .collect()
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn sitemap(origin: &SiteOrigin, catalog: &[Exercise]) -> String {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for path in public_paths(catalog) {
        let _ = writeln!(
            xml,
            "<url><loc>{}</loc></url>",
            xml_escape(&origin.url(&path))
        );
    }
    xml.push_str("</urlset>\n");
    xml
}

pub fn robots(origin: &SiteOrigin) -> String {
    // Do not disallow private routes: crawlers must see their noindex headers.
    format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}\n",
        origin.url("/sitemap.xml")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lesson_keys_are_encoded_as_one_segment_without_changing_real_urls() {
        assert_eq!(
            lesson_key_path("99_new topic/part?draft#details%"),
            "/exercise/99_new%20topic%2Fpart%3Fdraft%23details%25"
        );
        let catalog = course_server::exercises::load(std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples"
        )))
        .unwrap();
        for lesson in catalog.iter() {
            for key in [&lesson.file_stem, &lesson.slug] {
                assert_eq!(lesson_key_path(key), format!("/exercise/{key}"));
            }
            assert_eq!(lesson_path(lesson), lesson_key_path(&lesson.file_stem));
        }
    }

    #[test]
    fn validates_origin_without_repairs_or_url_components() {
        for bad in [
            "",
            "javascript:alert(1)",
            "https:example.com",
            "https:///example.com",
            "https://user@example.com",
            "https://@example.com",
            "https://example.com/path",
            "https://example.com/?x=1",
            "https://example.com/#x",
            "https://example.com\\evil",
            " https://example.com",
            "https://example.com\n",
        ] {
            assert!(SiteOrigin::parse(bad).is_err(), "accepted {bad:?}");
        }
        assert_eq!(
            SiteOrigin::parse("https://example.com:8443/")
                .unwrap()
                .url("/tour"),
            "https://example.com:8443/tour"
        );
        assert!(SiteOrigin::parse("http://localhost:3000").is_ok());
    }

    #[test]
    fn json_is_script_safe_and_round_trips() {
        let value = json!({"name": "</script><script>alert('x')</script>&\u{2028}\u{2029}"});
        let encoded = script_json(&value);
        assert!(!encoded.contains('<'));
        assert!(!encoded.contains('&'));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&encoded).unwrap(),
            value
        );
    }

    #[test]
    fn xml_escapes_all_delimiters() {
        assert_eq!(xml_escape("<&>\"'"), "&lt;&amp;&gt;&quot;&apos;");
    }
}
