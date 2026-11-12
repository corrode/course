//! Browser-account regressions against the real router, isolated SQLite, and loopback HTTP.
use super::*;
use axum_extra::extract::cookie::{Cookie, SameSite};
use reqwest::{Method, Response as HttpResponse};

const ORIGIN: &str = "https://courses.example:8443";
const COOKIE: &str = "corrode_participant";
const ACCOUNT_A: &str = "corrode_participant=account-a";
const ACCOUNT_B: &str = "corrode_participant=account-b";
const UNKNOWN: &str = "corrode_participant=unknown-account";

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

fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}

impl TestServer {
    async fn start(state: AppState) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let router = build_router_with_static(
            state,
            std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../static")),
        );
        let task = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            base,
            client: http_client(),
            task,
        }
    }

    fn request(&self, method: Method, path: &str, cookie: Option<&str>) -> reqwest::RequestBuilder {
        let request = self.client.request(method, format!("{}{path}", self.base));
        match cookie {
            Some(cookie) => request.header("cookie", cookie),
            None => request,
        }
    }

    async fn get(&self, path: &str, cookie: Option<&str>) -> HttpResponse {
        send(self.request(Method::GET, path, cookie)).await
    }
}

async fn send(request: reqwest::RequestBuilder) -> HttpResponse {
    // Request errors can contain bearer URLs; don't include them in test output.
    request
        .send()
        .await
        .unwrap_or_else(|_| panic!("loopback HTTP request failed"))
}

async fn state() -> AppState {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO participants (id, name, team_token) VALUES \
         ('account-a', 'Account A', 'test-team'), ('account-b', 'Account B', 'test-team')",
    )
    .execute(&pool)
    .await
    .unwrap();
    AppState {
        site_origin: SiteOrigin::parse(ORIGIN).unwrap(),
        pool,
        admin_token: "test-admin".into(),
        exercises: exercises::load(std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples"
        )))
        .unwrap(),
    }
}

fn personal_paths(id: &str, lesson: &str) -> Vec<String> {
    vec![
        format!("/dashboard/{id}"),
        format!("/dashboard/{id}/team"),
        format!("/exercise/{id}/{lesson}"),
        format!("/tour/{id}"),
        format!("/playground/{id}"),
        format!("/cheatsheet/{id}"),
        format!("/settings/{id}"),
    ]
}

fn assert_no_cookie(response: &HttpResponse) {
    assert!(
        !response.headers().contains_key("set-cookie"),
        "cookie must remain unchanged"
    );
}

fn response_cookie(response: &HttpResponse) -> Cookie<'static> {
    let mut cookies = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .filter_map(|value| {
            let cookie = Cookie::parse(value.to_str().ok()?.to_owned()).ok()?;
            (cookie.name() == COOKIE).then_some(cookie)
        });
    let cookie = cookies.next().expect("missing participant Set-Cookie");
    assert!(cookies.next().is_none(), "duplicate participant Set-Cookie");
    cookie
}

fn assert_cookie(response: &HttpResponse, id: &str, secure: bool, expired: bool) -> String {
    let cookie = response_cookie(response);
    // Boolean comparisons deliberately avoid printing bearer values on failure.
    assert!(cookie.value() == id, "unexpected participant cookie value");
    assert_eq!(cookie.path(), Some("/"));
    assert_eq!(cookie.http_only(), Some(true));
    assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    assert_eq!(cookie.secure().unwrap_or(false), secure);
    assert!(cookie.domain().is_none(), "cookie must be host-only");
    assert_eq!(
        cookie.max_age().map(|age| age.whole_seconds()),
        Some(if expired { 0 } else { 180 * 24 * 60 * 60 }),
    );
    format!("{COOKIE}={}", cookie.value())
}

fn assert_private(response: &HttpResponse) {
    let cache = response
        .headers()
        .get("cache-control")
        .expect("missing cache policy");
    assert!(
        cache
            .to_str()
            .unwrap()
            .split(',')
            .any(|part| part.trim() == "no-store")
    );
    assert_eq!(response.headers()["x-robots-tag"], "noindex, nofollow");
    assert_eq!(response.headers()["referrer-policy"], "strict-origin");
}

fn local_location(server: &TestServer, response: &HttpResponse) -> String {
    let location = response
        .headers()
        .get("location")
        .expect("missing redirect destination");
    let location = location.to_str().expect("invalid Location");
    assert!(location.starts_with('/') && !location.starts_with("//"));
    assert!(
        !location.contains(['\\', '\r', '\n']),
        "unsafe redirect destination"
    );
    let base = url::Url::parse(&server.base).unwrap();
    let target = base.join(location).expect("invalid local redirect");
    assert!(
        target.origin() == base.origin(),
        "redirect escaped the loopback origin"
    );
    location.to_owned()
}

