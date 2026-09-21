import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { createResultRenderer } from "./editor-results.js";
import { highlightOutput } from "./output-highlight.js";

// These tests exercise the textarea controller without a browser. The extension
// factory is deliberately stubbed: CodeMirror is not mounted in this suite.
const source = await readFile(
  new URL("./inline-editor.js", import.meta.url),
  "utf8",
);
const extensionStub =
  "data:text/javascript," +
  encodeURIComponent(
    "export const buildLocalCompletions = () => {}; export const createUrlExtensions = () => [];",
  );
const moduleSource = source.replace(
  /from "(\.\/[^\"]+)"/g,
  (_, path) =>
    `from ${JSON.stringify(path === "./editor-extensions.js" ? extensionStub : new URL(path, import.meta.url).href)}`,
);
const { mountInlineEditor } = await import(
  "data:text/javascript;base64," + Buffer.from(moduleSource).toString("base64")
);

class Element extends EventTarget {
  constructor(document) {
    super();
    this.ownerDocument = document;
    this.style = {};
    this.dataset = {};
    this.attributes = new Map();
    this.children = [];
    this.textContent = "";
    this.value = "starter";
    this.disabled = false;
    this.classList = { toggle() {}, remove() {}, add() {} };
  }
  get textContent() {
    return this.children.length
      ? this.children.map((child) => child.textContent).join("")
      : this.text;
  }
  set textContent(value) {
    this.children = [];
    this.text = value;
  }
  setAttribute(key, value) {
    this.attributes.set(key, value);
  }
  getAttribute(key) {
    return this.attributes.get(key) ?? null;
  }
  removeAttribute(key) {
    this.attributes.delete(key);
  }
  append(child) {
    this.children.push(child);
    child.parentNode = this;
  }
  prepend(child) {
    this.children.unshift(child);
    child.parentNode = this;
  }
  remove() {
    if (!this.parentNode) return;
    this.parentNode.children = this.parentNode.children.filter(
      (child) => child !== this,
    );
    this.parentNode = null;
  }
  replaceChildren(...children) {
    this.textContent = "";
    children.forEach((child) => this.append(child));
  }
  setRangeText(insert, from, to) {
    this.value = this.value.slice(0, from) + insert + this.value.slice(to);
  }
  focus() {
    this.ownerDocument.activeElement = this;
  }
  querySelector() {
    return null;
  }
}
function fixture() {
  const document = new EventTarget();
  const window = new EventTarget();
  document.defaultView = window;
  window.navigator = { clipboard: { writeText: async () => {} } };
  window.confirm = () => true;
  document.createElement = () => new Element(document);
  document.getElementById = () => null;
  document.querySelector = () => null;
  const section = new Element(document);
  const roles = Object.fromEntries(
    [
      "editor-fallback",
      "run-btn",
      "submit-btn",
      "format-btn",
      "reset-btn",
      "copy-btn",
      "run-status",
      "action-status",
      "run-spinner",
      "output-panel",
      "test-list",
      "output-stderr",
      "output-details",
    ].map((role) => [role, new Element(document)]),
  );
  for (const element of Object.values(roles)) section.append(element);
  section.querySelector = (selector) =>
    roles[selector.match(/"([^"]+)"/)[1]] ?? null;
  return { document, window, section, roles };
}
const tick = () => new Promise((resolve) => setImmediate(resolve));

test("result rows expose textual statuses, preserve both streams, and never scroll", () => {
  const { roles } = fixture();
  const panel = roles["output-panel"];
  panel.scrollIntoView = () =>
    assert.fail("Results should not move the viewport");
  const render = createResultRenderer({
    panel,
    list: roles["test-list"],
    output: roles["output-stderr"],
    details: roles["output-details"],
    testResults: true,
  });
  render({
    success: false,
    stdout: "hello",
    stderr: "warning",
    test_results: [
      { name: "good", passed: true },
      { name: "bad", passed: false },
    ],
  });
  assert.equal(roles["output-stderr"].textContent, "hello\n\nwarning");
  assert.equal(
    roles["test-list"].children[0].children[0].textContent,
    "Passed: good",
  );
  assert.equal(
    roles["test-list"].children[1].children[0].textContent,
    "Failed: bad",
  );
  assert.equal(roles["output-details"].open, true);
});

