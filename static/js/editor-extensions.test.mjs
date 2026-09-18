import assert from "node:assert/strict";
import test from "node:test";
import { EditorState } from "@codemirror/state";
import { CompletionContext } from "@codemirror/autocomplete";
import { rust } from "@codemirror/lang-rust";
import {
  buildLocalCompletions,
  createUrlExtensions,
  urlAtPosition,
} from "./editor-extensions.js";

function complete(
  source,
  explicit = false,
  completion = buildLocalCompletions(),
) {
  const pos = source.indexOf("|");
  const state = EditorState.create({
    doc: source.replace("|", ""),
    extensions: [rust()],
  });
  return completion(new CompletionContext(state, pos, explicit));
}

test("completion is suppressed in comments and literals", () => {
  for (const source of [
    "// hel|lo",
    "/* hel|lo */",
    'fn main() { "hel|lo"; }',
    'fn main() { r#"hel|lo"#; }',
    "fn main() { 'a|'; }",
  ]) {
    assert.equal(complete(source), null, source);
    assert.equal(complete(source, true), null, source);
  }
});

test("completion collects code identifiers rather than prose and string words", () => {
  const result = complete(
    'fn main() { let document_name = "string_noise"; /* comment_noise */ doc| }',
  );
  const names = result.options.map((option) => option.label);
  assert.ok(names.includes("document_name"));
  assert.ok(!names.includes("string_noise"));
  assert.ok(!names.includes("comment_noise"));
  assert.equal(
    result.options.find((option) => option.label === "String").type,
    "type",
  );
  assert.equal(
    result.options.find((option) => option.label === "println!").type,
    "function",
  );
});

test("explicit completion also works at an empty position", () => {
  assert.equal(complete("fn main() { | }"), null);
  assert.ok(complete("fn main() { | }", true).options.length > 0);
});

test("completion options are reused across cursor-only transactions", () => {
  const state = EditorState.create({
    doc: "fn main() { let message = 1; mes }",
    extensions: [rust()],
  });
  const source = buildLocalCompletions();
  const pos = state.doc.length - 2;
  const first = source(new CompletionContext(state, pos, false));
  const next = state.update({ selection: { anchor: pos } }).state;
  assert.equal(
    source(new CompletionContext(next, pos, false)).options,
    first.options,
  );
});

test("URL lookup reads the full document URL and trims prose punctuation", () => {
  for (const [text, expected] of [
    [
      "// See https://example.com/path?q=yes#part.",
      "https://example.com/path?q=yes#part",
    ],
    [
      "// (https://example.com/wiki/Foo_(bar)).",
      "https://example.com/wiki/Foo_(bar)",
    ],
    ["// https://[::1]/path", "https://[::1]/path"],
  ]) {
    const state = EditorState.create({ doc: text });
    assert.equal(urlAtPosition(state, text.indexOf("https") + 12), expected);
    assert.equal(urlAtPosition(state, 0), null);
  }
});

test("plain clicks edit URLs; modified and middle clicks open the full URL", () => {
  let handlers;
  createUrlExtensions({
    EditorView: { theme: (value) => value },
    Decoration: { mark: (value) => value },
    ViewPlugin: {
      fromClass: (_class, spec) => {
        handlers = spec.eventHandlers;
      },
    },
    MatchDecorator: class {},
  });
  const opened = [];
  const view = {
    state: EditorState.create({ doc: "// https://example.com/full/path" }),
    posAtCoords: () => 20,
    dom: {
      ownerDocument: { defaultView: { open: (...args) => opened.push(args) } },
    },
  };
  const event = { button: 0, clientX: 0, clientY: 0 };
  assert.equal(handlers.mousedown(event, view), false);
  assert.equal(opened.length, 0);
  assert.equal(handlers.mousedown({ ...event, ctrlKey: true }, view), true);
  assert.equal(handlers.mousedown({ ...event, metaKey: true }, view), true);
  assert.equal(handlers.mousedown({ ...event, button: 1 }, view), true);
  assert.equal(opened.length, 2);
  assert.equal(handlers.auxclick({ ...event, button: 1 }, view), true);
  assert.deepEqual(opened[2], [
    "https://example.com/full/path",
    "_blank",
    "noopener,noreferrer",
  ]);
});
