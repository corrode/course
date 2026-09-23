//! Response-level coverage uses an ephemeral listener and an in-memory database.
use super::*;
use std::collections::HashSet;

struct TestServer {
    base: String,
    client: reqwest::Client,
    task: tokio::task::JoinHandle<()>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl TestServer {
    async fn start(state: AppState) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let task = tokio::spawn(async move {
            let router = build_router_with_static(
                state,
                std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../static")),
            );
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            base,
            client: reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            task,
        }
    }

    async fn get(&self, path: &str) -> reqwest::Response {
        self.client
            .get(format!("{}{path}", self.base))
            .header("host", "attacker.invalid")
            .header("x-forwarded-host", "attacker.invalid")
            .header("x-forwarded-proto", "http")
            .send()
            .await
            .unwrap()
    }
}

async fn state() -> AppState {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    sqlx::query("INSERT INTO participants (id, name, team_token) VALUES ('private-id', 'Private Person', 'secret-team')").execute(&pool).await.unwrap();
    AppState {
        site_origin: SiteOrigin::parse("https://courses.example:8443/").unwrap(),
        pool,
        admin_token: "secret-admin".into(),
        exercises: exercises::load(std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples"
        )))
        .unwrap(),
    }
}

fn head(html: &str) -> &str {
    html.split_once("<head>")
        .unwrap()
        .1
        .split_once("</head>")
        .unwrap()
        .0
}

fn value<'a>(html: &'a str, prefix: &str, suffix: &str) -> &'a str {
    html.split_once(prefix)
        .unwrap_or_else(|| panic!("missing {prefix}"))
        .1
        .split_once(suffix)
        .unwrap()
        .0
}

fn json_ld(html: &str) -> serde_json::Value {
    serde_json::from_str(value(
        html,
        "<script type=\"application/ld+json\">",
        "</script>",
    ))
    .unwrap()
}

fn assert_no_store(response: &reqwest::Response) {
    assert_eq!(
        response
            .headers()
            .get("cache-control")
            .and_then(|value| value.to_str().ok()),
        Some("no-store, must-revalidate"),
        "{}",
        response.url()
    );
}

#[tokio::test]
async fn homepage_preserves_editorial_intro_and_has_anonymous_prefooter() {
    let server = TestServer::start(state().await).await;
    let html = server.get("/").await.text().await.unwrap();
    assert!(html.contains("<h1 class=\"book-title\">A Beginner's Guide to Rust</h1>"));
    assert!(html.contains("The day has finally come."));
    assert!(html.contains("or, Making Friends with the Borrow Checker"));
    assert!(!html.contains("Frequently Asked Questions"));
    let about = value(&html, "<section class=\"course-about\"", "</section>");
    assert_eq!(about.matches("<p>").count(), 2);
    assert!(about.contains("/static/assets/matthias-endler.jpg"));
    assert!(about.contains("loading=\"lazy\""));
    assert!(html.find("class=\"course-about\"").unwrap() < html.find("<footer").unwrap());
    let returning = server
        .get("/dashboard/private-id")
        .await
        .text()
        .await
        .unwrap();
    assert!(!returning.contains("class=\"course-about\""));
    let photo = server.get("/static/assets/matthias-endler.jpg").await;
    assert_eq!(photo.status(), StatusCode::OK);
    assert_eq!(photo.headers()["content-type"], "image/jpeg");
}

#[tokio::test]
async fn lesson_navigation_uses_topbar_without_duplicate_breadcrumbs() {
    let state = state().await;
    let lesson = state.exercises[0].file_stem.clone();
    let server = TestServer::start(state).await;
    for (path, home) in [
        (format!("/exercise/{lesson}"), "/"),
        (
            format!("/exercise/private-id/{lesson}"),
            "/dashboard/private-id",
        ),
    ] {
        let html = server.get(&path).await.text().await.unwrap();
        assert!(!html.contains("class=\"course-breadcrumbs\""));
        assert!(html.contains(&format!("href=\"{home}\" class=\"topbar-home\">Course</a>")));
        assert_eq!(html.matches("id=\"chapter-picker-menu\"").count(), 1);
        assert!(html.contains("aria-current=\"page\""));
        if home == "/" {
            assert_eq!(json_ld(head(&html))["itemListElement"][0]["name"], "Course");
        }
    }
    let tour = server.get("/tour").await.text().await.unwrap();
    assert!(!tour.contains("class=\"course-breadcrumbs\""));
    assert!(tour.contains("class=\"topbar-home\">Course</a>"));
}

