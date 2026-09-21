import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { runInNewContext } from "node:vm";

const template = await readFile(
  new URL("../../templates/settings.html", import.meta.url),
  "utf8",
);
const script = template.match(/<script>([\s\S]*?)<\/script>/)?.[1];
assert.ok(script, "settings inline script exists");
const storageKey = "corrode:editor:scroll-output";

function fixture({ stored, blocked = false } = {}) {
  const values = new Map(stored === undefined ? [] : [[storageKey, stored]]);
  const label = { textContent: "" };
  const input = new EventTarget();
  input.checked = false;
  input.closest = (selector) => {
    assert.equal(selector, ".settings-switch");
    return {
      querySelector(selector) {
        assert.equal(selector, ".settings-switch-label");
        return label;
      },
    };
  };
  const document = new EventTarget();
  document.getElementById = (id) =>
    id === "settings-scroll-output-toggle" ? input : null;
  document.querySelector = () => null;
  document.documentElement = {
    dataset: new Proxy({}, {
      set() {
        assert.fail("scroll preference must not set HTML attributes");
      },
      deleteProperty() {
        assert.fail("scroll preference must not delete HTML attributes");
      },
    }),
  };
  const localStorage = {
    getItem(key) {
      if (blocked) throw new Error("Storage blocked");
      return values.get(key) ?? null;
    },
    setItem(key, value) {
      if (blocked) throw new Error("Storage blocked");
      values.set(key, value);
    },
  };
  runInNewContext(script, { document, localStorage });
  return {
    input,
    label,
    values,
    toggle(on) {
      input.checked = on;
      input.dispatchEvent(new Event("change"));
    },
  };
}

test("scroll to output has a labeled checkbox and defaults to on", () => {
  assert.match(template, /<h3>Scroll to Output<\/h3>/);
  assert.match(template, /for="settings-scroll-output-toggle"/);
  assert.match(
    template,
    /<input\s+type="checkbox"\s+id="settings-scroll-output-toggle"\s+data-setting="scroll-output"\s+checked\s*\/>/,
  );
  const h = fixture();
  assert.equal(h.input.checked, true);
  assert.equal(h.label.textContent, "On");
  assert.equal(h.values.has(storageKey), false);
});

test("stored 0 disables scroll to output", () => {
  const h = fixture({ stored: "0" });
  assert.equal(h.input.checked, false);
  assert.equal(h.label.textContent, "Off");
});

test("toggling scroll to output persists both states and updates its label", () => {
  const h = fixture();
  h.toggle(false);
  assert.equal(h.values.get(storageKey), "0");
  assert.equal(h.input.checked, false);
  assert.equal(h.label.textContent, "Off");
  h.toggle(true);
  assert.equal(h.values.get(storageKey), "1");
  assert.equal(h.input.checked, true);
  assert.equal(h.label.textContent, "On");
  assert.equal(fixture({ stored: h.values.get(storageKey) }).input.checked, true);
});

test("blocked storage is nonfatal and the toggle still updates", () => {
  const h = fixture({ blocked: true });
  assert.equal(h.input.checked, true);
  assert.equal(h.label.textContent, "On");
  h.toggle(false);
  assert.equal(h.input.checked, false);
  assert.equal(h.label.textContent, "Off");
  h.toggle(true);
  assert.equal(h.input.checked, true);
  assert.equal(h.label.textContent, "On");
  assert.equal(h.values.size, 0);
});
