# SEO Launch Checklist

Prepared September 23, 2026. Nothing below implies deployment, Search Console
verification, submission, outreach, or a ranking improvement has happened.
Baseline and local validation are in [seo-audit.md](seo-audit.md).

## Before Deployment

- [ ] Review the focused patch and confirm CI passes.
- [ ] Run `make seo-check` and the repository's scoped checks. Do not substitute
  an all-target build that compiles deliberately broken teaching examples.
- [ ] Confirm the deployment's `SITE_ORIGIN` is `https://course.corrode.dev`
  (the default). It must be an HTTP(S) origin, with no credentials, query,
  fragment, or path. Invalid explicit values fail startup. A preview deployment
  needs separate access/indexing protection; changing the origin is not noindex.
- [ ] Confirm the proxy forwards root `/robots.txt` and `/sitemap.xml` to Axum.
  No repository deployment rule owned them before this patch; both live endpoints
  returned 404 on the audit date. A static-directory file is not sufficient.
- [ ] Preserve any owner-managed GPTBot/training crawler choices at the CDN/WAF.
  No such rule was found in this repository or the live robots endpoint.
  Do not remove a training restriction just to allow OAI-SearchBot.
- [ ] Confirm public Googlebot and OAI-SearchBot access, including assets, using
  the provider's documented validation/IP procedure. A spoofed User-Agent alone
  does not establish what real crawlers can fetch. Do not bypass private access
  checks or expose learner URLs while testing.
- [ ] Review logs, referrer policy, and retention for bearer-link routes separately.
  Robots directives do not make participant links confidential. Keep no-store
  and existing access checks. Do not paste learner tokens, links, or source into
  tickets, analytics exports, screenshots, or public reports.

Deployment record (owner fills in):

| Event | Date/time (UTC) | Commit / note |
| --- | --- | --- |
| Pre-release baseline exported | Not recorded | Owner action |
| Deployed | Not deployed by this task | Owner action |
| Production checks completed | Not recorded | Owner action |
| Search Console sitemap submitted | Not submitted | Owner action |
| First post-release review | Not scheduled | Owner action |

## Production Response Checks

Use normal GET and HEAD requests after deployment. Inspect initial HTML rather
than only the live DOM. Record status, content type, canonical, indexing header,
and cache policy without retaining sensitive response bodies.

- [ ] `/`, `/tour`, `/cheatsheet`, `/playground`: 200, one descriptive title,
  description, absolute self-canonical, matching OG/Twitter, usable content.
- [ ] Representative numbered lessons: strings, ownership/borrowing, Result,
  lifetimes, a bonus chapter, quiz, and appendix. Each has a distinct title and
  description, server-rendered prose/navigation, appropriate fallback code, and
  public breadcrumbs matching its JSON-LD. All sitemap URLs should return 200.
- [ ] A recognized unnumbered alias and its trailing-slash variant: one 308 to
  the numbered URL, then 200. Functional queries survive the redirect but never
  appear in metadata. Unknown lessons return a real 404, not a homepage soft 404.
- [ ] `/robots.txt`: 200 text/plain, allows public content/assets, declares
  `https://course.corrode.dev/sitemap.xml`. No disallow hides a needed noindex.
- [ ] `/sitemap.xml`: 200 application/xml with the sitemap namespace. Includes
  all loaded public chapters, including bonuses. No private paths, aliases,
  queries, fragments, redirects, invented lastmod, priority, or changefreq.
- [ ] `/signup`, team enrollment, settings, dashboards, participant lessons/tours,
  admin, API, health, fragments, and error responses are noindex. Use owner-created
  disposable fixtures privately, including an invalid participant fallback.
  Private HTML has no personal head metadata, canonical or structured data.
- [ ] Confirm dynamic `Cache-Control: no-store, must-revalidate` survives proxy
  handling. Public static assets should not accidentally become no-store/noindex.
- [ ] Parse JSON-LD independently, checking Course/provider/author IDs, types,
  supported properties and visible claims. Use a schema.org validator if desired.
  Rich Results Test support is a different question: absence of a Google
  enhancement does not invalidate schema.org Course.
- [ ] Check the retained social image through normal public requests and social
  preview tools. No social platform preview was verified during implementation.
- [ ] Repeat mobile and desktop keyboard/layout/editor checks on production.
  Monitor actual field LCP, INP, and CLS if available through Search Console or
  CrUX. Local byte counts and mount movement are not real-user Core Web Vitals.
  Follow up separately on lesson editor fallback expansion and the narrow-screen
  cheatsheet table. Do not hide teaching content to improve a lab score.

## Search Console (Owner Access Required)

