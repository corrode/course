import test from "node:test";
import assert from "node:assert/strict";
import { renameTeam } from "./admin-teams.js";

function harness() {
  const input = { value: "Ecosmic", readOnly: false };
  const button = { textContent: "Rename team", disabled: false };
  const error = { textContent: "Old error", hidden: false };
  const attributes = new Map();
  const form = {
    action: "https://example.test/admin/team/Ecosia/rename?token=a%26b",
    dataset: {},
    elements: { namedItem: () => input },
    querySelector: (selector) => selector.startsWith("button") ? button : error,
    setAttribute: (key, value) => attributes.set(key, value),
    removeAttribute: (key) => attributes.delete(key),
  };
  const navigations = [];
  return { form, input, button, error, attributes, navigations,
    navigate: (url) => navigations.push(url) };
}

function assertReady(h) {
  assert.equal(h.input.readOnly, false);
  assert.equal(h.button.disabled, false);
  assert.equal(h.button.textContent, "Rename team");
  assert.equal(h.form.dataset.saving, undefined);
  assert.equal(h.attributes.has("aria-busy"), false);
}

test("posts the team name and navigates to the server-confirmed new team", async () => {
  const h = harness();
  const destination = "https://example.test/admin/team/Ecosmic?token=a%26b";
  await renameTeam(h.form, {
    navigate: h.navigate,
    request: async (url, options) => {
      assert.equal(url, h.form.action);
      assert.equal(options.method, "POST");
      assert.equal(options.credentials, "same-origin");
      assert.equal(options.body.toString(), "team_token=Ecosmic");
      assert.equal(h.input.readOnly, true);
      assert.equal(h.button.disabled, true);
      assert.equal(h.error.hidden, true);
      assert.equal(h.error.textContent, "");
      assert.equal(h.attributes.get("aria-busy"), "true");
      return { ok: true, redirected: true, url: destination };
    },
  });
  assert.deepEqual(h.navigations, [destination]);
  assertReady(h);
});

test("displays a collision inline without losing the proposed name", async () => {
  const h = harness();
  await renameTeam(h.form, {
    navigate: h.navigate,
    request: async () => ({ ok: false, text: async () => "Destination team already exists" }),
  });
  assert.equal(h.error.hidden, false);
  assert.equal(h.error.textContent, "Destination team already exists");
  assert.equal(h.input.value, "Ecosmic");
  assert.deepEqual(h.navigations, []);
  assertReady(h);
});

test("reports network failure and allows retry after checking the team", async () => {
  const h = harness();
  await renameTeam(h.form, {
    navigate: h.navigate,
    request: async () => { throw new Error("offline"); },
  });
  assert.equal(h.error.hidden, false);
  assert.match(h.error.textContent, /check the team name before retrying/);
  assert.deepEqual(h.navigations, []);
  assertReady(h);
});

test("does not mistake a non-redirect response for a confirmed rename", async () => {
  const h = harness();
  await renameTeam(h.form, {
    navigate: h.navigate,
    request: async () => ({ ok: true, redirected: false }),
  });
  assert.equal(h.error.hidden, false);
  assert.match(h.error.textContent, /did not confirm/);
  assert.deepEqual(h.navigations, []);
  assertReady(h);
});

test("ignores duplicate submits while a rename is pending", async () => {
  const h = harness();
  let finish;
  let calls = 0;
  const request = () => {
    calls++;
    return new Promise((resolve) => { finish = resolve; });
  };
  const pending = renameTeam(h.form, { request, navigate: h.navigate });
  await renameTeam(h.form, { request, navigate: h.navigate });
  assert.equal(calls, 1);
  finish({ ok: false, text: async () => "Team not found" });
  await pending;
  assertReady(h);
});
