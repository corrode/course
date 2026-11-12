#!/usr/bin/env python3
"""Run offline editor checks against the built bundle (headless Chrome and Node 22+)."""
import argparse
import functools
import json
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import shutil
import subprocess
import tempfile
import threading
import time
from urllib.request import urlopen


# Node's built-in WebSocket avoids a browser automation dependency. Unlike a
# virtual-time dump, this lets ResizeObserver, animation frames and native scrolls
# settle before evaluating geometry and taking screenshots.
CDP_DRIVER = r"""
import {writeFile} from 'node:fs/promises';
const [socket, url, width, height, screenshot, motion] = process.argv.slice(1);
const ws = new WebSocket(socket);
await new Promise((resolve, reject) => { ws.onopen = resolve; ws.onerror = reject; });
let id = 0;
const pending = new Map();
ws.onmessage = event => {
  const message = JSON.parse(event.data);
  if (!message.id) return;
  const call = pending.get(message.id);
  pending.delete(message.id);
  if (message.error) call.reject(new Error(JSON.stringify(message.error)));
  else call.resolve(message.result);
};
const send = (method, params = {}) => new Promise((resolve, reject) => {
  pending.set(++id, {resolve, reject});
  ws.send(JSON.stringify({id, method, params}));
});
try {
  await send('Emulation.setDeviceMetricsOverride', {width: +width, height: +height, deviceScaleFactor: 1, mobile: false});
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: motion}]});
  await send('Page.navigate', {url});
  const deadline = Date.now() + 30000;
  let result;
  do {
    await new Promise(resolve => setTimeout(resolve, 100));
    const response = await send('Runtime.evaluate', {expression: '({status: document.querySelector("#checks")?.dataset.result, text: document.querySelector("#checks")?.textContent})', returnByValue: true});
    result = response.result.value;
  } while ((!result?.status || result.status === 'running') && Date.now() < deadline);
  if (screenshot) {
    const image = await send('Page.captureScreenshot');
    await writeFile(screenshot, Buffer.from(image.data, 'base64'));
  }
  console.log(JSON.stringify(result || {status: 'failed', text: 'Browser checks did not load.'}));
} finally {
  ws.close();
}
"""


class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *_args):
        pass


def browser_check(chrome, url, size, screenshot, motion):
    with tempfile.TemporaryDirectory(prefix="course-editor-browser-") as profile:
        command = [chrome, "--headless=new", "--disable-gpu", "--no-first-run", "--disable-background-networking", "--remote-debugging-port=0", f"--user-data-dir={profile}", "about:blank"]
        with subprocess.Popen(command, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL) as browser:
            try:
                port_file = Path(profile) / "DevToolsActivePort"
                deadline = time.monotonic() + 10
                while not port_file.exists():
                    if browser.poll() is not None or time.monotonic() >= deadline:
                        raise RuntimeError("Chrome did not open its debugging port within 10 seconds")
                    time.sleep(0.05)
                port = port_file.read_text().splitlines()[0]
                with urlopen(f"http://127.0.0.1:{port}/json/list", timeout=5) as response:
                    target = next(page for page in json.load(response) if page["type"] == "page")
                result = subprocess.run(["node", "--input-type=module", "-e", CDP_DRIVER, target["webSocketDebuggerUrl"], url, str(size[0]), str(size[1]), str(screenshot.resolve()) if screenshot else "", motion], capture_output=True, text=True, timeout=45)
                if result.returncode:
                    raise RuntimeError(result.stderr)
                return json.loads(result.stdout)
            finally:
                browser.terminate()
                try:
                    browser.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    browser.kill()
                    browser.wait()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--chrome", default=shutil.which("chromium") or shutil.which("google-chrome") or "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    parser.add_argument("--screenshot", type=Path, help="Screenshot of the workflow fixture")
    parser.add_argument("--screenshot-dir", type=Path, help="Capture production layout fixtures")
    parser.add_argument("--suite", choices=["all", "workflow", "layout"], default="all")
    parser.add_argument("--template", choices=["exercise", "playground", "tour", "index"], help="Limit layout checks to one template")
    parser.add_argument("--motion", choices=["reduce", "no-preference"], default="reduce")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    server = ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(QuietHandler, directory=str(root)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    try:
        cases = []
        if args.suite != "layout":
            cases.append(("workflow", "", (1440, 1400), args.screenshot))
        if args.suite != "workflow":
            for template in [args.template] if args.template else ["exercise", "playground", "tour", "index"]:
                for width, height in [(375, 500), (800, 450), (1440, 900)]:
                    name = f"{template}-{width}x{height}"
                    screenshot = args.screenshot_dir / f"{name}.png" if args.screenshot_dir else None
                    cases.append((name, f"?layout={template}", (width, height), screenshot))
            if args.screenshot_dir and args.template in [None, "playground"]:
                cases.append(("playground-fullscreen", "?layout=playground&fullscreen=1", (375, 500), args.screenshot_dir / "playground-fullscreen.png"))
        failed = False
        for name, query, (width, height), screenshot in cases:
            if screenshot:
                screenshot.parent.mkdir(parents=True, exist_ok=True)
            url = f"http://127.0.0.1:{server.server_port}/scripts/editor-browser.html{query}"
            try:
                result = browser_check(args.chrome, url, (width, height), screenshot, args.motion)
            except (RuntimeError, subprocess.TimeoutExpired) as error:
                print(f"{name}: {error}")
                failed = True
                continue
            text = result.get("text") or "Browser checks did not load."
            if result.get("status") != "passed":
                print(f"{name}: {text}")
                failed = True
            else:
                print(f"{name}: {text.splitlines()[0]}")
        return int(failed)
    finally:
        server.shutdown()
        server.server_close()


if __name__ == "__main__":
    raise SystemExit(main())