#[tokio::test]
async fn course_context_banner_is_anonymous_only_for_chapters_and_absent_on_tour() {
    let state = state().await;
    let chapters = state.exercises.clone();
    let server = TestServer::start(state).await;
    for chapter in chapters.iter() {
        for participant in [None, Some("private-id")] {
            let path = participant.map_or_else(
                || seo::lesson_path(chapter),
                |id| format!("/exercise/{id}/{}", chapter.file_stem),
            );
            let html = server.get(&path).await.text().await.unwrap();
            assert_eq!(
                html.matches("class=\"register-banner accent-callout\"")
                    .count(),
                usize::from(participant.is_none())
            );
            if participant.is_none() {
                let banner = value(&html, "<aside class=\"register-banner", "</aside>");
                assert!(banner.contains("href=\"/\""));
                assert!(banner.contains("Guide to Rust</a>"));
                assert!(banner.contains("href=\"/signup\""));
                assert!(banner.contains("save your progress"));
            } else {
                assert!(!html.contains("id=\"inline-signup\""));
            }
        }
    }
    for (path, expected) in [("/tour", 0), ("/tour/private-id", 0)] {
        let html = server.get(path).await.text().await.unwrap();
        assert_eq!(
            html.matches("class=\"register-banner accent-callout\"")
                .count(),
            expected
        );
    }
}

#[tokio::test]
async fn homepage_links_to_canonical_lessons() {
    let state = state().await;
    let catalog = state.exercises.clone();
    let server = TestServer::start(state).await;
    let html = server.get("/").await.text().await.unwrap();
    assert!(html.contains("id=\"curriculum\""));

    for chapter in catalog.iter() {
        assert!(html.contains(&format!("href=\"{}\"", seo::lesson_path(chapter))));
    }
}

#[tokio::test]
async fn homepage_schema_ids_follow_canonical_and_claims_match_visible_content() {
    for origin in [seo::DEFAULT_ORIGIN, "https://courses.example:8443/"] {
        let mut state = state().await;
        state.site_origin = SiteOrigin::parse(origin).unwrap();
        let server = TestServer::start(state).await;
        let response = server.get("/?token=untrusted-query").await;
        assert_no_store(&response);
        let html = response.text().await.unwrap();
        let head = head(&html);
        let canonical = value(head, "rel=\"canonical\" href=\"", "\"");
        assert_eq!(canonical, format!("{}/", origin.trim_end_matches('/')));
        assert_eq!(
            value(head, "<title>", "</title>")
                .replace("&#39;", "'")
                .replace("&#x27;", "'"),
            "A Beginner's Guide to Rust | corrode"
        );
        let description = value(head, "name=\"description\" content=\"", "\"")
            .replace("&#39;", "'")
            .replace("&#x27;", "'");
        assert_eq!(
            description,
            "A Beginner's Guide to Rust. A free, interactive course. Write code in your browser and practice ownership, borrowing, and error handling. No setup needed."
        );
        let json = json_ld(head);
        assert_eq!(json["url"], canonical);
        assert_eq!(json["@id"], format!("{canonical}#course"));
        assert_eq!(json["provider"]["@id"], format!("{canonical}#organization"));
        assert_eq!(json["author"]["@id"], format!("{canonical}#person"));
        assert_eq!(json["description"], description);
        assert_eq!(json["isAccessibleForFree"], true);
        assert_eq!(json["inLanguage"], "en");
        assert_eq!(json["educationalLevel"], "Beginner");
        assert_eq!(
            json["coursePrerequisites"],
            "Familiarity with variables, functions, and loops in another programming language. No previous Rust experience is needed."
        );
        assert_eq!(
            json["teaches"],
            serde_json::json!([
                "ownership and borrowing",
                "pattern matching",
                "Option",
                "Result",
                "iterators",
                "structs and methods",
                "modules"
            ])
        );
        let body = html.split_once("</head>").unwrap().1;
        let body = body.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(body.contains(json["name"].as_str().unwrap()));
        assert!(body.contains("variables, functions, and loops in another"));
        assert!(body.contains("You don't need any Rust experience."));
        for topic in json["teaches"].as_array().unwrap() {
            assert!(body.contains(topic.as_str().unwrap()));
        }
        for entity in ["provider", "author"] {
            assert!(body.contains(json[entity]["name"].as_str().unwrap()));
            assert!(body.contains(&format!(
                "href=\"{}\"",
                json[entity]["url"].as_str().unwrap()
            )));
        }
        let plain = server.get("/").await.text().await.unwrap();
        assert_eq!(json_ld(self::head(&plain)), json);
    }
}

