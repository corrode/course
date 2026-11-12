//! Remember the existing participant bearer link; API authentication stays explicit.
use std::collections::HashMap;

use askama::Template;
use axum::{
    RequestExt,
    extract::{MatchedPath, Path, Request, State},
    http::{HeaderMap, Method, StatusCode, header},
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::{AppState, PageMetadata};

#[derive(Template)]
#[template(path = "login_error.html")]
struct InvalidLoginTemplate {
    seo: PageMetadata,
}

pub fn invalid_login() -> Response {
    match (InvalidLoginTemplate {
        seo: PageMetadata::private(),
    })
    .render()
    {
        Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
        Err(error) => {
            log::error!("Could not render invalid login page: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not display the login error.",
            )
                .into_response()
        }
    }
}

const COOKIE_NAME: &str = "corrode_participant";

fn account_cookie(state: &AppState, id: String) -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, id))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(state.site_origin.url("/").starts_with("https://"))
        .max_age(
            std::time::Duration::from_hours(180 * 24)
                .try_into()
                .expect("180 days fits a cookie duration"),
        )
        .build()
}

fn forget(state: &AppState, jar: CookieJar) -> CookieJar {
    let mut cookie = account_cookie(state, String::new());
    cookie.make_removal();
    jar.add(cookie)
}

fn is_navigation(request: &Request) -> bool {
    request.method() == Method::GET
        && request
            .headers()
            .get("sec-fetch-dest")
            .is_none_or(|value| value == "document")
        && ["purpose", "sec-purpose"].iter().all(|name| {
            request
                .headers()
                .get(*name)
                .is_none_or(|value| !value.to_str().unwrap_or("").contains("prefetch"))
        })
}

pub fn unavailable(error: &sqlx::Error) -> Response {
    log::error!("Could not resolve browser account: {error}");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        "Could not load your account. Please try again shortly.",
    )
        .into_response()
}

pub async fn participant_exists(state: &AppState, id: &str) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM participants WHERE id = ?)")
        .bind(id)
        .fetch_one(&state.pool)
        .await
}

fn personal_path(
    state: &AppState,
    route: &str,
    id: &str,
    params: &HashMap<String, String>,
) -> String {
    let mut url = url::Url::parse(&state.site_origin.url("/")).expect("validated site origin");
    let segments = match route {
        "/" | "/dashboard" => vec!["dashboard", id],
        "/exercise/{slug}" => vec!["exercise", id, params["slug"].as_str()],
        "/tour" => vec!["tour", id],
        "/playground" => vec!["playground", id],
        "/cheatsheet" => vec!["cheatsheet", id],
        "/settings" => vec!["settings", id],
        _ => unreachable!("only anonymous learner routes have personal counterparts"),
    };
    url.path_segments_mut()
        .expect("HTTP origin has path segments")
        .extend(segments);
    url.path().to_owned()
}

pub async fn remember(State(state): State<AppState>, mut request: Request, next: Next) -> Response {
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map_or("", MatchedPath::as_str)
        .to_owned();
    let anonymous = matches!(
        route.as_str(),
        "/" | "/dashboard"
            | "/exercise/{slug}"
            | "/tour"
            | "/playground"
            | "/cheatsheet"
            | "/settings"
    );
    let personal = matches!(
        route.as_str(),
        "/dashboard/{ulid}"
            | "/dashboard/{ulid}/team"
            | "/exercise/{ulid}/{slug}"
            | "/tour/{ulid}"
            | "/playground/{ulid}"
            | "/cheatsheet/{ulid}"
            | "/settings/{ulid}"
    );
    if !(anonymous || personal) || !matches!(*request.method(), Method::GET | Method::HEAD) {
        return next.run(request).await;
    }

    let remember = is_navigation(&request);
    let jar = CookieJar::from_headers(request.headers());
    let params = if route.contains('{') {
        match request
            .extract_parts::<Path<HashMap<String, String>>>()
            .await
        {
            Ok(Path(params)) => params,
            Err(error) => return error.into_response(),
        }
    } else {
        HashMap::new()
    };
    if personal {
        let id = &params["ulid"];
        match participant_exists(&state, id).await {
            Ok(true) => {}
            Ok(false) => {
                // Do not redirect through `/`: that could silently open a different
                // remembered account after a mistyped or deleted login link.
                return invalid_login();
            }
            Err(error) => return unavailable(&error),
        }
        let id = id.clone();
        let response = next.run(request).await;
        if remember && response.status().is_success() {
            (jar.add(account_cookie(&state, id)), response).into_response()
        } else {
            response
        }
    } else if let Some(cookie) = jar.get(COOKIE_NAME) {
        match participant_exists(&state, cookie.value()).await {
            Ok(true) => {
                let mut target = personal_path(&state, &route, cookie.value(), &params);
                if let Some(query) = request.uri().query() {
                    target.push('?');
                    target.push_str(query);
                }
                Redirect::temporary(&target).into_response()
            }
            Ok(false) => {
                let response = next.run(request).await;
                if remember {
                    (forget(&state, jar), response).into_response()
                } else {
                    response
                }
            }
            Err(error) => unavailable(&error),
        }
    } else {
        next.run(request).await
    }
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap, jar: CookieJar) -> Response {
    // Browser forms send Origin. Referer covers older same-origin clients;
    // never trust Host or forwarded headers as the configured application origin.
    let source = headers
        .get(header::ORIGIN)
        .or_else(|| headers.get(header::REFERER));
    let same_origin = source
        .and_then(|value| value.to_str().ok())
        .and_then(|value| url::Url::parse(value).ok())
        .is_some_and(|url| {
            format!("{}/", url.origin().ascii_serialization()) == state.site_origin.url("/")
        });
    if !same_origin {
        return (
            StatusCode::FORBIDDEN,
            "Sign out from the course settings page.",
        )
            .into_response();
    }
    (forget(&state, jar), Redirect::to("/")).into_response()
}