test("output highlights Rust values and diagnostics without interpreting HTML", () => {
  const { roles } = fixture();
  const output = roles["output-stderr"];
  const text = [
    "error[E0308]: mismatched types",
    " --> src/main.rs:2:5",
    "2 | let value = Some(42);",
    "assertion `left == right` failed",
    " left: None",
    'right: Some("<script>alert(1)</script>&")',
    "test example ... FAILED",
    "test another ... ok",
    "warning: unused variable",
    "help: try `Some(2)`",
    "plain <img src=x onerror=alert(1)> output\r\n",
  ].join("\n");
  highlightOutput(output, text);
  assert.equal(output.textContent, text);
  for (const cls of ["output-error", "output-warning", "output-help", "output-success", "tok-keyword", "tok-number", "tok-string"]) {
    assert.ok(output.children.some((span) => span.className.split(" ").includes(cls)), cls);
  }
  assert.ok(output.children.every((span) => span.children.length === 0));
  highlightOutput(output, "replacement");
  assert.equal(output.textContent, "replacement");
});

test("failure snippets and full logs both receive highlighting", () => {
  const { roles } = fixture();
  const render = createResultRenderer({
    panel: roles["output-panel"],
    list: roles["test-list"],
    output: roles["output-stderr"],
    testResults: true,
  });
  render({
    success: false,
    stderr: "---- example stdout ----\nthread 'example' panicked at src/main.rs:2:5:\nassertion `left == right` failed\n left: None\nright: Some(2)\n\nfailures:\n    example",
    test_results: [{ name: "example", passed: false }],
  });
  const snippet = roles["test-list"].children[0].children[1];
  assert.match(snippet.textContent, /right: Some\(2\)/);
  assert.ok(snippet.children.some((span) => span.className === "tok-number"));
  assert.ok(roles["output-stderr"].children.some((span) => span.className === "tok-number"));
});

test("diagnostics remain visible with a stale notice until fresh results replace them", () => {
  const { roles } = fixture();
  const panel = roles["output-panel"];
  const output = roles["output-stderr"];
  const list = roles["test-list"];
  const details = roles["output-details"];
  const render = createResultRenderer({
    panel,
    output,
    list,
    details,
    testResults: true,
  });
  const notice = panel.children[0];
  assert.match(notice.textContent, /previous version.*Run again/);
  render.clear();
  render.invalidate();
  assert.equal(panel.style.display, "none");
  assert.equal(notice.hidden, true);

  render({
    success: false,
    stderr: "compiler diagnostic",
    test_results: [{ name: "failed", passed: false }],
  });
  const row = list.children[0];
  render.invalidate();
  render.invalidate();
  assert.equal(panel.style.display, "block");
  assert.equal(output.textContent, "compiler diagnostic");
  assert.equal(list.children[0], row);
  assert.equal(details.open, true);
  assert.equal(notice.hidden, false);
  assert.equal(panel.children.length, 1);

  render({
    success: true,
    stdout: "new output",
    test_results: [{ name: "passed", passed: true }],
  });
  assert.equal(notice.hidden, true);
  assert.equal(output.textContent, "new output");
  assert.notEqual(list.children[0], row);
  assert.equal(details.open, false);
  render.invalidate();
  render.clear();
  render.invalidate();
  assert.equal(panel.style.display, "none");
  assert.equal(notice.hidden, true);
  render.destroy();
  render.destroy();
  assert.equal(panel.children.length, 0);
});

const scrollPreference = "corrode:editor:scroll-output";
const smoothScroll = { behavior: "smooth", block: "start", inline: "nearest" };

function mockScrollStorage(t, storage = new Map()) {
  const original = Object.getOwnPropertyDescriptor(globalThis, "localStorage");
  Object.defineProperty(globalThis, "localStorage", {
    configurable: true,
    get: () => ({ getItem: (key) => storage.get(key) ?? null }),
  });
  t.after(() => {
    if (original) Object.defineProperty(globalThis, "localStorage", original);
    else delete globalThis.localStorage;
  });
  return storage;
}

