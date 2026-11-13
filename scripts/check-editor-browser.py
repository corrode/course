#!/usr/bin/env python3
"""Run offline editor checks against the built bundle (headless Chrome and Node 22+)."""
import argparse
import functools
import json
import os
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import shutil
import signal
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
  // Headless targets can otherwise retain an active element in an unfocused document.
  await send('Emulation.setFocusEmulationEnabled', {enabled: true});
  await send('Page.navigate', {url});
  const deadline = Date.now() + 30000;
  let result;
  do {
    await new Promise(resolve => setTimeout(resolve, 100));
    const response = await send('Runtime.evaluate', {expression: '({status: document.querySelector("#checks")?.dataset.result, text: document.querySelector("#checks")?.textContent})', returnByValue: true});
    result = response.result.value;
  } while ((!result?.status || result.status === 'running') && Date.now() < deadline);
  if (result?.status === 'passed' && new URL(url).searchParams.has('layout')) {
    // Check computed accessible names, not title attributes or textContent:
    // display:none labels and accidental aria-label overrides must fail too.
    const {root} = await send('DOM.getDocument');
    const kind = new URL(url).searchParams.get('layout');
    const names = kind === 'index' ? [] : [
      ['reset-btn', /^Reset$/], ['format-btn', /^Format$/],
      ...(kind === 'exercise' ? [['vscode-btn', /^Open in Web Editor$/], ['copy-btn', /^Copy code$/]] : []),
    ];
    names.push(['run-btn', /\bRun\b/]);
    if (kind === 'exercise') names.push(['submit-btn', /\bSubmit(?:ted)?\b/]);
    try {
      for (const [role, expected] of names) {
        const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector: `.exercise-section [data-role="${role}"]`});
        if (!nodeId) throw new Error(`Missing accessible control: ${role}`);
        const {nodes} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
        const control = nodes[0];
        // Submit is intentionally hidden again after a later source revision.
        if (role === 'submit-btn' && control?.ignored) continue;
        if (!control || control.ignored || !expected.test(control.name?.value || '')) {
          throw new Error(`${role}: expected accessible name ${expected}, got ${JSON.stringify(control)}`);
        }
      }
      result.text += '\nProduction toolbar accessible names verified in Chrome accessibility tree.';
    } catch (error) {
      result = {status: 'failed', text: result.text + '\nFAILED: ' + error.message};
    }
  }
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


class BrowserStartupError(RuntimeError):
    pass


def find_chrome(override=None):
    explicit = override or os.environ.get("CHROME_BIN")
    if explicit:
        return shutil.which(explicit) or explicit
    for name in ("google-chrome", "google-chrome-stable", "chromium", "chromium-browser"):
        if executable := shutil.which(name):
            return executable
    return "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"


