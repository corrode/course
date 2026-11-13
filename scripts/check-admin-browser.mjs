// Build first: cargo build --locked -p course-server --bin server
// Node 24+, Chrome, no npm dependencies. See README.md for browser selection.
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { constants } from "node:fs";
import { access, mkdir, mkdtemp, readFile, rm, symlink, writeFile } from "node:fs/promises";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { delimiter, join, resolve } from "node:path";
import { DatabaseSync } from "node:sqlite";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: {
  chrome: { type: "string" },
  "screenshot-dir": { type: "string" },
} });
const root = fileURLToPath(new URL("../", import.meta.url));
const token = "browser-only&plus+question?hash#percent%";
const query = new URLSearchParams({ token });
const team = "T".repeat(64);
const names = ["Alice", "Bob", "W".repeat(128)];
const alice = "01J00000000000000000000001";
const delay = (ms) => new Promise((done) => setTimeout(done, ms));

async function findChrome() {
  const explicit = values.chrome ?? process.env.CHROME_BIN;
  const candidates = explicit ? [resolve(explicit)] : [
    ...(process.env.PATH ?? "").split(delimiter).filter(Boolean).flatMap((dir) =>
      ["google-chrome", "google-chrome-stable", "chromium", "chromium-browser"].map((name) => join(dir, name))),
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/Applications/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing",
  ];
  for (const path of candidates) {
    try { await access(path, constants.X_OK); return path; } catch { /* next candidate */ }
  }
  throw new Error("Chrome not found. Pass --chrome=/path/to/chrome or set CHROME_BIN to an executable path.");
}

const chrome = await findChrome();
const server = resolve(root, "target/debug/server");
await access(server, constants.X_OK).catch(() => {
  throw new Error("Build the server first: cargo build --locked -p course-server --bin server");
});
const scratch = await mkdtemp(join(tmpdir(), "course-admin-browser-"));
const env = { PATH: process.env.PATH, HOME: scratch, TMPDIR: scratch, LANG: "en_US.UTF-8" };
const children = [];
const pending = new Map();
const requests = [], responses = [], browserErrors = [];
let socket, db, base, logs = "", childFailure, cleanupPromise, nextId = 0;
console.log(`Temporary runtime: ${scratch}`);

function launch(label, command, args, extraEnv = {}) {
  const child = spawn(command, args, {
    cwd: scratch, env: { ...env, ...extraEnv }, detached: true, stdio: ["ignore", "pipe", "pipe"],
  });
  children.push(child);
  child.closed = new Promise((done) => child.once("close", done));
  for (const stream of [child.stdout, child.stderr]) {
    stream.on("data", (data) => { logs = (logs + `[${label}] ${data}`).slice(-16000); });
  }
  child.on("error", (error) => { childFailure = error; });
  child.on("exit", (code, signal) => { childFailure = new Error(`${label} exited (${signal ?? code})`); });
  console.log(`Started ${label}: ${child.pid}`);
}
async function until(fn, label, deadline = Date.now() + 10000) {
  let last;
  while (Date.now() < deadline) {
    if (childFailure) throw childFailure;
    try { const value = await fn(); if (value) return value; } catch (error) { last = error; }
    await delay(50);
  }
  throw new Error(`Timed out: ${label}`, { cause: last });
}
function killGroup(child, signal) {
  if (!child.pid) return;
  try { process.kill(-child.pid, signal); } catch (error) { if (error.code !== "ESRCH") throw error; }
}
async function cleanup() {
  if (cleanupPromise) return cleanupPromise;
  cleanupPromise = (async () => {
    socket?.close();
    for (const request of pending.values()) {
      clearTimeout(request.timer);
      request.reject(new Error("Browser test shutting down"));
    }
    pending.clear();
    db?.close();
    for (const child of children.toReversed()) {
      // Kill the group even if its leader exited: Chrome also spawns helpers.
      killGroup(child, "SIGTERM");
      await Promise.race([child.closed, delay(1500)]);
      killGroup(child, "SIGKILL");
      await Promise.race([child.closed, delay(1500)]);
    }
    await rm(scratch, { recursive: true, force: true, maxRetries: 3, retryDelay: 200 });
    console.log("Cleaned up server, Chrome, and temporary runtime");
  })();
  return cleanupPromise;
}
async function stop(reason, code) {
  console.error(reason);
  try { await cleanup(); } finally { process.exit(code); }
}
const watchdog = setTimeout(() => void stop("Browser test exceeded 120 seconds", 1), 120000);
for (const [signal, code] of [["SIGINT", 130], ["SIGTERM", 143]]) {
  process.once(signal, () => void stop(signal, code));
}

