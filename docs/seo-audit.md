# SEO Audit and Implementation

Audit date: September 23, 2026. Baseline commit:
`d5b176c6ec2933cfe6734f2641ca30f99f56f809`.
The working tree was clean before this work. No repository `AGENTS.md` was
present. The teaching audience and prose changes follow
[writing-guidelines.md](writing-guidelines.md).

The goal is one useful homepage for people seeking a beginner Rust course,
not separate pages for each wording of that query. Technical eligibility,
content relevance, search-result presentation, discovery, and measured search
outcomes are different things. This patch improves the first four; it does not
establish a ranking improvement or guarantee indexing or a #1 result.

## Baseline Findings

These source findings were checked against the baseline, not assumed from an
older audit. HTTP observations below are separate and do not prove which commit
production was running.

| Priority | Confirmed source finding and affected routes/files | Acceptance test |
| --- | --- | --- |
| P1 | `templates/base.html` had no standard description or canonical; social image URL was relative. | Each successful public HTML response has exactly one title, useful description, absolute canonical, and matching OG/Twitter metadata. |
| P1 | `templates/exercise.html` inherited a generic title and social description for every lesson. | All loaded public chapters have distinct titles/descriptions, including bonuses, quiz, and appendix. |
| P1 | `render_exercise_page` in `crates/server/src/main.rs` accepted both slug and file stem without consolidation. | GET/HEAD aliases return one 308 to the existing numbered URL; destination is 200; unknown lessons remain 404. |
| P1 | The Axum router registered neither root discovery endpoint. No repository proxy configuration supplied them. | GET/HEAD `/robots.txt` and `/sitemap.xml` return 200 with text/plain and application/xml respectively. Parse sitemap XML and compare inventory with public navigation. |
| P1 | Public and participant routes shared templates; loaded participant state alone could not express route indexing policy. | Application HTML has noindex, no personal head metadata, no public canonical/schema. Headers cover invalid-participant fallback, APIs, fragments, errors, and rejected methods too. |
| P2 | Homepage had a working demo and curriculum but scattered audience, price, setup, and signup information. | Anonymous initial HTML explains these together, retains crawlable curriculum links, visible enrollment guidance and author attribution; returning dashboard stays focused on progress. |
| P2 | Important lesson introductions and related-topic discovery could better support entry from search. | Initial HTML retains teaching prose and fallback code, uses existing navbar breadcrumbs and adds curriculum links, and preserves keys/anchors. |
| P2 | No Course or breadcrumb structured data. | Parse emitted JSON; verify canonical IDs, visible claims, and hostile-string HTML/script escaping independently of Google tools. |
| P2 | Workspace smoke tests expected anonymous alias URLs to render immediately. | Smoke workflow uses numbered URLs; dedicated response tests exercise aliases. CI performs deterministic local checks without Google requests. |

### Observed Production HTTP Behavior

Public requests on September 23, 2026, before deployment of this patch:

- GET `/` returned 200, with title “A Beginner's Guide to Rust | corrode”,
  no canonical, and no standard description.
- GET `/exercise/05_borrowing_and_ownership` and its unnumbered alias both
  returned 200 with title “corrode Rust Course”, no canonical or description.
- GET and HEAD `/robots.txt` and `/sitemap.xml` returned **404**. HEAD responses
  included `Cache-Control: no-store, must-revalidate`.
- A single homepage request transferred 55,333 body bytes and took about
  0.215 seconds in this environment. This is not a performance benchmark,
  Lighthouse score, or Core Web Vitals measurement.

No production participant URLs, submissions, or learner database were inspected.
There is no Search Console, keyword-volume, backlink, traffic, or ranking data
in this audit. Whether metadata changes improve CTR, canonical selection, or
ranking remains a hypothesis to test after deployment.

## Implemented Policy

`crates/server/src/seo.rs` owns trusted origin validation, metadata, the public
inventory, canonical lesson paths, sitemap XML, robots text, and script-safe
JSON serialization. `SITE_ORIGIN` defaults to `https://course.corrode.dev`.
Invalid explicit origins stop startup. Request Host/forwarding headers cannot
change metadata. Canonicals never include request queries or fragments.

`templates/base.html` owns all head metadata. Removed child overrides prevent
metadata drift and personal titles. Optional author-maintained descriptions in
`crates/server/src/lesson_seo.rs` are separate from `.chapter.toml`; future
chapters receive title-derived fallback metadata. No browser directives, bonus
rules, progress counts, storage keys, or submission identifiers changed.