function observeScroll(f) {
  const calls = [];
  f.roles["output-panel"].scrollIntoView = function (options) {
    calls.push({
      target: this,
      options,
      display: this.style.display,
      output: f.roles["output-stderr"].textContent,
      rows: f.roles["test-list"].textContent,
      focused: f.document.activeElement,
    });
  };
  return calls;
}

for (const [label, result, row] of [
  ["success", { success: true, stdout: "hello", test_results: [{ name: "example", passed: true }] }, "Passed: example"],
  ["compile failure", { success: false, stderr: "error[E0308]: mismatched types", test_results: [] }, ""],
  ["test failure", { success: false, stderr: "assertion failed", test_results: [{ name: "example", passed: false }] }, "Failed: example"],
]) {
  test(`run scrolls smoothly only after displaying ${label}, without moving focus`, async (t) => {
    t.mock.method(console, "warn", () => {});
    mockScrollStorage(t);
    let resolve;
    t.mock.method(globalThis, "fetch", () => new Promise((done) => { resolve = done; }));
    const f = fixture();
    const calls = observeScroll(f);
    const api = await mountInlineEditor(f.section);
    t.after(() => api.destroy());
    const input = f.roles["editor-fallback"];
    input.focus();
    f.roles["run-btn"].dispatchEvent(new Event("click"));
    assert.equal(calls.length, 0);
    assert.equal(f.roles["output-panel"].style.display, "none");
    resolve({ ok: true, json: async () => result });
    await tick();
    assert.deepEqual(calls, [{
      target: f.roles["output-panel"],
      options: smoothScroll,
      display: "block",
      output: result.stdout || result.stderr,
      rows: row,
      focused: input,
    }]);
    assert.equal(f.document.activeElement, input);
  });
}

for (const [label, preference, reducedMotion, expected] of [
  ["opted out", "0", false, null],
  ["reduced motion", null, true, { ...smoothScroll, behavior: "instant" }],
  ["no motion preference", null, false, smoothScroll],
  ["nonzero preference", "false", false, smoothScroll],
]) {
  test(`${label}: results render with the expected scroll policy`, async (t) => {
    t.mock.method(console, "warn", () => {});
    mockScrollStorage(t, new Map([[scrollPreference, preference]]));
    t.mock.method(globalThis, "fetch", async () => ({
      ok: true,
      json: async () => ({ success: true, stdout: "rendered", test_results: [] }),
    }));
    const f = fixture();
    const queries = [];
    f.window.matchMedia = (query) => {
      queries.push(query);
      return { matches: reducedMotion };
    };
    const calls = observeScroll(f);
    const api = await mountInlineEditor(f.section);
    t.after(() => api.destroy());
    f.roles["run-btn"].dispatchEvent(new Event("click"));
    await tick();
    assert.equal(f.roles["output-panel"].style.display, "block");
    assert.equal(f.roles["output-stderr"].textContent, "rendered");
    assert.deepEqual(calls.map((call) => call.options), expected ? [expected] : []);
    if (expected) assert.deepEqual(queries, ["(prefers-reduced-motion: reduce)"]);
  });
}

test("mounted editor reads scroll opt-out at render time on every run", async (t) => {
  t.mock.method(console, "warn", () => {});
  const storage = mockScrollStorage(t);
  let resolve;
  t.mock.method(globalThis, "fetch", () => new Promise((done) => { resolve = done; }));
  const f = fixture();
  const calls = observeScroll(f);
  const api = await mountInlineEditor(f.section);
  t.after(() => api.destroy());
  for (const [preference, expectedCount] of [["0", 0], [null, 1], ["0", 1]]) {
    f.roles["run-btn"].dispatchEvent(new Event("click"));
    storage.set(scrollPreference, preference);
    resolve({ ok: true, json: async () => ({ success: true, stdout: `run ${preference}`, test_results: [] }) });
    await tick();
    assert.equal(f.roles["output-stderr"].textContent, `run ${preference}`);
    assert.equal(calls.length, expectedCount);
  }
});