function cdp(method, params = {}) {
  return new Promise((resolve, reject) => {
    const id = ++nextId;
    const timer = setTimeout(() => { pending.delete(id); reject(new Error(`CDP timeout: ${method}`)); }, 8000);
    pending.set(id, { resolve, reject, timer });
    socket.send(JSON.stringify({ id, method, params }));
  });
}
async function js(expression) {
  const result = await cdp("Runtime.evaluate", { expression, returnByValue: true, awaitPromise: true });
  if (result.exceptionDetails) throw new Error(JSON.stringify(result.exceptionDetails));
  return result.result.value;
}
async function key(key, number) {
  for (const type of ["keyDown", "keyUp"]) {
    await cdp("Input.dispatchKeyEvent", {
      type, key, code: key, windowsVirtualKeyCode: number,
      text: type === "keyDown" && key === "Enter" ? "\r" : "",
    });
  }
}
async function click(selector) {
  const point = await js(`(() => {
    const el = document.querySelector(${JSON.stringify(selector)});
    el.scrollIntoView({block: 'center', inline: 'center'});
    const r = el.getBoundingClientRect();
    return {x: r.x + r.width / 2, y: r.y + r.height / 2};
  })()`);
  for (const type of ["mousePressed", "mouseReleased"]) {
    await cdp("Input.dispatchMouseEvent", { type, ...point, button: "left", clickCount: 1 });
  }
}
async function typeInto(selector, text) {
  await click(selector);
  await js(`document.querySelector(${JSON.stringify(selector)}).select()`);
  if (text) await cdp("Input.insertText", { text });
  else await key("Backspace", 8);
}
async function arrived(path) {
  await until(() => js(`!window.oldDocument && document.readyState === 'complete'
    && location.pathname === ${JSON.stringify(path)}`), `navigation to ${path}`);
  assert.equal(await js("new URL(location.href).searchParams.get('token')"), token);
  await js("document.fonts.ready.then(() => true)");
}
async function follow(selector, path) {
  assert.equal(new URL(await js(`document.querySelector(${JSON.stringify(selector)}).href`)).searchParams.get("token"), token);
  await js("window.oldDocument = true");
  await click(selector);
  await arrived(path);
}