fn assert_redirect(server: &TestServer, response: &HttpResponse, status: StatusCode, target: &str) {
    assert_eq!(response.status(), status);
    assert_private(response);
    assert!(
        local_location(server, response) == target,
        "unexpected redirect destination"
    );
}

mod admin_token_queries {
    use super::*;
    use std::collections::HashMap;

    const TOKENS: [&str; 7] = [
        "plain-admin",
        "secret&value",
        "secret+value",
        "secret?value",
        "secret#value",
        "secret%26value",
        "secret +&=?#%/é",
    ];

    async fn admin_state(token: &str) -> AppState {
        let mut state = state().await;
        state.admin_token = token.to_owned();
        sqlx::query("INSERT INTO participants (id, name) VALUES ('unassigned', 'Unassigned')")
            .execute(&state.pool)
            .await
            .unwrap();
        state
    }

    async fn page(server: &TestServer, path: &str, token: &str) -> String {
        let response = server
            .get(&format!("{path}?token={}", encode_query_value(token)), None)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        response.text().await.unwrap()
    }

    fn attribute_urls(html: &str, attribute: &str, prefix: &str) -> Vec<String> {
        html.split(&format!("{attribute}=\""))
            .skip(1)
            .map(|rest| rest.split_once('"').unwrap().0)
            .filter(|value| value.starts_with(prefix))
            // Decode HTML attribute escaping, not URL percent encoding.
            .map(|value| value.replace("&#38;", "&").replace("&amp;", "&"))
            .collect()
    }

    fn assert_token(server: &TestServer, path: &str, token: &str) -> url::Url {
        let base = url::Url::parse(&server.base).unwrap();
        let url = base.join(path).unwrap();
        assert_eq!(url.origin(), base.origin());
        assert!(url.fragment().is_none());
        let tokens: Vec<_> = url
            .query_pairs()
            .filter(|(key, _)| key == "token")
            .map(|(_, value)| value.into_owned())
            .collect();
        assert_eq!(tokens, [token]);
        url
    }

    #[tokio::test]
    async fn rendered_team_links_preserve_admin_tokens() {
        for token in TOKENS {
            let server = TestServer::start(admin_state(token).await).await;
            let html = page(&server, "/admin", token).await;
            let links = attribute_urls(&html, "href", "/admin/team");
            assert_eq!(links.len(), 2);
            for link in links {
                let url = assert_token(&server, &link, token);
                assert!(matches!(
                    url.path(),
                    "/admin/team/test-team" | "/admin/team-unassigned"
                ));
                assert_eq!(url.query_pairs().count(), 1);
                let response = server.get(&link, None).await;
                assert_eq!(response.status(), StatusCode::OK);
                let team_html = response.text().await.unwrap();
                let back_links = attribute_urls(&team_html, "href", "/admin?");
                assert_eq!(back_links.len(), 1);
                let url = assert_token(&server, &back_links[0], token);
                assert_eq!(url.path(), "/admin");
                assert_eq!(url.query_pairs().count(), 1);
                let response = server.get(&back_links[0], None).await;
                assert_eq!(response.status(), StatusCode::OK);
            }
        }
    }