#[tokio::test]
async fn all_public_pages_have_unique_consistent_metadata_and_breadcrumbs() {
    let state = state().await;
    let paths = seo::public_paths(&state.exercises);
    assert!(state.exercises.iter().any(Exercise::is_bonus));
    let server = TestServer::start(state).await;
    let mut titles = HashSet::new();
    let mut descriptions = HashSet::new();
    for path in paths {
        let response = server.get(&format!("{path}?token=untrusted-query")).await;
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert_no_store(&response);
        assert_eq!(
            response.headers()["x-robots-tag"],
            "index, follow",
            "{path}"
        );
        let html = response.text().await.unwrap();
        let head = head(&html);
        assert_eq!(head.matches("<title>").count(), 1);
        assert_eq!(head.matches("name=\"description\"").count(), 1);
        assert_eq!(head.matches("rel=\"canonical\"").count(), 1);
        let title = value(head, "<title>", "</title>");
        let description = value(head, "name=\"description\" content=\"", "\"");
        assert!(titles.insert(title.to_owned()), "duplicate title: {path}");
        assert!(
            descriptions.insert(description.to_owned()),
            "duplicate description: {path}"
        );
        for prefix in [
            "property=\"og:title\" content=\"",
            "name=\"twitter:title\" content=\"",
        ] {
            assert_eq!(value(head, prefix, "\""), title);
        }
        for prefix in [
            "property=\"og:description\" content=\"",
            "name=\"twitter:description\" content=\"",
        ] {
            assert_eq!(value(head, prefix, "\""), description);
        }
        let canonical = format!("https://courses.example:8443{path}");
        assert_eq!(value(head, "rel=\"canonical\" href=\"", "\""), canonical);
        assert_eq!(
            value(head, "property=\"og:url\" content=\"", "\""),
            canonical
        );
        for prefix in [
            "property=\"og:image\" content=\"",
            "name=\"twitter:image\" content=\"",
        ] {
            assert_eq!(
                value(head, prefix, "\""),
                "https://courses.example:8443/static/assets/social.png"
            );
        }
        assert!(!head.contains("attacker.invalid"));
        assert!(!head.contains("untrusted-query"));
        if path == "/" {
            let json = json_ld(head);
            assert_eq!(json["@type"], "Course");
            assert_eq!(json["provider"]["@type"], "Organization");
            assert_eq!(json["author"]["@type"], "Person");
        } else if path.starts_with("/exercise/") {
            let json = json_ld(head);
            assert_eq!(json["@type"], "BreadcrumbList");
            assert_eq!(json["itemListElement"][1]["item"], canonical);
        }
    }
}