for (const blocked of ["access", "getItem"]) {
  test(`blocked storage ${blocked} is nonfatal and defaults to smooth scrolling`, async (t) => {
    t.mock.method(console, "warn", () => {});
    mockScrollStorage(t);
    const denied = () => { throw new Error("Storage blocked"); };
    Object.defineProperty(globalThis, "localStorage", {
      configurable: true,
      get: blocked === "access" ? denied : () => ({ getItem: denied }),
    });
    t.mock.method(globalThis, "fetch", async () => ({
      ok: true,
      json: async () => ({ success: true, stdout: "rendered", test_results: [] }),
    }));
    const f = fixture();
    const calls = observeScroll(f);
    const api = await mountInlineEditor(f.section);
    t.after(() => api.destroy());
    f.roles["run-btn"].dispatchEvent(new Event("click"));
    await tick();
    assert.deepEqual(calls.map((call) => call.options), [smoothScroll]);
    assert.equal(calls[0].output, "rendered");
    assert.equal(f.roles["run-status"].textContent, "Compiled. No tests ran.");
    assert.equal(f.roles["run-btn"].disabled, false);
  });
}

test("Ctrl+Enter scrolls only its editor's output and preserves textarea focus", async (t) => {
  t.mock.method(console, "warn", () => {});
  mockScrollStorage(t);
  const requests = [];
  t.mock.method(globalThis, "fetch", async (url) => {
    requests.push(url);
    return { ok: true, json: async () => ({ success: true, stdout: "keyboard run", test_results: [] }) };
  });
  const first = fixture();
  const second = fixture();
  const firstCalls = observeScroll(first);
  const secondCalls = observeScroll(second);
  for (const f of [first, second]) {
    const api = await mountInlineEditor(f.section);
    t.after(() => api.destroy());
  }
  const input = second.roles["editor-fallback"];
  input.focus();
  const event = new Event("keydown", { cancelable: true });
  Object.assign(event, { key: "Enter", ctrlKey: true });
  input.dispatchEvent(event);
  assert.equal(event.defaultPrevented, true);
  await tick();
  assert.deepEqual(requests, ["/api/run"]);
  assert.equal(firstCalls.length, 0);
  assert.equal(first.roles["output-panel"].style.display, "none");
  assert.equal(secondCalls.length, 1);
  assert.equal(secondCalls[0].target, second.roles["output-panel"]);
  assert.deepEqual(secondCalls[0].options, smoothScroll);
  assert.equal(secondCalls[0].output, "keyboard run");
  assert.equal(secondCalls[0].focused, input);
  assert.equal(second.document.activeElement, input);
});

test("format and reset do not scroll output", async (t) => {
  t.mock.method(console, "warn", () => {});
  mockScrollStorage(t);
  const requests = [];
  t.mock.method(globalThis, "fetch", async (url) => {
    requests.push(url);
    return { ok: true, json: async () => url === "/api/run"
      ? { success: true, stdout: "previous output", test_results: [] }
      : { success: true, code: "formatted" } };
  });
  const f = fixture();
  const calls = observeScroll(f);
  const api = await mountInlineEditor(f.section);
  t.after(() => api.destroy());
  f.roles["run-btn"].dispatchEvent(new Event("click"));
  await tick();
  assert.equal(calls.length, 1);
  f.roles["format-btn"].dispatchEvent(new Event("click"));
  await tick();
  assert.equal(api.getValue(), "formatted");
  assert.equal(f.roles["output-stderr"].textContent, "previous output");
  assert.equal(calls.length, 1);
  f.roles["reset-btn"].dispatchEvent(new Event("click"));
  await tick();
  assert.equal(api.getValue(), "starter");
  assert.equal(f.roles["output-panel"].style.display, "none");
  assert.equal(calls.length, 1);
  assert.deepEqual(requests, ["/api/run", "/api/format"]);
});

for (const invalidation of ["edit", "reset", "destroy"]) {
  test(`${invalidation} prevents late results from rendering or scrolling`, async (t) => {
    t.mock.method(console, "warn", () => {});
    mockScrollStorage(t);
    let resolve;
    let signal;
    t.mock.method(globalThis, "fetch", (_, options) => {
      signal = options.signal;
      return new Promise((done) => { resolve = done; });
    });
    const f = fixture();
    const calls = observeScroll(f);
    const api = await mountInlineEditor(f.section);
    t.after(() => api.destroy());
    api.setValue("edited before run");
    f.roles["run-btn"].dispatchEvent(new Event("click"));
    if (invalidation === "destroy") api.destroy();
    else if (invalidation === "reset") f.roles["reset-btn"].dispatchEvent(new Event("click"));
    else {
      f.roles["editor-fallback"].value = "new revision";
      f.roles["editor-fallback"].dispatchEvent(new Event("input"));
    }
    assert.equal(signal.aborted, true);
    resolve({ ok: true, json: async () => ({ success: true, stdout: "late output", test_results: [] }) });
    await tick();
    assert.equal(calls.length, 0);
    assert.equal(f.roles["output-panel"].style.display, "none");
    assert.equal(f.roles["output-stderr"].textContent, "");
  });
}

