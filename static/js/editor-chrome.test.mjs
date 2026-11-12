import test from "node:test";
import assert from "node:assert/strict";
import { createEditorChrome } from "./editor-chrome.js";

function fixture({ toolbar = true, observer = true, navigation = true } = {}) {
  const properties = new Map();
  const observed = [];
  let resize;
  let disconnected = false;
  const topbar = { height: 48, getBoundingClientRect() { return this; } };
  const header = { height: 80, getBoundingClientRect() { return this; } };
  const section = {
    fullscreen: false,
    querySelector: () => toolbar ? header : null,
    classList: { contains: () => section.fullscreen },
    style: {
      setProperty: (key, value) => properties.set(key, value),
      removeProperty: (key) => properties.delete(key),
    },
    ownerDocument: {
      querySelector: () => navigation ? topbar : null,
      defaultView: {
        ResizeObserver: observer ? class {
          constructor(callback) { resize = callback; }
          observe(element) { observed.push(element); }
          disconnect() { disconnected = true; }
        } : undefined,
      },
    },
  };
  const chrome = createEditorChrome(section);
  return { chrome, section, header, topbar, properties, observed,
    resize: () => resize(), disconnected: () => disconnected };
}

test("chrome measures actual toolbar/topbar heights and follows wrapping and fullscreen", () => {
  const f = fixture();
  assert.deepEqual(f.observed, [f.header, f.topbar]);
  assert.equal(f.properties.get("--editor-topbar-height"), "48px");
  assert.equal(f.properties.get("--editor-toolbar-height"), "80px");
  f.header.height = 124.5;
  f.resize();
  assert.equal(f.properties.get("--editor-toolbar-height"), "124.5px");
  f.topbar.height = 0;
  f.resize();
  assert.equal(f.properties.get("--editor-topbar-height"), "0px");
  f.chrome.destroy();
  assert.ok(f.disconnected());
  assert.equal(f.properties.size, 0);
  f.resize();
  assert.equal(f.properties.size, 0, "queued observer callbacks cannot revive a destroyed mount");
});

test("cursor margins reserve full viewport clearance regardless of scroller offset", () => {
  const f = fixture();
  const view = (top) => ({ scrollDOM: { getBoundingClientRect: () => ({ top }) } });
  assert.deepEqual(f.chrome.scrollMargins(view(-500)), { top: 128 });
  assert.deepEqual(f.chrome.scrollMargins(view(80)), { top: 128 });
  assert.deepEqual(f.chrome.scrollMargins(view(200)), { top: 128 });
  f.section.fullscreen = true;
  assert.equal(f.chrome.scrollMargins(view(80)), null);
});

test("homepage chrome needs no navigation bar", () => {
  const f = fixture({ navigation: false });
  assert.deepEqual(f.observed, [f.header]);
  assert.equal(f.properties.get("--editor-topbar-height"), "0px");
  assert.deepEqual(f.chrome.scrollMargins({ scrollDOM: { getBoundingClientRect: () => ({ top: -100 }) } }), { top: 80 });
  f.chrome.destroy();
});

test("measurement works without ResizeObserver; missing toolbars need no setup", () => {
  const f = fixture({ observer: false });
  f.header.height = 100;
  f.chrome.measure();
  assert.equal(f.properties.get("--editor-toolbar-height"), "100px");
  f.chrome.destroy();
  const missing = fixture({ toolbar: false });
  assert.deepEqual(missing.observed, []);
  assert.equal(missing.chrome.scrollMargins({}), null);
  missing.chrome.destroy();
});
