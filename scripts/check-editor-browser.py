#!/usr/bin/env python3
"""Run offline editor integration checks against the built bundle in headless Chrome."""
import argparse
import functools
from html.parser import HTMLParser
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import shutil
import subprocess

import threading


class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *_args):
        pass


class Results(HTMLParser):
    def __init__(self):
        super().__init__()
        self.status = None
        self.inside = False
        self.text = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "pre" and attrs.get("id") == "checks":
            self.status = attrs.get("data-result")
            self.inside = True

    def handle_endtag(self, tag):
        if tag == "pre":
            self.inside = False

    def handle_data(self, data):
        if self.inside:
            self.text.append(data)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--chrome", default=shutil.which("chromium") or shutil.which("google-chrome") or "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    parser.add_argument("--screenshot", type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    server = ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(QuietHandler, directory=str(root)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    try:
        # Headless Chrome creates its own temporary profile when none is specified.
        command = [args.chrome, "--headless=new", "--disable-gpu", "--no-first-run", "--disable-background-networking", "--window-size=1440,1400", "--virtual-time-budget=10000", "--dump-dom"]
        if args.screenshot:
            command.append(f"--screenshot={args.screenshot.resolve()}")
        command.append(f"http://127.0.0.1:{server.server_port}/scripts/editor-browser.html")
        try:
            result = subprocess.run(command, capture_output=True, text=True, timeout=60)
        except subprocess.TimeoutExpired as error:
            print("Chrome did not finish within 60 seconds.")
            print(error.stdout or "")
            print(error.stderr or "")
            return 1
        results = Results()
        results.feed(result.stdout)
        print("".join(results.text) or "Browser checks did not load.")
        if result.returncode != 0 or results.status != "passed":
            print(result.stderr)
            return 1
        return 0
    finally:
        server.shutdown()
        server.server_close()


if __name__ == "__main__":
    raise SystemExit(main())