#[tokio::test]
async fn application_pages_never_publish_personal_metadata_including_invalid_participants() {
    let state = state().await;
    let lesson = state.exercises[0].file_stem.clone();
    let server = TestServer::start(state).await;
    let mut paths: Vec<String> = [
        "/signup",
        "/signup/secret-team",
        "/settings",
        "/settings/private-id",
        "/dashboard/private-id",
        "/dashboard/private-id/team",
        "/tour/private-id",
        "/tour/missing",
        "/admin?token=secret-admin",
        "/admin/team/secret-team?token=secret-admin",
        "/admin/team-unassigned?token=secret-admin",
        "/admin/participants/private-id/submissions?token=secret-admin",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    paths.push(format!("/exercise/private-id/{lesson}"));
    paths.push(format!("/exercise/missing/{lesson}"));
    for path in paths {
        let response = server.get(&path).await;
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert_no_store(&response);
        assert_eq!(
            response.headers()["x-robots-tag"],
            "noindex, nofollow",
            "{path}"
        );
        let html = response.text().await.unwrap();
        let head = head(&html);
        for secret in [
            "Private Person",
            "private-id",
            "secret-team",
            "secret-admin",
            "rel=\"canonical\"",
            "property=\"og:",
            "name=\"twitter:",
            "application/ld+json",
        ] {
            assert!(!head.contains(secret), "{path} leaked {secret}");
        }
        assert_eq!(head.matches("<title>").count(), 1);
        assert!(head.contains("noindex, nofollow"));
    }
}

#[tokio::test]
async fn fragments_apis_errors_and_mutations_are_noindex() {
    let state = state().await;
    let pool = state.pool.clone();
    let server = TestServer::start(state).await;
    for path in [
        "/cheatsheet/fragment",
        "/admin/team-members?token=secret-admin",
        "/api/status/private-id",
        "/health",
        "/not-found",
        "/exercise/not-found",
        "/dashboard/missing",
        "/settings/missing",
        "/admin",
        "/admin?token=wrong",
        "/static/missing.css",
        "/api/unknown",
        "/exercise/%FF",
    ] {
        let response = server.get(path).await;
        assert_eq!(
            response.headers()["x-robots-tag"],
            "noindex, nofollow",
            "{path}"
        );
    }
    for (method, path) in [
        (reqwest::Method::POST, "/"),
        (reqwest::Method::POST, "/register"),
        (reqwest::Method::POST, "/api/register"),
        (reqwest::Method::POST, "/api/submit"),
        (reqwest::Method::POST, "/api/events"),
        (reqwest::Method::POST, "/api/run"),
        (reqwest::Method::POST, "/api/format"),
        (reqwest::Method::DELETE, "/admin/remove-participant/missing"),
        (
            reqwest::Method::POST,
            "/admin/participants/missing/team-token",
        ),
    ] {
        let response = server
            .client
            .request(method, format!("{}{path}", server.base))
            .send()
            .await
            .unwrap();
        assert!(response.status().is_client_error(), "{path}");
        assert_eq!(
            response.headers()["x-robots-tag"],
            "noindex, nofollow",
            "{path}"
        );
    }
    pool.close().await;
    let response = server.get("/health").await;
    assert_no_store(&response);
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.headers()["x-robots-tag"], "noindex, nofollow");
}

#[tokio::test]
async fn redirects_are_direct_local_and_preserve_queries_without_redirecting_posts() {
    let state = state().await;
    let catalog = state.exercises.clone();
    let server = TestServer::start(state).await;
    for lesson in catalog.iter() {
        for path in [
            format!("/exercise/{}", lesson.slug),
            format!("/exercise/{}/", lesson.slug),
            format!("/exercise/{}/", lesson.file_stem),
        ] {
            let response = server.get(&format!("{path}?a=1&b=2")).await;
            assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT, "{path}");
            assert_no_store(&response);
            assert_eq!(
                response.headers()["location"],
                format!("/exercise/{}?a=1&b=2", lesson.file_stem)
            );
            assert_eq!(response.headers()["x-robots-tag"], "noindex, nofollow");
        }
    }
    for path in [
        "/tour/",
        "/settings/",
        "/cheatsheet/",
        "/playground/",
        "/robots.txt/",
        "/sitemap.xml/",
    ] {
        let response = server.get(path).await;
        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(response.headers()["location"], path.trim_end_matches('/'));
    }
    let lesson = &catalog[0];
    let response = server
        .get(&format!("/exercise/private-id/{}/", lesson.slug))
        .await;
    assert_eq!(
        response.headers()["location"],
        format!("/exercise/private-id/{}", lesson.file_stem)
    );
    let response = server
        .client
        .head(format!("{}/exercise/{}", server.base, lesson.slug))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    let response = server
        .client
        .post(format!("{}/tour/", server.base))
        .send()
        .await
        .unwrap();
    assert!(!response.status().is_redirection());
    for path in [
        "//attacker.invalid/",
        "/%2f%2fattacker.invalid/",
        "/\\attacker.invalid/",
        "/../tour/",
        "/./tour/",
    ] {
        assert_eq!(canonical_redirect(path, &catalog), None);
    }
}