- [ ] Verify an appropriate Domain property or exact HTTPS URL-prefix property.
  Use the owner's real DNS/HTML verification workflow. No token is supplied here.
- [ ] Submit the absolute sitemap once through Search Console; use normal
  recrawling/submission rather than obsolete sitemap-ping endpoints.
- [ ] Inspect homepage, a core lesson, a bonus, tour, and cheatsheet. Compare
  declared and Google-selected canonical; check live fetch/rendering and crawl
  permissions. Request indexing of representative changed public pages if useful.
- [ ] Inspect non-indexable route classes with authorized disposable fixtures.
  Confirm Google can observe noindex without publishing or sharing sensitive URLs.
- [ ] Review Page Indexing and sitemap processing after crawlers revisit. Do not
  interpret “discovered” as indexed, or “indexed” as ranked first.
- [ ] Track unexpected alternate canonicals, soft 404s, blocked resources, or
  duplicate private pages. Fix causes rather than adding contradictory robots
  disallow and noindex directives.

## Measurement Plan

No numerical search baseline was available to this task. Export it before
release, then compare complete, equal-length periods (for example 28 days before
and 28 days after, allowing time for recrawling). Compare year-over-year where
available, annotate curriculum/marketing changes, and account for seasonality,
query mix, device mix, and ranking variability. Review early technical issues
weekly; avoid attributing every daily ranking change to this patch.

Keep these measurements separate:

1. **Eligibility:** crawl status, response codes, noindex policy, sitemap health,
   canonical selection and indexing coverage.
2. **Relevance/presentation:** representative titles/snippets and landing-page
   alignment. Google may rewrite title links or choose a different snippet.
3. **Discovery/outcomes:** non-brand impressions, clicks, CTR and average position
   from Search Console, segmented by query, page, country and device. Keep the
   homepage's beginner/free/interactive/online-learning intent together, and
   analyze lesson-specific queries separately. Define the brand exclusion
   consistently (for example corrode and Matthias Endler variants).
4. **Learner activation:** first chapter interaction and first successful exercise
   from the existing first-party events, reported as aggregates only.

For a “#1” objective, record an explicit definition such as: first ordinary
organic web result for the exact query **beginner Rust course**, in a specified
country, language and device category, on a specified date. Do not count a paid
placement, a personalized screenshot, an LLM recommendation, or a Search Console
average-position value as universal proof of #1. No ranking is promised.

### Existing Activation Signals

Reviewed `static/js/analytics.js`, server event handling and
[analytics.md](analytics.md). No tracker or new event schema was added.

- `chapter_view` and `editor_focus` already provide a course-start proxy. Define
  a start as a tab session interacting with a real exercise chapter; exclude
  `dashboard`, warmup and tour from that definition. The new homepage start-link
  click is **not** a separately instrumented event, and navigation without JS
  is not measured by UI events.
- `exercise_run` with `result = 'passed'` and positive test totals can measure
  first successful tested exercise per session (or authorized participant
  aggregation). A successful hello-world `ran`/`no_tests` warmup is not an
  exercise completion. Existing backend progress remains authoritative for
  submitted chapter completion.
- Tab-scoped session IDs cannot establish unique people across devices or tabs.
  Without referral/channel data these events cannot attribute an activation to
  a Google query or organic visit. Do not claim a search-to-activation funnel
  that current data cannot support.
- Keep reports aggregate, use authorized read-only backups, and retain no raw
  code, token-bearing URLs, fingerprint, or third-party tracking identifier.
  Record deployment commit/course version for comparable event cohorts.

## Editorial Discovery, Outside This Repository

The first-party `https://corrode.dev/` homepage already contained a normal link
to `https://course.corrode.dev/` when checked on September 23, 2026. Recheck it
and review relevant training/resource pages with that site's owner. No changes
to that repository or outreach were performed.

A factual resource description for an editor to adapt:

> A free, interactive Rust course by Matthias Endler at corrode for developers
> who already know another programming language. Lessons combine explanations,
> browser exercises, test feedback, hints, and solutions. Start without signup,
> or use the optional progress and local CLI workflow.

Consider a relevant Rust learning-resource list, community learning thread, or
newsletter only when its editorial/submission rules allow it. Disclose the
author connection, submit once, and accept editorial decisions. No paid links,
fake reviews, repeated promotion, manufactured endorsements, or AI-only pages.
Do not turn chapters into separate courses to claim Course list eligibility.

Google's ordinary public-content and search guidance is the focus. No llms.txt
work is prioritized; such a file is not a Google visibility or ranking lever.
Search access and training permissions are separate owner decisions. Discovery
work, technical changes, and measured outcomes must remain separate in reports.