test("fallback edits retain stale diagnostics; remount starts clear and removes old notices", async (t) => {
  t.mock.method(console, "warn", () => {});
  t.mock.method(globalThis, "fetch", async () => ({
    ok: true,
    json: async () => ({
      success: false,
      stderr: "diagnostic",
      test_results: [],
    }),
  }));
  const f = fixture();
  const panel = f.roles["output-panel"];
  const api = await mountInlineEditor(f.section);
  assert.equal(panel.style.display, "none");
  f.roles["run-btn"].dispatchEvent(new Event("click"));
  await tick();
  f.roles["editor-fallback"].value = "edited";
  f.roles["editor-fallback"].dispatchEvent(new Event("input"));
  assert.equal(panel.style.display, "block");
  assert.equal(f.roles["output-stderr"].textContent, "diagnostic");
  assert.equal(panel.children[0].hidden, false);
  api.destroy();
  assert.equal(panel.children.length, 0);
  const next = await mountInlineEditor(f.section);
  t.after(() => next.destroy());
  assert.equal(panel.style.display, "none");
  assert.equal(panel.children.length, 1);
  assert.equal(panel.children[0].hidden, true);
});

test("fallback mount is idempotent, invalidates on input, flushes and removes listeners on destroy", async (t) => {
  const saved = new Map([["draft", "saved"]]);
  t.mock.method(console, "warn", () => {});
  const originalStorage = Object.getOwnPropertyDescriptor(
    globalThis,
    "localStorage",
  );
  Object.defineProperty(globalThis, "localStorage", {
    configurable: true,
    value: {
      getItem: (key) => saved.get(key) ?? null,
      setItem: (key, value) => saved.set(key, value),
    },
  });
  t.after(() => {
    if (originalStorage)
      Object.defineProperty(globalThis, "localStorage", originalStorage);
    else delete globalThis.localStorage;
  });
  const f = fixture();
  const options = {
    submitted: "saved",
    submittedPassed: true,
    features: { draftKey: "draft", submit: { ulid: "id" } },
  };
  const first = mountInlineEditor(f.section, options);
  assert.equal(mountInlineEditor(f.section, options), first);
  const api = await first;
  assert.equal(f.roles["run-btn"].style.display, "");
  assert.equal(f.roles["submit-btn"].disabled, true);
  assert.match(
    f.roles["editor-fallback"].getAttribute("aria-label"),
    /Rust code editor/,
  );
  f.roles["editor-fallback"].value = "";
  f.roles["editor-fallback"].dispatchEvent(new Event("input"));
  assert.equal(f.roles["submit-btn"].style.display, "none");
  assert.equal(f.roles["output-panel"].style.display, "none");
  f.window.dispatchEvent(new Event("pagehide"));
  assert.equal(saved.get("draft"), "");
  let requests = 0;
  t.mock.method(globalThis, "fetch", async () => {
    requests++;
    return {
      ok: true,
      json: async () => ({ success: true, test_results: [] }),
    };
  });
  f.roles["run-btn"].dispatchEvent(new Event("click"));
  await tick();
  assert.equal(requests, 1);
  api.destroy();
  api.destroy();
  f.roles["run-btn"].dispatchEvent(new Event("click"));
  assert.equal(requests, 1);
  const remounted = await mountInlineEditor(f.section, options);
  assert.notEqual(remounted, api);
  assert.equal(remounted.getValue(), "");
  remounted.destroy();
});

