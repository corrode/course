"""Offline response checks shared by workspace smoke tests (standard library only)."""
from html.parser import HTMLParser
import json
import re
from urllib.parse import urlsplit
import xml.etree.ElementTree as ET

ORIGIN = "https://course.corrode.dev"


class Page(HTMLParser):
    def __init__(self, source):
        super().__init__(convert_charrefs=True)
        self.titles = []
        self.meta = {}
        self.canonicals = []
        self.links = []
        self.schemas = []
        self.images = []
        self.text = []
        self.noncontent = 0
        self.capture = None
        self.buffer = []
        self.feed(source)

    def handle_starttag(self, tag, attrs):
        if not re.fullmatch(r"[a-z][a-z0-9-]*", tag):
            raise RuntimeError("SEO: malformed rendered HTML tag")
        attrs = dict(attrs)
        if tag in ("script", "style"):
            self.noncontent += 1
        if tag == "img":
            self.images.append(attrs)
        if tag == "title" or (tag == "script" and attrs.get("type") == "application/ld+json"):
            self.capture = tag
            self.buffer = []
        if tag == "meta":
            self.meta.setdefault(attrs.get("name", attrs.get("property")), []).append(attrs.get("content", ""))
        if tag == "link" and attrs.get("rel") == "canonical":
            self.canonicals.append(attrs.get("href"))
        if tag == "a":
            self.links.append(attrs.get("href", ""))

    def handle_data(self, data):
        if self.capture:
            self.buffer.append(data)
        if not self.noncontent:
            self.text.append(data)

    def handle_endtag(self, tag):
        if tag in ("script", "style"):
            self.noncontent = max(0, self.noncontent - 1)
        if tag == self.capture:
            text = "".join(self.buffer)
            if tag == "title":
                self.titles.append(text)
            else:
                self.schemas.append(json.loads(text))
            self.capture = None


def verify(get):
    """Inspect initial HTML and parse sitemap XML, without executing JavaScript."""
    def require(condition, message):
        if not condition:
            raise RuntimeError("SEO: " + message)

    robots = get("/robots.txt", "text/plain").decode()
    require(f"Sitemap: {ORIGIN}/sitemap.xml" in robots, "robots sitemap URL")
    root = ET.fromstring(get("/sitemap.xml", "application/xml"))
    ns = "{http://www.sitemaps.org/schemas/sitemap/0.9}"
    require(root.tag == ns + "urlset", "sitemap namespace")
    urls = [node.text for node in root.findall(f"{ns}url/{ns}loc")]
    require(len(urls) == len(set(urls)) and len(urls) > 4, "unique sitemap inventory")
    require(not root.findall(f".//{ns}lastmod"), "no invented lastmod")
    home = Page(get("/", "text/html").decode())
    curriculum = {ORIGIN + urlsplit(link).path for link in home.links if link.startswith("/exercise/")}
    lessons = {url for url in urls if urlsplit(url).path.startswith("/exercise/")}
    require(curriculum == lessons, "curriculum and sitemap lesson inventories differ")
    titles, descriptions = set(), set()
    for url in urls:
        parsed = urlsplit(url)
        require(url.startswith(ORIGIN + "/") and not parsed.query and not parsed.fragment, "clean production URLs")
        path = parsed.path
        page = Page(get(path + "?utm_source=fixture", "text/html").decode())
        require(len(page.titles) == 1 and page.titles[0] not in titles, "one distinct title: " + path)
        titles.update(page.titles)
        require("A Beginner's Guide to Rust" in page.titles[0], "consistent course title: " + path)
        require(page.meta.get("og:site_name") == ["A Beginner's Guide to Rust"], "course social identity: " + path)
        description = page.meta.get("description", [])
        require(len(description) == 1 and len(description[0]) > 40 and description[0] not in descriptions, "distinct useful description: " + path)
        descriptions.update(description)
        require("A Beginner's Guide to Rust" in description[0], "course description: " + path)
        require(page.canonicals == [url], "canonical: " + path)
        require(page.meta.get("og:url") == [url], "social URL: " + path)
        for prefix in ("og", "twitter"):
            require(page.meta.get(prefix + ":title") == page.titles, "social title: " + path)
            require(page.meta.get(prefix + ":description") == description, "social description: " + path)
            require(page.meta.get(prefix + ":image") == [ORIGIN + "/static/assets/social.png"], "absolute image: " + path)
        if path.startswith("/exercise/"):
            require(any(link in ("/#curriculum", "/") for link in page.links), "crawlable curriculum return")
            require("exercise-section" in get(path, "text/html").decode() or len(" ".join(page.text)) > 1000, "server rendered teaching content")
            require(len(page.schemas) == 1 and page.schemas[0]["@type"] == "BreadcrumbList", "lesson schema")
            require(page.schemas[0]["itemListElement"][1]["item"] == url, "breadcrumb canonical")
    require(len(home.schemas) == 1 and home.schemas[0]["@type"] == "Course", "homepage course schema")
    require(home.schemas[0]["@id"] == ORIGIN + "/#course", "stable course identity")
    require(bool(curriculum), "crawlable course entry through existing curriculum")
    require(any(image.get("src") == "/static/assets/matthias-endler.jpg"
                and image.get("alt") == "Matthias Endler"
                and image.get("width") == image.get("height") == "400"
                and image.get("loading") == "lazy" for image in home.images),
            "sized, lazy-loaded author portrait")
    require("You can start without an account." in " ".join(home.text),
            "visible enrollment information")
    print(f"PASS: SEO HTML/JSON parsing and XML sitemap ({len(urls)} public pages)")