try {
  // dotenv searches parent directories unless it finds a file. Stop it here.
  await writeFile(join(scratch, ".env"), "");
  for (const dir of ["examples", "solutions", "migrations", "static"]) {
    await symlink(join(root, dir), join(scratch, dir), "dir");
  }
  const reservation = createServer();
  await new Promise((done, reject) => { reservation.once("error", reject); reservation.listen(0, "127.0.0.1", done); });
  const port = reservation.address().port;
  await new Promise((done) => reservation.close(done));
  base = `http://127.0.0.1:${port}`;
  const database = join(scratch, "isolated.sqlite");
  launch("server", server, [], {
    PORT: String(port), DATABASE_URL: `sqlite:${database}`, CORRODE_ADMIN_TOKEN: token, SITE_ORIGIN: base, RUST_LOG: "warn",
  });
  await until(async () => (await fetch(`${base}/health`, { signal: AbortSignal.timeout(1000) })).ok, "server health");
  db = new DatabaseSync(database);
  db.exec("PRAGMA busy_timeout = 3000");
  const insert = db.prepare("INSERT INTO participants (id, name, team_token) VALUES (?, ?, ?)");
  names.forEach((name, index) => insert.run(index === 0 ? alice : `browser-member-${index}`, name, team));

  const chromeDeadline = Date.now() + 30000;
  launch("Chrome", chrome, ["--headless=new", "--disable-gpu", "--no-first-run", "--no-default-browser-check",
    "--disable-background-networking", "--disable-component-update", "--disable-sync", "--disable-dev-shm-usage",
    "--use-mock-keychain", "--password-store=basic", "--no-proxy-server",
    "--host-resolver-rules=MAP * ~NOTFOUND, EXCLUDE 127.0.0.1", "--remote-debugging-address=127.0.0.1",
    "--remote-debugging-port=0", `--user-data-dir=${scratch}/chrome`, "about:blank"]);
  const debugPort = await until(async () => (await readFile(join(scratch, "chrome/DevToolsActivePort"), "utf8")).split("\n")[0], "Chrome debugger", chromeDeadline);
  // The debug port may exist before DevTools serves a page. Share one startup
  // budget across port discovery, page readiness, and the WebSocket connection.
  const pageSocket = await until(async () => {
    const timeout = Math.max(1, Math.min(3000, chromeDeadline - Date.now()));
    const pages = await (await fetch(`http://127.0.0.1:${debugPort}/json/list`, { signal: AbortSignal.timeout(timeout) })).json();
    return pages.find((page) => page.type === "page")?.webSocketDebuggerUrl;
  }, "Chrome page target", chromeDeadline);
  socket = new WebSocket(pageSocket);
  await until(() => socket.readyState === WebSocket.OPEN, "CDP connection", chromeDeadline);
  socket.addEventListener("message", ({ data }) => {
    const message = JSON.parse(data), p = message.params;
    if (message.method === "Fetch.requestPaused") {
      const local = new URL(p.request.url).origin === base;
      void cdp(local ? "Fetch.continueRequest" : "Fetch.failRequest", local
        ? { requestId: p.requestId } : { requestId: p.requestId, errorReason: "BlockedByClient" }).catch(() => {});
    }
    if (message.method === "Network.requestWillBeSent" && new URL(p.request.url).pathname.startsWith("/admin")) {
      requests.push({ id: p.requestId, ...p.request });
      if (p.redirectResponse) responses.push(p.redirectResponse);
    }
    if (message.method === "Network.responseReceived") responses.push(p.response);
    if (message.method === "Runtime.exceptionThrown") browserErrors.push(p.exceptionDetails);
    if (message.method === "Runtime.consoleAPICalled" && p.type === "error") browserErrors.push(p.args);
    const request = pending.get(message.id);
    if (!request) return;
    pending.delete(message.id);
    clearTimeout(request.timer);
    if (message.error) request.reject(new Error(message.error.message));
    else request.resolve(message.result);
  });
  for (const domain of ["Page", "Runtime", "Network", "Accessibility"]) await cdp(`${domain}.enable`);
  // Never send the synthetic admin token to an external origin, even after a regression.
  await cdp("Fetch.enable", { patterns: [{ urlPattern: "*" }] });
  await cdp("Emulation.setDeviceMetricsOverride", { width: 1440, height: 1000, deviceScaleFactor: 1, mobile: false });
  await cdp("Page.navigate", { url: `${base}/admin?${query}` });
  await arrived("/admin");
  await click(".team-section summary");
  const controls = (await cdp("Accessibility.getFullAXTree")).nodes;
  for (const [role, name] of [["searchbox", "Filter members in this team by name"], ["button", "Name"]]) {
    assert.ok(controls.some((n) => !n.ignored && n.role?.value === role && n.name?.value === name), `${role}: ${name}`);
  }
  await js(`window.swaps = 0; window.htmxErrors = [];
    document.body.addEventListener('htmx:afterSettle', () => swaps++);
    for (const event of ['htmx:responseError', 'htmx:sendError', 'htmx:swapError'])
      document.body.addEventListener(event, () => htmxErrors.push(event));`);
  const filter = '.team-section input[name="filter"]';
  async function swap(action, expectedNames, dir, filterValue, inputFocused = false) {
    const before = await js("swaps");
    await js("window.oldMembers = document.querySelector('.team-members')");
    await action();
    await until(() => js(`swaps > ${before} && !oldMembers.isConnected`), "HTMX table replacement");
    assert.deepEqual(await js("[...document.querySelectorAll('.team-members .admin-cell-name')].map(el => el.textContent.trim())"), expectedNames);
    assert.deepEqual(await js(`['sort', 'dir', 'filter'].map(name => document.querySelector('.team-section input[name="' + name + '"]').value)`), ["name", dir, filterValue]);
    assert.equal(await js("document.querySelector('.team-members th').getAttribute('aria-sort')"), dir === "asc" ? "ascending" : "descending");
    if (inputFocused) assert.equal(await js(`document.activeElement === document.querySelector(${JSON.stringify(filter)})`), true);
    assert.equal(await js("location.pathname"), "/admin");
  }
  const sort = () => click(".admin-sort");
  await swap(() => typeInto(filter, "Alice"), ["Alice"], "asc", "Alice", true);
  await swap(sort, ["Alice"], "desc", "Alice");
  await swap(() => typeInto(filter, ""), names.toReversed(), "desc", "", true);
  await swap(sort, names, "asc", "");
  await swap(() => typeInto(filter, "Nobody&+?#%"), [], "asc", "Nobody&+?#%", true);
  await swap(() => typeInto(filter, ""), names, "asc", "", true);
  assert.deepEqual(await js("htmxErrors"), []);
  const htmx = requests.filter((r) => new URL(r.url).pathname === "/admin/team-members");
  assert.equal(htmx.length, 6);
  for (const request of htmx) {
    assert.equal(Object.entries(request.headers).find(([name]) => name.toLowerCase() === "hx-request")?.[1], "true");
    assert.ok(responses.some((r) => r.url === request.url && r.status === 200));
  }
  assert.ok(htmx.some((r) => new URL(r.url).searchParams.get("filter") === "Nobody&+?#%"));
  console.log("PASS: real HTMX filtering/sorting, focus, state, and special-character query encoding");

  const form = `form.admin-inline-form[action*="/${alice}/"]`;
  async function move(destination) {
    const action = await js(`document.querySelector(${JSON.stringify(form)}).action`);
    assert.equal(new URL(action).searchParams.get("token"), token);
    await typeInto(`${form} input`, destination);
    await key("Tab", 9);
    assert.equal(await js(`document.activeElement === document.querySelector(${JSON.stringify(`${form} button`)})`), true);
    await js("window.oldDocument = true");
    await key("Enter", 13);
    await arrived("/admin");
    assert.deepEqual(db.prepare("SELECT name, team_token FROM participants ORDER BY name").all().map((r) => [r.name, r.team_token]),
      names.map((name) => [name, name === "Alice" ? destination : team]));
    const post = requests.findLast((r) => r.method === "POST" && r.url === action);
    assert.ok(post, "Move must submit a real browser POST");
    assert.equal(new URLSearchParams(post.postData).get("team_token"), destination);
    const redirect = responses.findLast((r) => r.url === action && r.status === 303);
    assert.ok(redirect, "Move must return 303");
    const location = Object.entries(redirect.headers).find(([name]) => name.toLowerCase() === "location")?.[1];
    assert.equal(new URL(location, base).searchParams.get("token"), token);
  }
  // Submit from the HTMX-replaced partial; keep Bob and the long name for layout checks.
  await move("Moved-from-partial");
  await click(`.team-section:has(a[href^="/admin/team/${team}?"]) summary`);
  await follow(`a[href^="/admin/team/${team}?"]`, `/admin/team/${team}`);

  for (const width of [320, 390, 1440]) {
    await cdp("Emulation.setDeviceMetricsOverride", { width, height: 1000, deviceScaleFactor: 1, mobile: width < 600 });
    await js("document.querySelector('.team-roster-scroll').scrollLeft = 0");
    const layout = await js(`(() => {
      const region = document.querySelector('.team-roster-scroll');
      const rect = region.getBoundingClientRect();
      return {viewport: innerWidth, page: document.documentElement.scrollWidth, body: document.body.scrollWidth,
        left: rect.left, right: rect.right, tabIndex: region.tabIndex, overflow: getComputedStyle(region).overflowX,
        clippedPage: [document.documentElement, document.body].some(el => ['hidden', 'clip'].includes(getComputedStyle(el).overflowX))};
    })()`);
    assert.equal(layout.viewport, width);
    assert.ok(layout.page <= width + 1 && layout.body <= width + 1, `${width}px: page overflow: ${JSON.stringify(layout)}`);
    assert.ok(layout.left >= -1 && layout.right <= width + 1);
    assert.equal(layout.clippedPage, false, "Do not hide document overflow to make the check pass");
    assert.equal(layout.overflow, "auto");
    assert.equal(layout.tabIndex, 0);
    const ax = await cdp("Accessibility.getFullAXTree");
    const region = ax.nodes.find((n) => n.role?.value === "region" && n.name?.value === "Team roster");
    assert.ok(region?.properties.some((p) => p.name === "focusable" && p.value.value));
    for (const [role, name] of [["button", "Move"], ["link", "Open account"], ["button", "Remove"]]) {
      assert.ok(ax.nodes.some((n) => !n.ignored && n.role?.value === role && n.name?.value === name), `${role}: ${name}`);
    }
    await js("document.querySelector('.team-back a').focus()");
    for (let i = 0; i < 12; i++) {
      await key("Tab", 9);
      if (await js("document.activeElement.matches('.team-roster-scroll')")) break;
    }
    assert.equal(await js("document.activeElement.matches('.team-roster-scroll:focus-visible')"), true);
    assert.equal(await js("getComputedStyle(document.activeElement).outlineStyle !== 'none' && parseFloat(getComputedStyle(document.activeElement).outlineWidth) > 0"), true);
    await key("ArrowRight", 39);
    await until(() => js("document.activeElement.scrollWidth <= document.activeElement.clientWidth || document.activeElement.scrollLeft > 0"), "keyboard roster scroll");
    // Arrow-key scrolling animates; don't race the next Tab against that animation.
    let previous = -1, stable = 0;
    await until(async () => {
      const left = await js("document.activeElement.scrollLeft");
      stable = left === previous ? stable + 1 : 0;
      previous = left;
      return stable >= 3;
    }, "roster scroll settling");
    assert.equal(await js("scrollX"), 0);
    // Native focus scrolling may reveal only part of a control. Its center must
    // still be visible/hit-testable, and Tab must reach every control in the row.
    for (const selector of [".admin-participant-link", "input", "button", ".admin-col-actions a", "[data-remove-participant]"]) {
      await key("Tab", 9);
      assert.equal(await js(`document.activeElement === document.querySelector('.team-roster-scroll tbody tr').querySelector(${JSON.stringify(selector)})`), true);
      await until(async () => {
        const visible = await js(`(() => {
          const el = document.activeElement, r = el.getBoundingClientRect(), outer = el.closest('.team-roster-scroll').getBoundingClientRect();
          return {left: r.left, right: r.right, regionLeft: outer.left, regionRight: outer.right,
            hit: document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2) === el, scrollX};
        })()`);
        const center = (visible.left + visible.right) / 2;
        assert.ok(visible.right > visible.left && center >= visible.regionLeft
          && center <= visible.regionRight && visible.hit && visible.scrollX === 0, JSON.stringify(visible));
        return true;
      }, `${width}px: ${selector} visible and reachable`);
    }
    console.log(`PASS: ${width}px team page fits; named roster, keyboard scrolling, focus, and row controls`);
  }
  await follow(".team-back a", "/admin");
  await click('.team-section:has(a[href^="/admin/team/Moved-from-partial?"]) summary');
  await follow('a[href^="/admin/team/Moved-from-partial?"]', "/admin/team/Moved-from-partial");
  await cdp("Emulation.setDeviceMetricsOverride", { width: 320, height: 1000, deviceScaleFactor: 1, mobile: true });
  await move("Moved-from-team");
  await click('.team-section:has(a[href^="/admin/team/Moved-from-team?"]) summary');
  await follow('a[href^="/admin/team/Moved-from-team?"]', "/admin/team/Moved-from-team");
  assert.equal(await js("document.querySelector('.admin-cell-name').textContent.trim()"), "Alice");
  await follow(".team-back a", "/admin");
  for (const request of requests) assert.equal(new URL(request.url).searchParams.get("token"), token, request.url);
  console.log("PASS: Move from swapped partial and mobile team page, persisted membership, 303 redirects, and back links");

  // Use server-rendered participant navigation: the offline editor fixture has
  // only an empty topbar and cannot catch crowded breadcrumbs or missing links.
  async function chapter(path) {
    // Picker links use slugs; the server redirects them to numbered file stems.
    const response = await fetch(base + path, { signal: AbortSignal.timeout(5000) });
    assert.equal(response.status, 200);
    path = new URL(response.url).pathname;
    await response.body.cancel();
    await js("window.oldDocument = true");
    await cdp("Page.navigate", { url: base + path });
    await until(() => js(`!window.oldDocument && document.readyState === 'complete'
      && location.pathname === ${JSON.stringify(path)} && !!document.querySelector('.chapter-picker')`), `chapter navigation to ${path}`);
    await js("document.fonts.ready.then(() => true)");
  }
  await chapter(`/exercise/${alice}/00_numbers_in_rust`);
  const longChapter = await js(`[...document.querySelectorAll('.chapter-picker-item')]
    .sort((a, b) => b.querySelector('.chapter-picker-title').textContent.length - a.querySelector('.chapter-picker-title').textContent.length)[0].pathname`);
  for (const path of [`/exercise/${alice}/00_numbers_in_rust`, longChapter]) {
    await chapter(path);
    const title = await js("document.querySelector('.chapter-picker-label').textContent.trim()");
    for (const width of [320, 375, 768, 800, 1024, 1200, 1201, 1440]) {
      await cdp("Emulation.setDeviceMetricsOverride", { width, height: 450, deviceScaleFactor: 1, mobile: false });
      await js("window.scrollTo(0, 0); new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))");
      const layout = await js(`(() => {
        const header = document.querySelector('.topbar').getBoundingClientRect();
        const controls = [...document.querySelectorAll('.topbar-home, .chapter-picker-btn, .topbar-link, .topbar .theme-toggle')];
        const label = document.querySelector('.chapter-picker-label');
        const actionCenters = [...document.querySelectorAll('.topbar-link, .topbar .theme-toggle')]
          .map(el => { const r = el.getBoundingClientRect(); return r.y + r.height / 2; });
        return {viewport: innerWidth, page: document.documentElement.scrollWidth,
          actionsTogether: Math.max(...actionCenters) - Math.min(...actionCenters) <= 1,
          clippedPage: [document.documentElement, document.body].some(el => ['hidden', 'clip'].includes(getComputedStyle(el).overflowX)),
          progress: document.querySelector('.topbar-progress').getBoundingClientRect().width,
          titleFits: label.scrollWidth <= label.clientWidth + 1,
          controls: controls.map(el => {
            const r = el.getBoundingClientRect();
            return {name: el.textContent.trim() || el.getAttribute('aria-label'),
              fits: r.width > 0 && r.left >= 0 && r.right <= innerWidth + 1 && r.top >= header.top && r.bottom <= header.bottom && r.bottom <= innerHeight,
              hit: el.contains(document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2))};
          })};
      })()`);
      assert.equal(layout.viewport, width);
      assert.ok(layout.page <= width + 1 && !layout.clippedPage && layout.titleFits && layout.progress > 0,
        `${width}px ${title}: ${JSON.stringify(layout)}`);
      if (width > 768) assert.ok(layout.actionsTogether, `${width}px: keep labelled navigation actions on the same row`);
      assert.equal(layout.controls.length, 8);
      for (const control of layout.controls) assert.ok(control.fits && control.hit, `${width}px ${title}: ${JSON.stringify(control)}`);
      await js("document.querySelector('.topbar-home').focus()");
      for (let i = 1; i < layout.controls.length; i++) {
        await key("Tab", 9);
        assert.equal(await js(`document.activeElement === document.querySelectorAll('.topbar-home, .chapter-picker-btn, .topbar-link, .topbar .theme-toggle')[${i}] && scrollX === 0`), true);
      }
      if (values["screenshot-dir"]) {
        const dir = resolve(values["screenshot-dir"]);
        await mkdir(dir, { recursive: true });
        const image = await cdp("Page.captureScreenshot");
        await writeFile(join(dir, `navigation-${path.split('/').at(-1)}-${width}.png`), Buffer.from(image.data, "base64"));
      }
      await click('.chapter-picker-btn');
      assert.equal(await js("document.querySelector('.chapter-picker-btn').getAttribute('aria-expanded')"), "true");
      assert.equal(await js(`(() => {
        const r = document.querySelector('.chapter-picker-menu').getBoundingClientRect();
        return r.width > 0 && r.left >= 0 && r.right <= innerWidth + 1 && r.top >= 0 && r.bottom <= innerHeight;
      })()`), true, `${width}px: chapter menu fits`);
      await key("Escape", 27);
      assert.equal(await js("document.querySelector('.chapter-picker-menu').hidden"), true);
      await js(`document.querySelector('.exercise-section').scrollIntoView(); window.scrollBy(0, 200)`);
      await until(() => js(`(() => {
        const bar = document.querySelector('.topbar').getBoundingClientRect();
        const toolbar = document.querySelector('[data-role="editor-toolbar"]').getBoundingClientRect();
        return Math.abs(toolbar.top - bar.bottom) <= 1 && toolbar.bottom < innerHeight;
      })()`), `${width}px: editor toolbar clears wrapped navigation`);
      console.log(`PASS: ${width}px ${title}: navigation fits, keyboard reaches all controls, chapter menu opens, editor toolbar clears header`);
    }
  }
  // Chrome requests this optional icon even though the app does not serve one.
  assert.deepEqual(responses.filter((r) => r.status >= 400
    && !(r.status === 404 && new URL(r.url).pathname === "/favicon.ico")).map((r) => [r.status, r.url]), []);
  assert.deepEqual(browserErrors, []);
} catch (error) {
  console.error(error);
  if (socket?.readyState === WebSocket.OPEN) {
    console.error(await js("({url: location.href, ready: document.readyState, focused: document.activeElement?.outerHTML.slice(0, 500)})").catch(() => "Page unavailable"));
    console.error("Last admin requests:", requests.slice(-3).map(({ method, url, postData }) => ({ method, url, postData })));
  }
  console.error(logs);
  process.exitCode = 1;
} finally {
  clearTimeout(watchdog);
  await cleanup();
}