def browser_check(chrome, url, size, screenshot, motion):
    with tempfile.TemporaryDirectory(prefix="course-editor-browser-") as profile:
        command = [chrome, "--headless=new", "--disable-gpu", "--no-first-run", "--disable-background-networking", "--remote-debugging-port=0", f"--user-data-dir={profile}", "about:blank"]
        # A file avoids filling a pipe while Chrome runs; retain stderr for
        # launch failures (notably distro wrappers that exit before starting).
        with tempfile.TemporaryFile(mode="w+t") as stderr:
            try:
                browser = subprocess.Popen(command, stdout=subprocess.DEVNULL, stderr=stderr, start_new_session=True)
            except OSError as error:
                raise BrowserStartupError(f"Could not start Chrome: {error}") from error
            try:
                port_file = Path(profile) / "DevToolsActivePort"
                deadline = time.monotonic() + 30
                port = None
                while port is None:
                    try:
                        lines = port_file.read_text().splitlines()
                        port = lines[0] if lines else None
                    except FileNotFoundError:
                        pass
                    if port is not None:
                        break
                    exit_code = browser.poll()
                    if exit_code is not None or time.monotonic() >= deadline:
                        reason = (f"Chrome exited before opening its debugging port (exit code {exit_code})"
                                  if exit_code is not None else
                                  "Chrome did not open its debugging port within 30 seconds (process still running)")
                        stderr.seek(0)
                        raise BrowserStartupError(f"{reason}\nChrome stderr:\n{stderr.read().strip() or '(empty)'}")
                    time.sleep(0.05)
                while True:
                    remaining = deadline - time.monotonic()
                    if remaining <= 0:
                        stderr.seek(0)
                        raise BrowserStartupError(
                            f"Chrome opened its debugging port but no page target became ready within 30 seconds "
                            f"(exit code {browser.poll()})\nChrome stderr:\n{stderr.read().strip() or '(empty)'}"
                        )
                    try:
                        with urlopen(f"http://127.0.0.1:{port}/json/list", timeout=min(5, remaining)) as response:
                            target = next((page for page in json.load(response)
                                           if page["type"] == "page" and page.get("webSocketDebuggerUrl")), None)
                    except TimeoutError:
                        # The port can appear before DevTools serves requests.
                        # Retry readiness only, within the shared startup budget.
                        target = None
                    if target is not None:
                        break
                    time.sleep(min(0.05, remaining))
                result = subprocess.run(["node", "--input-type=module", "-e", CDP_DRIVER, target["webSocketDebuggerUrl"], url, str(size[0]), str(size[1]), str(screenshot.resolve()) if screenshot else "", motion], capture_output=True, text=True, timeout=45)
                if result.returncode:
                    raise RuntimeError(result.stderr)
                return json.loads(result.stdout)
            finally:
                # Chrome helpers may outlive the launcher and keep writing to
                # the profile. Stop the whole isolated group before deleting it.
                try:
                    os.killpg(browser.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
                try:
                    browser.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    pass
                finally:
                    try:
                        os.killpg(browser.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    browser.wait(timeout=5)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--chrome", help="Chrome executable (overrides CHROME_BIN and automatic discovery)")
    parser.add_argument("--screenshot", type=Path, help="Screenshot of the workflow fixture")
    parser.add_argument("--screenshot-dir", type=Path, help="Capture production layout fixtures")
    parser.add_argument("--suite", choices=["all", "workflow", "layout"], default="all")
    parser.add_argument("--template", choices=["exercise", "playground", "tour", "index"], help="Limit layout checks to one template")
    parser.add_argument("--motion", choices=["reduce", "no-preference"], default="reduce")
    args = parser.parse_args()
    chrome = find_chrome(args.chrome)
    print(f"Chrome executable: {chrome}", flush=True)
    try:
        version = subprocess.run([chrome, "--version"], capture_output=True, text=True, timeout=10)
    except (OSError, subprocess.TimeoutExpired) as error:
        print(f"Could not query Chrome version: {error}")
        return 1
    print(f"Chrome version: {version.stdout.strip() or '(no stdout)'}", flush=True)
    if version.stderr.strip():
        print(f"Chrome version stderr:\n{version.stderr.strip()}")
    if version.returncode:
        print(f"Chrome version check failed (exit code {version.returncode}); no browser cases started")
        return 1
    root = Path(__file__).resolve().parent.parent
    server = ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(QuietHandler, directory=str(root)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    try:
        cases = []
        if args.suite != "layout":
            cases.append(("workflow", "", (1440, 1400), args.screenshot))
        if args.suite != "workflow":
            for template in [args.template] if args.template else ["exercise", "playground", "tour", "index"]:
                for width, height in [(320, 500), (375, 500), (800, 450), (1440, 900)]:
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
                result = browser_check(chrome, url, (width, height), screenshot, args.motion)
            except BrowserStartupError as error:
                print(f"{name}: {error}\nStopping: Chrome cannot start; remaining browser cases skipped")
                return 1
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