The existing numbered lesson URLs remain canonical. The same path builder feeds
public curriculum, navigation, metadata, breadcrumbs, redirects, and sitemap.
GET/HEAD alias and trailing-slash normalization happen in one 308, retaining
functional queries. Mutation methods are not redirected. Unknown lessons are
not redirected to the homepage. Safe trailing slashes normalize before normal
route handling, so an unknown slash path can redirect to its slashless 404.

Indexable public pages are `/`, `/tour`, `/cheatsheet`, `/playground`, and all
loaded canonical chapters. The playground is retained because its server-rendered
starter and browser experimentation tool are useful public content. The current
sitemap contains 32 pages (28 chapters and four other pages). These are inventory
counts, **not coding-exercise or progress totals**. No new count summary is
shown on the homepage. Required/optional curriculum grouping remains data-driven.

All application routes are noindex by route class, including participant tours
and lessons whose participant lookup fails. Private HTML has generic metadata
and no canonical/social/schema URLs. Outer middleware covers fragment, API,
health, admin, error, and extractor responses. Existing dynamic no-store policy
and access checks remain intact. Successful static assets retain their existing
crawl/cache behavior; static errors are noindex.

The application owns the root robots/sitemap endpoints. Robots permits crawling,
including required assets and OAI-SearchBot, and advertises the absolute sitemap.
It does not disallow routes that must expose noindex. No GPTBot-specific policy
was found in the baseline repository or live robots endpoint, and none was added.
An owner-managed CDN/WAF training policy is outside this repository and must be
preserved at launch. Sitemap URLs come from the loaded catalog, not a parallel
chapter list. No invented lastmod, priority, or changefreq fields are emitted.

Homepage Course data uses stable absolute Course, Organization, and Person IDs.
It describes one free beginner course, with prerequisites and outcomes reflected
in the page. Chapters get BreadcrumbList, not invented Course records. JSON is
serialized with serde_json, then escapes `<`, `>`, `&`, and Unicode line/paragraph
separators before embedding. Askama handles ordinary attribute escaping.

### Access-Control Boundary

Participant links are bearer capabilities in the existing design. Someone with
a valid link can access its learner state; team views can expose team work as
already designed. Noindex is not authentication and cannot undo disclosure of a
link. Review access-log handling, referrers, retention, and capability sharing
separately with the owner. This patch neither redesigns authentication nor
inspects real learner records.

## Content and Browser Verification

The anonymous homepage preserves the original editorial headline, byline,
introduction, and runnable demo. The added start-button row, curriculum heading,
and count summary were removed to minimize homepage changes. Existing curriculum
links remain, with an anchor for return links. Enrollment guidance and author attribution
form a two-paragraph pre-footer with a locally served, lazy-loaded portrait
(400 × 400, 52,008 bytes). Its surface background adapts to light and dark themes.
The duplicate lesson/tour breadcrumb rows were removed; existing lesson navbar
navigation is retained, and BreadcrumbList names match its visible labels.
At the owner's request, chapter prose changes were reverted. A shared anonymous
banner now supplies course context and optional signup on chapter pages only.
The tour and participant views have no context banner. Plain links use the theme
color in both visited and unvisited states, and the curriculum return link is
smaller and centered beneath chapter navigation. The existing signup-on-pass
flow remains intact. Titles, social site name, and descriptions consistently use
“A Beginner's Guide to Rust”; lesson topics remain distinct.
README now leads with the live course and distinguishes local practice from
running the server. No license claim, testimonials, duration, or certificate was
invented. Author identity was checked at `https://endler.dev/about/` and
`https://corrode.dev/`; their consulting claims are not course endorsements.

Chrome 153 local checks used temporary SQLite fixtures and blocked external
browser requests. Homepage and lesson screenshots were inspected at 375 × 812
and 1440 × 900. A small mobile topbar fix eliminated measured lesson overflow
(427 → 375 px document width), retaining all controls and keyboard behavior;
320 px was checked too. Before restoring the original homepage wording, the
demo heading began at about 649 px on mobile and 613 px on desktop; these are
historical measurements, not positions in the revised page. The revised
pre-footer was visually checked at 375 and 1440 px in both themes: two paragraphs,
loaded portrait, no horizontal overflow, and immediately before the visible
footer. Navbar picker Enter/Escape and focus restoration passed in all four
combinations, with no duplicate breadcrumb row.

