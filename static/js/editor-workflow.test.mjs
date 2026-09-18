import test from "node:test";
import assert from "node:assert/strict";
import { EditorState, Transaction } from "@codemirror/state";
import { history, isolateHistory, undo } from "@codemirror/commands";
import {
  createWorkflow,
  initialDocument,
  formatChanges,
  minimalChange,
  submissionPassed,
} from "./editor-workflow.js";
import { cargoOutput } from "./editor-results.js";
import { createController } from "./editor-controller.js";

const green = {
  success: true,
  test_results: [{ name: "works", passed: true }],
};
function harness(options = {}) {
  let text = options.text ?? "old";
  let state;
  const pending = [];
  const renders = [];
  const submissions = [];
  const successes = [];
  const workflow = createWorkflow({
    getValue: () => text,
    setValue: (value) => {
      text = value;
      workflow.edit();
    },
    request: (kind, payload, signal) =>
      new Promise((resolve, reject) =>
        pending.push({ kind, payload, signal, resolve, reject }),
      ),
    changed: (value) => {
      state = value;
    },
    render: (data) => renders.push(data),
    submitPayload: (code) => ({ ulid: "participant", source_code: code }),
    autoSubmit: true,
    onSubmit: (data) => submissions.push(data),
    onRunSuccess: (data) => successes.push(data),
    ...options,
  });
  return {
    workflow,
    pending,
    renders,
    submissions,
    successes,
    get text() {
      return text;
    },
    get state() {
      return state;
    },
    edit(value) {
      text = value;
      workflow.edit();
    },
  };
}
const tick = () => new Promise((resolve) => setImmediate(resolve));

test("empty drafts and empty submissions are valid initial documents", () => {
  assert.equal(initialDocument("", "submitted", "starter"), "");
  assert.equal(initialDocument(null, "", "starter"), "");
  assert.equal(initialDocument(null, null, "starter"), "starter");
});

test("saved state compares actual initial source with submitted source", () => {
  assert.equal(
    harness({ text: "draft", submitted: "saved", submittedPassed: true }).state
      .saved,
    false,
  );
  assert.equal(
    harness({ text: "saved", submitted: "saved", submittedPassed: true }).state
      .saved,
    true,
  );
  assert.equal(
    harness({ text: "", submitted: "", submittedPassed: true }).state.saved,
    true,
  );
  assert.equal(
    harness({ text: "saved", submitted: "saved" }).state.saved,
    false,
  );
});

test("a previously failed stored solution can rerun and save without editing", async () => {
  const h = harness({
    text: "saved",
    submitted: "saved",
    submittedPassed: false,
  });
  assert.equal(h.state.saved, false);
  const job = h.workflow.run();
  assert.equal(h.pending[0].payload.code, "saved");
  h.pending[0].resolve(green);
  await tick();
  assert.equal(h.pending[1].kind, "submit");
  assert.equal(h.pending[1].payload.source_code, "saved");
  h.pending[1].resolve({});
  await job;
  assert.equal(h.state.saved, true);
  assert.equal(h.text, "saved");
});

test("submit requires success and nonempty, all-passing tests", () => {
  for (const data of [
    null,
    {},
    { success: true },
    { success: true, test_results: [] },
    { ...green, success: false },
    { success: true, test_results: [{ passed: false }] },
  ])
    assert.equal(submissionPassed(data), false);
  assert.equal(submissionPassed(green), true);
});

for (const kind of ["format", "run"]) {
  test(`stale ${kind} response cannot overwrite edits or show results (even after edit/undo)`, async () => {
    const h = harness();
    const job = h.workflow[kind]();
    h.edit("new");
    h.edit("old");
    assert.equal(h.pending[0].signal.aborted, true);
    h.pending[0].resolve(
      kind === "format" ? { success: true, code: "formatted" } : green,
    );
    await job;
    assert.equal(h.text, "old");
    assert.deepEqual(h.renders, []);
    assert.deepEqual(h.successes, []);
    assert.equal(h.pending.length, 1);
    assert.equal(h.state.message, "");
    assert.equal(h.state.busy, null);
  });
}