    #[tokio::test]
    async fn rendered_htmx_filter_and_sort_links_preserve_admin_tokens() {
        for token in TOKENS {
            let server = TestServer::start(admin_state(token).await).await;
            let html = page(&server, "/admin", token).await;
            let links = attribute_urls(&html, "hx-get", "/admin/team-members?");
            assert_eq!(links.len(), 8, "filter and three sorts for each bucket");
            for link in links {
                let mut url = assert_token(&server, &link, token);
                let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
                assert!(matches!(params["team"].as_str(), "test-team" | ""));
                if let Some(sort) = params.get("sort") {
                    assert!(matches!(sort.as_str(), "name" | "progress" | "activity"));
                    assert_eq!(params["dir"], if sort == "name" { "desc" } else { "asc" });
                    assert_eq!(url.query_pairs().count(), 4);
                } else {
                    assert_eq!(url.query_pairs().count(), 2);
                    // The filter input includes the live hidden sort state.
                    url.query_pairs_mut()
                        .append_pair("sort", "name")
                        .append_pair("dir", "asc");
                }
                let filter = if params["team"].is_empty() {
                    "Unassigned"
                } else {
                    "Account A"
                };
                url.query_pairs_mut().append_pair("filter", filter);
                let response = send(server.client.get(url).header("HX-Request", "true")).await;
                assert_eq!(response.status(), StatusCode::OK);
                let fragment = response.text().await.unwrap();
                let sorts = attribute_urls(&fragment, "hx-get", "/admin/team-members?");
                assert_eq!(sorts.len(), 3);
                for sort in sorts {
                    let url = assert_token(&server, &sort, token);
                    assert_eq!(url.query_pairs().count(), 4);
                    let response = send(server.client.get(url).header("HX-Request", "true")).await;
                    assert_eq!(response.status(), StatusCode::OK);
                }
                let actions = attribute_urls(&fragment, "action", "/admin/participants/");
                assert_eq!(
                    actions.len(),
                    1,
                    "filter must retain only the matching member"
                );
                let url = assert_token(&server, &actions[0], token);
                let expected_id = if params["team"].is_empty() {
                    "unassigned"
                } else {
                    "account-a"
                };
                assert_eq!(
                    url.path(),
                    format!("/admin/participants/{expected_id}/team-token")
                );
            }
        }
    }

    #[tokio::test]
    async fn rendered_move_forms_preserve_admin_tokens_and_update_membership() {
        for token in TOKENS {
            let state = admin_state(token).await;
            let server = TestServer::start(state.clone()).await;
            // Render before moving anyone so both named and unassigned rosters are populated.
            let pages = [
                (
                    page(&server, "/admin", token).await,
                    "account-a",
                    "from-admin",
                ),
                (
                    page(&server, "/admin/team/test-team", token).await,
                    "account-a",
                    "from-team",
                ),
                (
                    page(&server, "/admin/team-unassigned", token).await,
                    "unassigned",
                    "from-unassigned",
                ),
            ];
            for (html, id, destination) in pages {
                let actions = attribute_urls(
                    &html,
                    "action",
                    &format!("/admin/participants/{id}/team-token?"),
                );
                assert_eq!(actions.len(), 1);
                let url = assert_token(&server, &actions[0], token);
                assert_eq!(url.query_pairs().count(), 1);
                let response = send(
                    server
                        .client
                        .post(url)
                        .header("content-type", "application/x-www-form-urlencoded")
                        .body(format!("team_token={destination}")),
                )
                .await;
                assert_eq!(response.status(), StatusCode::SEE_OTHER);
                let location = local_location(&server, &response);
                let url = assert_token(&server, &location, token);
                assert_eq!(url.path(), "/admin");
                assert_eq!(url.query_pairs().count(), 1);
                let response = server.get(&location, None).await;
                assert_eq!(response.status(), StatusCode::OK);
                let dashboard = response.text().await.unwrap();
                let team_links =
                    attribute_urls(&dashboard, "href", &format!("/admin/team/{destination}?"));
                assert_eq!(team_links.len(), 1);
                assert_token(&server, &team_links[0], token);
                let response = server.get(&team_links[0], None).await;
                assert_eq!(response.status(), StatusCode::OK);
                let actual: String =
                    sqlx::query_scalar("SELECT team_token FROM participants WHERE id = ?")
                        .bind(id)
                        .fetch_one(&state.pool)
                        .await
                        .unwrap();
                assert_eq!(actual, destination);
            }
        }
    }
}