Measured asset baseline: all 18 generated JS files total 638,223 bytes, including
618,315 bytes of shared chunks; these remain byte-identical. HTMX is unchanged
at 51,238 bytes. CSS started at 55,032 bytes; the initial content and mobile fixes added
1,078 bytes before the pre-footer revision. The unchanged social image is 1280 × 640 and 70,201 bytes. The logo
is 200 × 85, rendered about 61 × 26 px. Neither social.png nor screenshot.jpg
loads as an in-page raster image on the checked homepage/lesson.

Before optimizing the cheatsheet, a local network trace measured 124,378 bytes
of unnecessary highlighting JS despite no applicable code blocks. The template
now checks for blocks before importing the highlighter. A rebuilt browser check
confirmed zero external JS requests on that page, a reduction of three requests
and 124,378 JS bytes (124,373 net response-body bytes). All 14 tables still
rendered without JS errors. No extra library was added. Editor mount measurements showed pre-existing fallback expansion on the
strings lesson (first editor 72 → 453 px mobile, 72 → 282 px desktop); no baseline
browser comparison was taken, so these do not establish a regression. They are
not CLS measurements. A narrow cheatsheet table can still overflow mobile
content (406 px document width at 375 px); record for a separate reference-table
layout pass rather than hiding content. No cache-policy relaxation was used.

## Automated Verification

- `make seo-check` passed: 17 metadata/response tests plus Python HTMLParser,
  JSON and XML parsing
  against an isolated server, with the production default origin.
- Rust fixtures cover every loaded public page, alias destinations, canonical
  host spoofing, queries, invalid-participant paths, fragment/API/admin classes,
  rejected methods, no-store, new chapter fallback, and hostile strings.
- Workspace smoke checks now compare curriculum and sitemap and inspect initial
  HTML without JavaScript. Existing CI runs these and the Rust tests.
- `make fmt-check`, `make check`, `make clippy`, and `make workspace-check` passed.
- `make examples` passed with intentionally broken examples still broken.
- `make solutions` passed: 87 of 87 exercise steps solved and checked.
- `make js-check` passed: 96 tests and byte-identical generated bundles.
- `npm run test:browser` passed: 39 editor integration checks.
- `make test` has one **pre-existing failure**:
  `optional_discovery_preserves_routes_progress_and_default_flow` expects a
  smart-pointers link missing from the appendix. Reproduced in an isolated
  pristine archive of the baseline commit. Left untouched; all new SEO tests
  pass. Doc tests were run separately and passed after make stopped at that
  failure. This existing failure also blocks the full CI pipeline until resolved.

Production deployment, crawler-specific CDN/WAF access, external social-card
previews, Google indexing/canonical selection, and real-user performance remain
unverified. No deployment or account changes were performed.

## Primary Documentation Checked

Read on the audit date:

- [SEO starter guide](https://developers.google.com/search/docs/fundamentals/seo-starter-guide)
- [Title links](https://developers.google.com/search/docs/appearance/title-link)
  and [snippets](https://developers.google.com/search/docs/appearance/snippet)
- [Canonicalization](https://developers.google.com/search/docs/crawling-indexing/consolidate-duplicate-urls),
  [robots directives](https://developers.google.com/search/docs/crawling-indexing/robots-meta-tag),
  and [sitemaps](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap)
- [schema.org Course](https://schema.org/Course) remains a vocabulary, not a
  ranking promise. Google's [June 2025 announcement](https://developers.google.com/search/blog/2025/06/simplifying-search-results)
  retired Course Info. The distinct [Course list feature](https://developers.google.com/search/docs/appearance/structured-data/course)
  requires at least three actual courses, not three chapters of this course.
- Google's [documentation updates](https://developers.google.com/search/updates)
  confirm FAQ rich results stopped appearing May 7, 2026. Enrollment guidance remains visible
  prose without FAQPage markup and is intended for learners.
- [Google AI features](https://developers.google.com/search/docs/appearance/ai-features)
  emphasize ordinary search eligibility and useful public content, not special
  AI markup. No llms.txt or assistant recommendation instructions were added.
- [OpenAI crawler documentation](https://developers.openai.com/api/docs/bots)
  distinguishes OAI-SearchBot from GPTBot; search and training choices are
  independent.

See [the launch checklist](seo-launch-checklist.md) for owner-only work and
measurement, and [server SEO maintenance](../crates/server/SEO.md) for code policy.