test("editing during submit cannot mark the new document saved or trigger completion callbacks", async () => {
  const h = harness();
  const job = h.workflow.run();
  h.pending[0].resolve(green);
  await tick();
  assert.equal(h.pending[1].kind, "submit");
  assert.equal(h.pending[1].payload.source_code, "old");
  h.edit("new");
  h.pending[1].resolve({ progress_done: 1 });
  await job;
  assert.equal(h.state.saved, false);
  assert.equal(h.state.canSubmit, false);
  assert.deepEqual(h.submissions, []);
  assert.equal(h.state.message, "");
});

test("failed autosave retries the same passing snapshot without another run; manual success stays disabled", async () => {
  const h = harness();
  const run = h.workflow.run();
  h.pending[0].resolve(green);
  await tick();
  h.pending[1].reject(new Error("offline"));
  await run;
  assert.equal(h.state.canSubmit, true);
  const retry = h.workflow.submit();
  assert.equal(h.pending[2].kind, "submit");
  h.pending[2].resolve({});
  await retry;
  assert.equal(h.state.saved, true);
  assert.equal(h.state.busy, null);
  await h.workflow.submit();
  assert.equal(h.pending.length, 3);
});

test("manual submit uses an existing passing run", async () => {
  const h = harness({ autoSubmit: false });
  const run = h.workflow.run();
  h.pending[0].resolve(green);
  await run;
  const submit = h.workflow.submit();
  assert.equal(h.pending[1].kind, "submit");
  h.pending[1].resolve({});
  await submit;
  assert.equal(h.state.saved, true);
});

test("failed server success flag prevents autosave and success callbacks", async () => {
  const h = harness();
  const job = h.workflow.run();
  h.pending[0].resolve({ ...green, success: false });
  await job;
  assert.equal(h.pending.length, 1);
  assert.equal(h.state.canSubmit, false);
  assert.equal(h.state.tone, "fail");
  assert.deepEqual(h.successes, []);
});

test("format accepts empty output and reports status after its own edit", async () => {
  const h = harness();
  const job = h.workflow.format();
  h.pending[0].resolve({ success: true, code: "" });
  await job;
  assert.equal(h.text, "");
  assert.equal(h.state.message, "Formatted.");
});

for (const cancelledKind of ["run", "format", "submit"]) {
  for (const nextKind of ["run", "format"]) {
    for (const outcome of ["resolve", "reject"]) {
      test(`${cancelledKind} releases ownership on edit; late ${outcome} cannot interrupt ${nextKind}`, async () => {
        const h = harness({ autoSubmit: false });
        const cancelled = h.workflow[cancelledKind]();
        if (cancelledKind === "submit") {
          h.pending[0].resolve(green);
          await tick();
        }
        const oldRequest = h.pending.at(-1);
        const renders = h.renders.length;
        const successes = h.successes.length;
        h.edit("new source");
        assert.equal(oldRequest.signal.aborted, true);
        assert.equal(h.state.busy, null);

        const next = h.workflow[nextKind]();
        const newRequest = h.pending.at(-1);
        assert.notEqual(newRequest, oldRequest);
        assert.equal(newRequest.payload.code, "new source");
        assert.equal(h.state.busy, nextKind);
        const nextState = h.state;

        if (outcome === "reject")
          oldRequest.reject(new Error("late network failure"));
        else
          oldRequest.resolve(
            cancelledKind === "format"
              ? { success: true, code: "stale formatting" }
              : green,
          );
        await cancelled;
        assert.equal(h.state, nextState);
        assert.equal(h.text, "new source");
        assert.equal(h.renders.length, renders);
        assert.equal(h.successes.length, successes);
        assert.deepEqual(h.submissions, []);
        assert.equal(newRequest.signal.aborted, false);

        newRequest.resolve(
          nextKind === "format" ? { success: true, code: "new source" } : green,
        );
        await next;
        assert.equal(h.state.busy, null);
      });
    }
  }
}

for (const runWithoutTests of [true, false]) {
  test(`zero-test run status respects runWithoutTests=${runWithoutTests}`, async () => {
    const h = harness({ runWithoutTests });
    const job = h.workflow.run();
    h.pending[0].resolve({ success: true, test_results: [] });
    await job;
    assert.equal(
      h.state.message,
      runWithoutTests ? "Ran successfully." : "Compiled. No tests ran.",
    );
    assert.equal(h.state.canSubmit, false);
    assert.equal(h.pending.length, 1);
    assert.equal(h.successes.length, 1);
  });
}

