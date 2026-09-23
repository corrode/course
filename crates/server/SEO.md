# Server SEO

`SITE_ORIGIN` defaults to `https://course.corrode.dev`. Set an absolute HTTP(S)
origin, optionally with a port or final slash, for another deployment. Credentials,
paths, queries, fragments, whitespace, and backslashes are rejected before the
server touches its database. Invalid explicit configuration stops startup rather
than silently publishing incorrect canonical URLs. Request `Host`, forwarded
headers, and query parameters never supply metadata URLs.

## Template interface

Every full-page template has a `seo: PageMetadata` field. `templates/base.html`
owns the only title, description, canonical, Open Graph, Twitter, and JSON-LD
markup. Child templates should not define `title`, `og_title`, or
`og_description` blocks; the previous overrides have been removed.
Ordinary metadata uses Askama HTML escaping. Only JSON serialized and escaped by
`seo::script_json` is passed through the `safe` filter.

Public metadata is provided for `/`, `/tour`, `/cheatsheet`, `/playground`, and
`/exercise/{file_stem}` for every loaded chapter, including bonus chapters,
quizzes, and the appendix. Editorial lesson descriptions live in
`src/lesson_seo.rs`; new chapters have a catalog-derived fallback. No chapter
configuration, progress rules, submission keys, or on-disk names are changed.
The homepage emits Course data with a corrode Organization provider and Matthias
Endler as its Person author. Lessons emit BreadcrumbList data.

## Routing and crawl policy

- Successful public GET/HEAD pages receive `X-Robots-Tag: index, follow`.
- Other responses receive `noindex, nofollow`: signup, dashboards, participant
  lessons/tours, settings, teams, admin, fragments, API responses, health checks,
  redirects, errors, and unmatched paths. Successful static assets retain their
    existing crawl and cache behavior without an indexing header. Application HTML has
  a matching robots meta tag, generic workspace metadata, and no canonical,
  social tags, or structured data. Failed participant lookup does not change the
  requested route's classification.
- GET/HEAD lesson aliases redirect permanently (308) directly to their existing
  numbered `file_stem`. The same applies to participant aliases, retaining the
  participant path. Safe trailing slashes are removed in that same redirect,
  without an intermediate alias hop; `/` is unchanged. Queries are retained in
  redirects but excluded from metadata. POST and other mutation methods are not
  redirected. Ambiguous encoded paths, backslashes, dot segments, and
  scheme-relative paths are not normalized.
- `/robots.txt` advertises `/sitemap.xml`. It does not disallow application URLs:
  crawlers need to fetch those routes to observe their `noindex` headers.
- The sitemap uses the same catalog inventory as the indexing policy and only
  contains canonical public URLs. XML delimiters are escaped. No fabricated
  modification dates, priorities, or change frequencies are emitted.

Robots directives are not access control; they do not make bearer-link pages
private from someone who already has the URL.

## Tests

`cargo test -p course-server seo` runs metadata/configuration tests and HTTP
response tests against an ephemeral local listener and migrated in-memory SQLite
database. Tests cover all public chapters, uniqueness, canonical/social agreement,
JSON-LD, forged host headers, query exclusion, participant fallback, application
page privacy, redirects, fragments, API/extractor errors, method handling,
HEAD, robots/sitemap, bonus chapters, new-chapter fallback, and hostile HTML/JSON/XML
strings. They neither start the production bootstrap nor read `course.db`.

Run the full server suite with `cargo test -p course-server`.