#[tokio::test]
async fn remembered_navigation_redirects_locally_and_preserves_raw_queries() {
    let state = state().await;
    let lesson = state.exercises[0].file_stem.clone();
    let server = TestServer::start(state).await;
    let query = "?next=https%3A%2F%2Fevil.invalid%2F&next=%2f%2fevil.invalid&x=a+b&x=a%20b&empty=&flag&encoded=%26%3D%23%25%0D%0A";
    for (source, target) in [
        ("/".to_owned(), "/dashboard/account-a".to_owned()),
        ("/dashboard".to_owned(), "/dashboard/account-a".to_owned()),
        (
            format!("/exercise/{lesson}"),
            format!("/exercise/account-a/{lesson}"),
        ),
        ("/tour".to_owned(), "/tour/account-a".to_owned()),
        ("/playground".to_owned(), "/playground/account-a".to_owned()),
        ("/cheatsheet".to_owned(), "/cheatsheet/account-a".to_owned()),
        ("/settings".to_owned(), "/settings/account-a".to_owned()),
    ] {
        let response = send(
            server
                .request(Method::GET, &format!("{source}{query}"), Some(ACCOUNT_A))
                .header("host", "evil.invalid")
                .header("x-forwarded-host", "evil.invalid")
                .header("x-forwarded-proto", "http"),
        )
        .await;
        assert_redirect(
            &server,
            &response,
            StatusCode::TEMPORARY_REDIRECT,
            &format!("{target}{query}"),
        );
        assert_no_cookie(&response);
        let destination = server
            .get(&local_location(&server, &response), Some(ACCOUNT_A))
            .await;
        assert_eq!(destination.status(), StatusCode::OK);
        assert_private(&destination);
        assert_cookie(&destination, "account-a", true, false);
    }
    // Encoded path delimiters must stay inside the exercise slug, not become URL syntax.
    let response = server
        .get(
            "/exercise/%2F%2Fevil.invalid%3Fq%3D1%23fragment?x=%252F",
            Some(ACCOUNT_A),
        )
        .await;
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_private(&response);
    let location = local_location(&server, &response);
    assert!(location.starts_with("/exercise/account-a/%2F%2F"));
    assert!(location.ends_with("?x=%252F"));
    let target = url::Url::parse(&server.base)
        .unwrap()
        .join(&location)
        .unwrap();
    assert!(target.fragment().is_none());
    assert_eq!(target.query(), Some("x=%252F"));
}