for (const action of ["edit", "destroy"]) {
  test(`onRunSuccess may ${action} without triggering autosubmit`, async () => {
    const h = harness({
      onRunSuccess: () =>
        action === "edit" ? h.edit("new") : h.workflow.destroy(),
    });
    const job = h.workflow.run();
    h.pending[0].resolve(green);
    await job;
    assert.equal(h.pending.length, 1);
    assert.deepEqual(h.submissions, []);
  });
}

test("destroy during submit suppresses late completion and all callbacks", async () => {
  const h = harness();
  const job = h.workflow.run();
  h.pending[0].resolve(green);
  await tick();
  h.workflow.destroy();
  const state = h.state;
  assert.equal(h.pending[1].signal.aborted, true);
  h.pending[1].resolve({ progress_done: 1 });
  await job;
  assert.equal(h.state, state);
  assert.deepEqual(h.submissions, []);
});

test("callback exceptions do not strand operation ownership or undo a successful save", async (t) => {
  const errors = [];
  t.mock.method(console, "error", (...args) => errors.push(args));
  const h = harness({
    onRunSuccess: () => {
      throw new Error("run callback");
    },
    onSubmit: () => {
      throw new Error("submit callback");
    },
  });
  const job = h.workflow.run();
  h.pending[0].resolve(green);
  await tick();
  h.pending[1].resolve({});
  await job;
  assert.equal(h.state.busy, null);
  assert.equal(h.state.saved, true);
  assert.equal(errors.length, 2);
});

test("operations serialize status ownership", async () => {
  const h = harness();
  const job = h.workflow.format();
  await h.workflow.run();
  await h.workflow.submit();
  assert.equal(h.pending.length, 1);
  assert.equal(h.state.busy, "format");
  h.pending[0].resolve({ success: true, code: "old" });
  await job;
  assert.equal(h.state.message, "Already formatted.");
});

test("destroy aborts requests and blocks all late callbacks", async () => {
  const h = harness();
  const job = h.workflow.run();
  const state = h.state;
  h.workflow.destroy();
  h.workflow.destroy();
  assert.equal(h.pending[0].signal.aborted, true);
  h.pending[0].resolve(green);
  await job;
  await h.workflow.run();
  assert.equal(h.pending.length, 1);
  assert.equal(h.state, state);
  assert.deepEqual(h.renders, []);
});

test("minimal format change maps selection and is one isolated undo event", () => {
  const before = "fn main() {x();}";
  const after = "fn main() { x(); }";
  let state = EditorState.create({
    doc: before,
    selection: { anchor: before.length },
    extensions: [history()],
  });
  const dispatch = (transaction) => {
    state = transaction.state;
  };
  dispatch(
    state.update({
      changes: minimalChange(before, after),
      annotations: [
        Transaction.userEvent.of("input.format"),
        isolateHistory.of("full"),
      ],
    }),
  );
  assert.equal(state.doc.toString(), after);
  assert.equal(state.selection.main.head, after.length);
  assert.equal(undo({ state, dispatch }), true);
  assert.equal(state.doc.toString(), before);
  assert.equal(undo({ state, dispatch }), false);
  assert.deepEqual(minimalChange("abc", "abc"), { from: 3, to: 3, insert: "" });
});

for (const [name, before, after, token] of [
  [
    "multiple reindented lines",
    "fn main() {\n    first();\n    second();\n    third();\n}\n",
    "fn main() {\n  first();\n  second();\n  third();\n}\n",
    "second",
  ],
  [
    "line wrapping",
    "fn main() { call(first, second, third); }",
    "fn main() {\n  call(\n    first,\n    second,\n    third\n  );\n}\n",
    "second",
  ],
]) {
  test(`formatChanges preserves an interior caret and isolates undo across ${name}`, () => {
    const anchor = before.indexOf(token) + 2;
    let state = EditorState.create({
      doc: before,
      selection: { anchor },
      extensions: [history()],
    });
    const dispatch = (transaction) => {
      state = transaction.state;
    };
    const changes = formatChanges(before, after);
    assert.ok(changes.length > 1);
    dispatch(
      state.update({
        changes,
        annotations: [
          Transaction.userEvent.of("input.format"),
          isolateHistory.of("full"),
        ],
      }),
    );
    assert.equal(state.doc.toString(), after);
    assert.equal(state.selection.main.head, after.indexOf(token) + 2);
    assert.equal(undo({ state, dispatch }), true);
    assert.equal(state.doc.toString(), before);
    assert.equal(state.selection.main.head, anchor);
    assert.equal(undo({ state, dispatch }), false);
  });
}