for (const [kind, status, expected] of [
  ["submit", 401, /session is unknown or expired.*registering again/],
  ["run", 429, /Run rate-limited/],
  ["format", 502, /Formatter unreachable/],
  ["submit", 502, /Progress service unreachable/],
  ["run", 500, /Run failed \(HTTP 500\)/],
  ["run", null, /Run failed: offline.*Check your connection/],
]) {
  test(`${kind} reports actionable network error ${status ?? "offline"}`, async (t) => {
    t.mock.method(console, "warn", () => {});
    t.mock.method(globalThis, "fetch", async (url) => {
      if (kind === "submit" && url === "/api/run") {
        return {
          ok: true,
          json: async () => ({
            success: true,
            test_results: [{ passed: true }],
          }),
        };
      }
      if (status === null) throw new Error("offline");
      return { ok: false, status };
    });
    const f = fixture();
    const api = await mountInlineEditor(f.section, {
      features: { submit: { ulid: "id" } },
    });
    t.after(() => api.destroy());
    f.roles[kind === "format" ? "format-btn" : "run-btn"].dispatchEvent(
      new Event("click"),
    );
    await tick();
    assert.match(f.roles["run-status"].textContent, expected);
    assert.equal(f.roles["run-btn"].disabled, false);
    if (kind === "submit") {
      assert.equal(f.roles["submit-btn"].disabled, false);
      assert.equal(f.roles["submit-btn"].style.display, "");
    }
  });
}

test("scratchpad passes main-execution mode to request and status", async (t) => {
  t.mock.method(console, "warn", () => {});
  let payload;
  t.mock.method(globalThis, "fetch", async (_, options) => {
    payload = JSON.parse(options.body);
    return {
      ok: true,
      json: async () => ({ success: true, test_results: [] }),
    };
  });
  const f = fixture();
  const api = await mountInlineEditor(f.section, {
    features: { runWithoutTests: true },
  });
  t.after(() => api.destroy());
  f.roles["run-btn"].dispatchEvent(new Event("click"));
  await tick();
  assert.equal(payload.tests, false);
  assert.equal(f.roles["run-status"].textContent, "Ran successfully.");
});

test("destroy and remount isolate late request responses and callbacks", async (t) => {
  t.mock.method(console, "warn", () => {});
  const pending = [];
  t.mock.method(
    globalThis,
    "fetch",
    (_, options) =>
      new Promise((resolve) =>
        pending.push({ resolve, signal: options.signal }),
      ),
  );
  let callbacks = 0;
  const f = fixture();
  mockScrollStorage(t);
  const calls = observeScroll(f);
  const opts = {
    onRunSuccess: () => {
      callbacks++;
    },
  };
  const old = await mountInlineEditor(f.section, opts);
  f.roles["run-btn"].dispatchEvent(new Event("click"));
  old.destroy();
  assert.equal(pending[0].signal.aborted, true);
  const next = await mountInlineEditor(f.section, opts);
  t.after(() => next.destroy());
  f.roles["run-btn"].dispatchEvent(new Event("click"));
  pending[0].resolve({
    ok: true,
    json: async () => ({ success: true, test_results: [] }),
  });
  await tick();
  assert.equal(callbacks, 0);
  assert.equal(calls.length, 0);
  assert.equal(f.roles["output-panel"].style.display, "none");
  assert.equal(f.roles["run-btn"].disabled, true);
  assert.match(f.roles["run-status"].textContent, /Running/);
  pending[1].resolve({
    ok: true,
    json: async () => ({ success: true, test_results: [] }),
  });
  await tick();
  assert.equal(callbacks, 1);
  assert.equal(calls.length, 1);
  assert.deepEqual(calls[0].options, smoothScroll);
  assert.equal(f.roles["run-btn"].disabled, false);
});

test("clipboard failures have text feedback; destroy prevents late updates", async (t) => {
  t.mock.method(console, "warn", () => {});
  const f = fixture();
  let reject;
  f.window.navigator.clipboard.writeText = () =>
    new Promise((_, fail) => {
      reject = fail;
    });
  const api = await mountInlineEditor(f.section);
  f.roles["copy-btn"].dispatchEvent(new Event("click"));
  reject(new Error("denied"));
  await tick();
  assert.ok(
    f.section.children.some((child) =>
      child.textContent.includes("Could not copy"),
    ),
  );
  f.roles["copy-btn"].dispatchEvent(new Event("click"));
  api.destroy();
  reject(new Error("denied"));
  await tick();
  assert.equal(f.roles["copy-btn"].disabled, false);
});