#[tokio::test]
async fn cookie_security_uses_configured_origin_not_transport_or_forwarded_headers() {
    for (origin, secure, forwarded) in [
        (ORIGIN, true, "http"),
        ("http://courses.example:8080", false, "https"),
    ] {
        let mut state = state().await;
        state.site_origin = SiteOrigin::parse(origin).unwrap();
        let server = TestServer::start(state).await;
        let response = send(
            server
                .request(Method::GET, "/dashboard/account-a", None)
                .header("host", "evil.invalid")
                .header("x-forwarded-proto", forwarded)
                .header("sec-fetch-dest", "document"),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_private(&response);
        assert_cookie(&response, "account-a", secure, false);
    }
}

#[tokio::test]
async fn explicit_account_wins_on_every_personal_page() {
    let state = state().await;
    let paths = personal_paths("account-b", &state.exercises[0].file_stem);
    let server = TestServer::start(state).await;
    for path in paths {
        for cookie in [None, Some(ACCOUNT_A), Some(UNKNOWN)] {
            let response = server.get(&path, cookie).await;
            assert_eq!(response.status(), StatusCode::OK);
            assert_private(&response);
            let remembered = assert_cookie(&response, "account-b", true, false);
            let home = server.get("/", Some(&remembered)).await;
            assert_redirect(
                &server,
                &home,
                StatusCode::TEMPORARY_REDIRECT,
                "/dashboard/account-b",
            );
        }
    }
}

#[tokio::test]
async fn independent_client_can_log_in_without_changing_another_browser() {
    let server = TestServer::start(state().await).await;
    let first = server.get("/dashboard/account-a", None).await;
    let first_cookie = assert_cookie(&first, "account-a", true, false);
    let second = http_client();
    let anonymous = send(second.get(format!("{}/", server.base))).await;
    assert_eq!(anonymous.status(), StatusCode::OK);
    assert_no_cookie(&anonymous);
    let login = send(second.get(format!("{}/dashboard/account-b", server.base))).await;
    assert_eq!(login.status(), StatusCode::OK);
    let second_cookie = assert_cookie(&login, "account-b", true, false);
    let home = send(
        second
            .get(format!("{}/", server.base))
            .header("cookie", second_cookie),
    )
    .await;
    assert_redirect(
        &server,
        &home,
        StatusCode::TEMPORARY_REDIRECT,
        "/dashboard/account-b",
    );
    let first_home = server.get("/", Some(&first_cookie)).await;
    assert_redirect(
        &server,
        &first_home,
        StatusCode::TEMPORARY_REDIRECT,
        "/dashboard/account-a",
    );
}

#[tokio::test]
async fn invalid_explicit_links_never_fall_back_anywhere_in_the_redirect_chain() {
    let state = state().await;
    let lesson = &state.exercises[0];
    let mut paths = personal_paths("missing-account", &lesson.file_stem);
    paths.extend([
        "/dashboard/missing-account/".into(),
        format!("/exercise/missing-account/{}/?x=%2f&x=a+b", lesson.slug),
        "/exercise/missing-account/missing-exercise".into(),
    ]);
    let server = TestServer::start(state).await;
    for path in paths {
        for cookie in [None, Some(ACCOUNT_A), Some(UNKNOWN)] {
            let mut next = path.clone();
            let mut terminated = false;
            for _ in 0..4 {
                let response = server.get(&next, cookie).await;
                assert_no_cookie(&response);
                assert_private(&response);
                if response.status().is_redirection() {
                    // Canonicalization may precede account validation, but must retain the explicit ID.
                    next = local_location(&server, &response);
                    assert!(
                        next.split('?')
                            .next()
                            .unwrap()
                            .split('/')
                            .any(|s| s == "missing-account"),
                        "invalid login fell back to another account or anonymous page"
                    );
                } else {
                    assert_eq!(response.status(), StatusCode::NOT_FOUND);
                    terminated = true;
                    break;
                }
            }
            assert!(terminated, "invalid login redirect chain did not terminate");
        }
    }
    let home = server.get("/", Some(ACCOUNT_A)).await;
    assert_redirect(
        &server,
        &home,
        StatusCode::TEMPORARY_REDIRECT,
        "/dashboard/account-a",
    );
}

#[tokio::test]
async fn unknown_and_deleted_cookies_expire_without_losing_anonymous_access() {
    let state = state().await;
    let pool = state.pool.clone();
    let server = TestServer::start(state).await;
    let login = server.get("/dashboard/account-b", None).await;
    let deleted_cookie = assert_cookie(&login, "account-b", true, false);
    sqlx::query("DELETE FROM participants WHERE id = 'account-b'")
        .execute(&pool)
        .await
        .unwrap();
    for cookie in [UNKNOWN, deleted_cookie.as_str()] {
        for (path, status) in [
            ("/", StatusCode::OK),
            ("/settings", StatusCode::OK),
            ("/tour", StatusCode::OK),
            ("/exercise/missing-exercise", StatusCode::NOT_FOUND),
        ] {
            let response = server.get(path, Some(cookie)).await;
            assert_eq!(response.status(), status);
            assert!(!response.headers().contains_key("location"));
            assert_cookie(&response, "", true, true);
        }
    }
    let home = server.get("/", None).await;
    assert_eq!(home.status(), StatusCode::OK);
    assert_no_cookie(&home);
}

#[tokio::test]
async fn database_failure_is_unavailable_not_anonymous_or_cookie_expiry() {
    let state = state().await;
    let pool = state.pool.clone();
    let lesson = state.exercises[0].file_stem.clone();
    let server = TestServer::start(state).await;
    pool.close().await;
    let mut paths = personal_paths("account-b", &lesson);
    paths.extend([
        "/".into(),
        "/dashboard".into(),
        "/settings".into(),
        "/tour".into(),
        format!("/exercise/{lesson}"),
    ]);
    for path in paths {
        for cookie in [ACCOUNT_A, UNKNOWN] {
            let response = server.get(&path, Some(cookie)).await;
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert_private(&response);
            assert_no_cookie(&response);
            assert!(!response.headers().contains_key("location"));
        }
    }
    let explicit = server.get("/tour/account-b", None).await;
    assert_eq!(explicit.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_no_cookie(&explicit);
    let anonymous = server.get("/tour", None).await;
    assert_eq!(anonymous.status(), StatusCode::OK);
    assert_no_cookie(&anonymous);
}

async fn assert_personal_handlers_fail_closed(state: AppState, expected: StatusCode) {
    let lesson = state.exercises[0].file_stem.clone();
    for (handler, response) in [
        (
            "dashboard",
            participant_dashboard(AxumPath("account-b".into()), State(state.clone()))
                .await
                .into_response(),
        ),
        (
            "exercise",
            participant_exercise_page(AxumPath(("account-b".into(), lesson)), State(state.clone()))
                .await
                .into_response(),
        ),
        (
            "team",
            participant_team_page(AxumPath("account-b".into()), State(state.clone()))
                .await
                .into_response(),
        ),
        (
            "settings",
            participant_settings_page(AxumPath("account-b".into()), State(state))
                .await
                .into_response(),
        ),
    ] {
        assert_eq!(response.status(), expected, "{handler}");
        assert!(!response.headers().contains_key("location"), "{handler}");
        assert!(!response.headers().contains_key("set-cookie"), "{handler}");
    }
}

#[tokio::test]
async fn personal_handlers_reject_account_deleted_after_browser_validation() {
    let state = state().await;
    assert!(
        browser_account::participant_exists(&state, "account-b")
            .await
            .unwrap()
    );
    sqlx::query("DELETE FROM participants WHERE id = 'account-b'")
        .execute(&state.pool)
        .await
        .unwrap();

    // Call handlers directly so middleware cannot intercept the second lookup.
    assert_personal_handlers_fail_closed(state, StatusCode::NOT_FOUND).await;
}

#[tokio::test]
async fn personal_handlers_reject_database_failure_after_browser_validation() {
    let state = state().await;
    assert!(
        browser_account::participant_exists(&state, "account-b")
            .await
            .unwrap()
    );
    state.pool.close().await;

    assert_personal_handlers_fail_closed(state, StatusCode::SERVICE_UNAVAILABLE).await;
}

#[tokio::test]
async fn excluded_routes_ignore_both_valid_and_unknown_cookies() {
    let state = state().await;
    let pool = state.pool.clone();
    let server = TestServer::start(state).await;
    let paths = [
        "/signup",
        "/signup/test-team",
        "/admin",
        "/admin?token=test-admin",
        "/admin/team-members?token=test-admin&team=test-team",
        "/api/status/account-b",
        "/api/unknown",
        "/health",
        "/cheatsheet/fragment",
        "/static/css/base.css",
        "/static/missing.css",
        "/robots.txt",
        "/sitemap.xml",
        "/not-found",
    ];
    for path in paths {
        let baseline = server.get(path, None).await;
        let expected_status = baseline.status();
        let expected_location = baseline.headers().get("location").cloned();
        for cookie in [ACCOUNT_A, UNKNOWN] {
            let response = server.get(path, Some(cookie)).await;
            assert_eq!(
                response.status(),
                expected_status,
                "excluded route changed status"
            );
            assert!(
                response.headers().get("location") == expected_location.as_ref(),
                "excluded route changed destination"
            );
            assert_no_cookie(&response);
        }
    }
    // These handlers need no database: a cookie lookup here would incorrectly return 503.
    pool.close().await;
    for path in [
        "/signup",
        "/signup/test-team",
        "/cheatsheet/fragment",
        "/static/css/base.css",
    ] {
        let response = server.get(path, Some(ACCOUNT_A)).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_no_cookie(&response);
    }
}

#[tokio::test]
async fn missing_exercises_do_not_replace_a_remembered_account() {
    let server = TestServer::start(state().await).await;
    for cookie in [None, Some(ACCOUNT_A)] {
        let response = server
            .get("/exercise/account-b/missing-exercise", cookie)
            .await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_private(&response);
        assert_no_cookie(&response);
    }
    let response = server
        .get("/exercise/missing-exercise", Some(ACCOUNT_A))
        .await;
    assert_redirect(
        &server,
        &response,
        StatusCode::TEMPORARY_REDIRECT,
        "/exercise/account-a/missing-exercise",
    );
    assert_no_cookie(&response);
    let destination = server
        .get(&local_location(&server, &response), Some(ACCOUNT_A))
        .await;
    assert_eq!(destination.status(), StatusCode::NOT_FOUND);
    assert_no_cookie(&destination);
}

#[tokio::test]
async fn head_subresources_and_prefetch_never_write_or_expire_cookies() {
    let server = TestServer::start(state().await).await;
    for (method, headers) in [
        (Method::HEAD, vec![]),
        (Method::GET, vec![("sec-fetch-dest", "empty")]),
        (Method::GET, vec![("sec-fetch-dest", "iframe")]),
        (Method::GET, vec![("sec-fetch-dest", "script")]),
        (
            Method::GET,
            vec![("sec-fetch-dest", "document"), ("purpose", "prefetch")],
        ),
        (
            Method::GET,
            vec![
                ("sec-fetch-dest", "document"),
                ("sec-purpose", "prefetch;prerender"),
            ],
        ),
    ] {
        for (path, cookie, status) in [
            ("/tour/account-b", None, StatusCode::OK),
            ("/tour/account-b", Some(ACCOUNT_A), StatusCode::OK),
            ("/tour", Some(UNKNOWN), StatusCode::OK),
            ("/tour", Some(ACCOUNT_A), StatusCode::TEMPORARY_REDIRECT),
            (
                "/tour/missing-account",
                Some(ACCOUNT_A),
                StatusCode::NOT_FOUND,
            ),
        ] {
            let mut request = server.request(method.clone(), path, cookie);
            for (name, value) in &headers {
                request = request.header(*name, *value);
            }
            let response = send(request).await;
            assert_eq!(response.status(), status);
            assert_no_cookie(&response);
        }
    }
    let navigation = send(
        server
            .request(Method::GET, "/tour/account-b", Some(ACCOUNT_A))
            .header("sec-fetch-dest", "document"),
    )
    .await;
    assert_cookie(&navigation, "account-b", true, false);
}

#[tokio::test]
async fn logout_accepts_same_origin_or_referer_without_database_access() {
    for (origin, secure) in [(ORIGIN, true), ("http://courses.example:8080", false)] {
        let mut state = state().await;
        state.site_origin = SiteOrigin::parse(origin).unwrap();
        let pool = state.pool.clone();
        let server = TestServer::start(state).await;
        pool.close().await;
        for (header, source) in [
            ("origin", origin.to_owned()),
            ("referer", format!("{origin}/settings/account-a?from=menu")),
        ] {
            for cookie in [None, Some(ACCOUNT_A), Some(UNKNOWN)] {
                let response = send(
                    server
                        .request(Method::POST, "/logout", cookie)
                        .header(header, &source),
                )
                .await;
                assert_redirect(&server, &response, StatusCode::SEE_OTHER, "/");
                assert_cookie(&response, "", secure, true);
            }
        }
        let home = server.get("/", None).await;
        assert_eq!(home.status(), StatusCode::OK);
        assert_no_cookie(&home);
    }
}

#[tokio::test]
async fn logout_rejects_unsafe_origins_and_never_mutates_on_get() {
    let server = TestServer::start(state().await).await;
    for headers in [
        vec![],
        vec![("origin", "null")],
        vec![("origin", "https://evil.invalid")],
        vec![("origin", "http://courses.example:8443")],
        vec![("origin", "https://courses.example")],
        vec![("origin", "https://courses.example:8443.evil.invalid")],
        vec![("origin", "not a URL")],
        vec![("referer", "https://evil.invalid/settings")],
        vec![
            ("origin", "null"),
            ("referer", "https://courses.example:8443/settings/account-a"),
        ],
        vec![
            ("origin", "https://evil.invalid"),
            ("referer", "https://courses.example:8443/settings/account-a"),
        ],
        vec![
            ("origin", "https://evil.invalid"),
            ("host", "evil.invalid"),
            ("x-forwarded-host", "evil.invalid"),
            ("x-forwarded-proto", "https"),
        ],
    ] {
        let mut request = server.request(Method::POST, "/logout", Some(ACCOUNT_A));
        for (name, value) in headers {
            request = request.header(name, value);
        }
        let response = send(request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_no_cookie(&response);
        assert!(!response.headers().contains_key("location"));
    }
    for method in [Method::GET, Method::HEAD] {
        let response = send(
            server
                .request(method, "/logout", Some(ACCOUNT_A))
                .header("origin", ORIGIN),
        )
        .await;
        assert!(response.status().is_client_error());
        assert_no_cookie(&response);
        assert!(!response.headers().contains_key("location"));
    }
    let home = server.get("/", Some(ACCOUNT_A)).await;
    assert_redirect(
        &server,
        &home,
        StatusCode::TEMPORARY_REDIRECT,
        "/dashboard/account-a",
    );
}

#[tokio::test]
async fn api_submission_requires_explicit_identity_even_with_a_browser_cookie() {
    let state = state().await;
    let pool = state.pool.clone();
    let chapter = state
        .exercises
        .iter()
        .find(|exercise| !exercise.is_quiz() && !exercise.code_steps().is_empty())
        .unwrap();
    let chapter_name = chapter.file_stem.clone();
    let exercise_names: Vec<String> = chapter
        .code_steps()
        .iter()
        .map(|step| {
            if step.key().is_empty() {
                chapter_name.clone()
            } else {
                format!("{chapter_name}/{}", step.key())
            }
        })
        .collect();
    let server = TestServer::start(state).await;
    let mut submission = serde_json::json!({
        "ulid": "account-b", "exercise_name": exercise_names[0],
        "source_code": "fn main() {}", "tests_passed": true,
        "clippy_passed": true, "fmt_passed": true,
    });
    let response = send(
        server
            .request(Method::POST, "/api/submit", Some(ACCOUNT_A))
            .json(&submission),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_no_cookie(&response);
    assert_private(&response);
    let owner: String = sqlx::query_scalar("SELECT participant_id FROM submissions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(
        owner == "account-b",
        "submission used cookie identity instead of explicit identity"
    );
    submission["ulid"] = serde_json::json!("missing-account");
    let invalid = send(
        server
            .request(Method::POST, "/api/submit", Some(ACCOUNT_A))
            .json(&submission),
    )
    .await;
    assert_eq!(invalid.status(), StatusCode::UNAUTHORIZED);
    assert_no_cookie(&invalid);
    submission.as_object_mut().unwrap().remove("ulid");
    let missing = send(
        server
            .request(Method::POST, "/api/submit", Some(ACCOUNT_A))
            .json(&submission),
    )
    .await;
    assert_eq!(missing.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_no_cookie(&missing);
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM submissions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    // Finish the chapter for B so the two status responses cannot accidentally be identical.
    submission["ulid"] = serde_json::json!("account-b");
    for exercise_name in &exercise_names[1..] {
        submission["exercise_name"] = serde_json::json!(exercise_name);
        let response = send(
            server
                .request(Method::POST, "/api/submit", Some(ACCOUNT_A))
                .json(&submission),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_no_cookie(&response);
    }
    for (id, cookie) in [("account-a", ACCOUNT_B), ("account-b", ACCOUNT_A)] {
        let response = server.get(&format!("/api/status/{id}"), Some(cookie)).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_no_cookie(&response);
        let progress: ProgressResponse = response.json().await.unwrap();
        let chapter = progress
            .exercises
            .iter()
            .find(|exercise| exercise.name == chapter_name)
            .unwrap();
        assert_eq!(
            chapter.completed,
            id == "account-b",
            "status API used cookie identity"
        );
    }
}

#[tokio::test]
async fn web_registration_remembers_only_on_the_dashboard_or_exercise_destination() {
    let state = state().await;
    let pool = state.pool.clone();
    let lesson = state.exercises[0].file_stem.clone();
    let server = TestServer::start(state).await;
    for (index, next) in [
        None,
        Some(format!("/exercise/{lesson}")),
        Some("https://evil.invalid/".into()),
    ]
    .into_iter()
    .enumerate()
    {
        let name = format!("New browser {index}");
        let mut form = vec![("name", name.as_str())];
        if let Some(next) = next.as_deref() {
            form.push(("next", next));
        }
        let body = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(form)
            .finish();
        let response = send(
            server
                .request(Method::POST, "/register", Some(ACCOUNT_A))
                .header("content-type", "application/x-www-form-urlencoded")
                .body(body),
        )
        .await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_private(&response);
        assert_no_cookie(&response);
        let id: String = sqlx::query_scalar("SELECT id FROM participants WHERE name = ?")
            .bind(&name)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(id != "account-a" && id != "account-b");
        let expected = if index == 1 {
            format!("/exercise/{id}/{lesson}")
        } else {
            format!("/dashboard/{id}")
        };
        let destination = local_location(&server, &response);
        assert!(
            destination == expected,
            "registration changed its destination contract"
        );
        let page = server.get(&destination, Some(ACCOUNT_A)).await;
        assert_eq!(page.status(), StatusCode::OK);
        let cookie = assert_cookie(&page, &id, true, false);
        let home = server.get("/", Some(&cookie)).await;
        assert_redirect(
            &server,
            &home,
            StatusCode::TEMPORARY_REDIRECT,
            &format!("/dashboard/{id}"),
        );
    }
}

#[tokio::test]
async fn api_registration_stays_explicit_until_the_new_login_link_is_opened() {
    let state = state().await;
    let pool = state.pool.clone();
    let server = TestServer::start(state).await;
    let response = send(
        server
            .request(Method::POST, "/api/register", Some(ACCOUNT_A))
            .json(&serde_json::json!({"name": "New API browser"})),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_no_cookie(&response);
    assert_private(&response);
    let registration: RegistrationResponse = response.json().await.unwrap();
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM participants WHERE id = ? AND name = 'New API browser')",
    )
    .bind(&registration.ulid)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(exists);
    let old_home = server.get("/", Some(ACCOUNT_A)).await;
    assert_redirect(
        &server,
        &old_home,
        StatusCode::TEMPORARY_REDIRECT,
        "/dashboard/account-a",
    );
    let destination = server
        .get(
            &format!("/dashboard/{}", registration.ulid),
            Some(ACCOUNT_A),
        )
        .await;
    assert_eq!(destination.status(), StatusCode::OK);
    let cookie = assert_cookie(&destination, &registration.ulid, true, false);
    let home = server.get("/", Some(&cookie)).await;
    assert_redirect(
        &server,
        &home,
        StatusCode::TEMPORARY_REDIRECT,
        &format!("/dashboard/{}", registration.ulid),
    );
}
