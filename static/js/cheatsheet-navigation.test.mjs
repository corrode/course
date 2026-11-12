import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { runInNewContext } from "node:vm";

const template = await readFile(
  new URL("../../templates/base.html", import.meta.url),
  "utf8",
);
const script = template.match(
  /\/\/ ---- Cheatsheet modal ----([\s\S]*?)\/\/ ----- Admin/,
)?.[1];
assert.ok(script, "cheatsheet modal script exists");

for (const href of [
  "https://course.example/cheatsheet",
  "https://course.example/cheatsheet/participant-id",
  'https://course.example/cheatsheet/id" onmouseover="alert(1)',
]) {
  test(`failed modal preserves the full-page link safely: ${href}`, async () => {
    let message;
    const body = {
      replaceChildren(node) { message = node; },
      set innerHTML(_) { assert.fail("fallback must not interpolate HTML"); },
    };
    const attributes = new Map();
    const modal = {
      querySelector: (selector) => selector === "[data-cheatsheet-body]" ? body : null,
      querySelectorAll: () => [],
      addEventListener() {},
      setAttribute: (key, value) => attributes.set(key, value),
      getAttribute: (key) => attributes.get(key),
    };
    const toggle = new EventTarget();
    const document = {
      body: { style: {} },
      getElementById: () => modal,
      querySelectorAll: () => [toggle],
      querySelector(selector) {
        assert.equal(selector, "[data-cheatsheet-page]");
        return { href };
      },
      addEventListener() {},
      createElement(tagName) {
        return {
          tagName,
          children: [],
          append(...children) { this.children.push(...children); },
          set innerHTML(_) { assert.fail("fallback must use text and DOM nodes"); },
        };
      },
    };
    runInNewContext(script, {
      document,
      fetch: async (url) => {
        assert.equal(url, "/cheatsheet/fragment");
        throw new Error('<img src=x onerror="alert(1)">');
      },
    });
    toggle.dispatchEvent(new Event("click", { cancelable: true }));
    await new Promise((resolve) => setImmediate(resolve));
    assert.equal(message.tagName, "p");
    assert.match(message.textContent, /Could not load cheatsheet/);
    const [link, punctuation] = message.children;
    assert.equal(link.tagName, "a");
    assert.equal(link.href, href);
    assert.equal(link.textContent, "full page");
    assert.equal(link.onmouseover, undefined);
    assert.equal(punctuation, ".");
  });
}