#[tokio::test]
async fn discovery_inventory_contains_only_public_canonical_urls_including_bonus() {
    let state = state().await;
    let paths = seo::public_paths(&state.exercises);
    let server = TestServer::start(state).await;
    for path in ["/robots.txt", "/sitemap.xml"] {
        let get = server.get(path).await;
        let head = server
            .client
            .head(format!("{}{path}", server.base))
            .send()
            .await
            .unwrap();
        assert_eq!(get.status(), StatusCode::OK);
        assert_eq!(head.status(), StatusCode::OK);
        assert_no_store(&get);
        assert_no_store(&head);
        for header in ["content-type", "x-robots-tag", "cache-control"] {
            assert_eq!(get.headers()[header], head.headers()[header]);
        }
        assert!(head.text().await.unwrap().is_empty());
    }
    let robots = server.get("/robots.txt").await;
    assert!(
        robots.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("text/plain")
    );
    let robots = robots.text().await.unwrap();
    assert!(robots.contains("Sitemap: https://courses.example:8443/sitemap.xml"));
    assert!(!robots.contains("Disallow:"));
    let response = server.get("/sitemap.xml").await;
    assert!(
        response.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("application/xml")
    );
    let xml = response.text().await.unwrap();
    assert_eq!(xml.matches("<loc>").count(), paths.len());
    for path in paths {
        assert!(xml.contains(&format!("<loc>https://courses.example:8443{path}</loc>")));
    }
    for forbidden in [
        "private-id",
        "secret",
        "/signup",
        "/settings",
        "/admin",
        "/api/",
        "/dashboard",
        "lastmod",
        "priority",
        "changefreq",
    ] {
        assert!(!xml.contains(forbidden));
    }
}

#[tokio::test]
async fn dynamic_no_store_and_static_cacheability_are_preserved() {
    let server = TestServer::start(state().await).await;
    for path in [
        "/cheatsheet/fragment",
        "/api/status/private-id",
        "/admin/team-members?token=secret-admin",
        "/admin",
        "/admin?token=wrong",
        "/dashboard/missing",
        "/settings/missing",
        "/exercise/not-found",
        "/exercise/%FF",
        "/tour/",
    ] {
        let response = server.get(path).await;
        assert_no_store(&response);
    }
    for path in ["/api/register", "/api/submit", "/register"] {
        let response = server
            .client
            .post(format!("{}{path}", server.base))
            .send()
            .await
            .unwrap();
        assert!(response.status().is_client_error());
        assert_no_store(&response);
    }
    let response = server.get("/static/css/base.css").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(!response.headers().contains_key("cache-control"));
    assert!(!response.headers().contains_key("x-robots-tag"));
    let image = server.get("/static/assets/social.png").await;
    assert_eq!(image.status(), StatusCode::OK);
    assert!(!image.headers().contains_key("x-robots-tag"));
    assert!(response.headers().contains_key("last-modified"));
}

#[tokio::test]
async fn new_chapters_have_safe_fallback_metadata_and_xml_urls() {
    let mut state = state().await;
    state.site_origin = SiteOrigin::parse(seo::DEFAULT_ORIGIN).unwrap();
    let mut catalog = state.exercises.as_ref().clone();
    let mut lesson = catalog[0].clone();
    lesson.slug = "new_topic".into();
    lesson.file_stem = "99_new&topic".into();
    lesson.title = "New <topic> & practice".into();
    let path = seo::lesson_path(&lesson);
    catalog.push(lesson);
    state.exercises = Arc::new(catalog);
    let server = TestServer::start(state).await;
    let response = server.get(&path).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-robots-tag"], "index, follow");
    let html = response.text().await.unwrap();
    let head = head(&html);
    assert!(!value(head, "name=\"description\" content=\"", "\"").contains('<'));
    assert_eq!(
        json_ld(head)["itemListElement"][1]["item"],
        format!("{}{path}", seo::DEFAULT_ORIGIN)
    );
    let xml = server.get("/sitemap.xml").await.text().await.unwrap();
    assert!(xml.contains("https://course.corrode.dev/exercise/99_new&amp;topic</loc>"));
    assert!(!xml.contains("<topic>"));
    let response = server
        .client
        .head(format!("{}/tour", server.base))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-robots-tag"], "index, follow");
    assert!(response.text().await.unwrap().is_empty());
}

#[tokio::test]
async fn hostile_lesson_text_is_html_escaped_and_json_remains_parseable() {
    let mut state = state().await;
    let mut catalog = state.exercises.as_ref().clone();
    catalog[0].title = "Rust \"<& </script><script>alert(1)</script>".into();
    let expected = catalog[0].title.clone();
    let path = seo::lesson_path(&catalog[0]);
    state.exercises = Arc::new(catalog);
    let server = TestServer::start(state).await;
    let response = server.get(&path).await;
    assert_eq!(response.status(), StatusCode::OK);
    let html = response.text().await.unwrap();
    let head = head(&html);
    assert!(!head.contains("<script>alert(1)</script>"));
    assert!(head.contains("&lt;") || head.contains("&#60;"));
    assert!(!value(head, "<title>", "</title>").contains('<'));
    let json = json_ld(head);
    assert_eq!(json["itemListElement"][1]["name"], expected);
}