test("formatChanges preserves both endpoints of a selection across indentation edits", () => {
  const before = "    first();\n    second();\n    third();";
  const after = "  first();\n  second();\n  third();";
  const state = EditorState.create({
    doc: before,
    selection: {
      anchor: before.indexOf("first") + 1,
      head: before.indexOf("third") + 3,
    },
  });
  const formatted = state.update({
    changes: formatChanges(before, after),
  }).state;
  assert.equal(formatted.doc.toString(), after);
  assert.equal(formatted.selection.main.anchor, after.indexOf("first") + 1);
  assert.equal(formatted.selection.main.head, after.indexOf("third") + 3);
});

for (const [before, after] of [
  ["", ""],
  ["same", "same"],
  ["", "new\n"],
  ["old\n", ""],
  [" \t\n", "\n  "],
  ["x", " x "],
  [" x ", "x"],
  ["fn f(){a()}", "fn f() { a(); }"],
  ["    a();\n    b()", "  a();\n  b();"],
  ["x + y", "x - y"],
  ["a\r\nb", "a\nb"],
]) {
  test(`formatChanges exactly produces ${JSON.stringify(after)} from ${JSON.stringify(before)}`, () => {
    const changes = formatChanges(before, after);
    if (before === after) assert.deepEqual(changes, []);
    const state = EditorState.create({ doc: before });
    // Apply to the original string too: EditorState normalizes line endings.
    let result = "";
    let cursor = 0;
    for (const change of changes) {
      assert.ok(change.from >= cursor);
      result += before.slice(cursor, change.from) + change.insert;
      cursor = change.to;
    }
    result += before.slice(cursor);
    assert.equal(result, after);
    if (!before.includes("\r"))
      assert.equal(state.update({ changes }).state.doc.toString(), after);
  });
}

test("untouched draft lifecycle never reads or persists a document", () => {
  const window = new EventTarget();
  const document = new EventTarget();
  const controller = createController({
    window,
    document,
    read: () => assert.fail("untouched draft read"),
    persist: () => assert.fail("untouched draft write"),
  });
  controller.flush();
  window.dispatchEvent(new Event("pagehide"));
  document.visibilityState = "hidden";
  document.dispatchEvent(new Event("visibilitychange"));
  controller.destroy();
  controller.destroy();
  controller.schedule();
  controller.flush();
});

test("cargo output includes stdout and stderr", () => {
  assert.equal(
    cargoOutput({ stdout: "hello", stderr: "warning" }),
    "hello\n\nwarning",
  );
  assert.equal(cargoOutput({}), "(no output)");
});

test("drafts flush on hidden, pagehide, and destroy; listeners and timers are removed", async () => {
  const window = new EventTarget();
  const document = new EventTarget();
  let text = "";
  const saved = [];
  const controller = createController({
    window,
    document,
    read: () => text,
    persist: (value) => saved.push(value),
    delay: 5,
  });
  controller.schedule();
  window.dispatchEvent(new Event("pagehide"));
  assert.deepEqual(saved, [""]);
  text = "hidden draft";
  controller.schedule();
  document.visibilityState = "hidden";
  document.dispatchEvent(new Event("visibilitychange"));
  text = "destroy draft";
  controller.schedule();
  controller.destroy();
  controller.destroy();
  window.dispatchEvent(new Event("pagehide"));
  document.dispatchEvent(new Event("visibilitychange"));
  await new Promise((resolve) => setTimeout(resolve, 15));
  assert.deepEqual(saved, ["", "hidden draft", "destroy draft"]);
});
