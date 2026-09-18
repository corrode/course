import assert from "node:assert/strict";
import test from "node:test";
import { EditorState } from "@codemirror/state";
import { insertNewlineAndIndent, indentMore } from "@codemirror/commands";
import { indentUnit } from "@codemirror/language";
import { rust } from "@codemirror/lang-rust";
import { rustIndentUnit } from "./rust-indent.js";

function applyCommand(doc, anchor, command) {
  let state = EditorState.create({
    doc,
    selection: { anchor },
    extensions: [rust(), indentUnit.of(rustIndentUnit)],
  });
  command({ state, dispatch: (transaction) => { state = transaction.state; } });
  return state;
}

test("Enter after an opening brace uses rustfmt's four-space default", () => {
  const doc = "fn char_count(text: &str) -> usize {\n}";
  const state = applyCommand(doc, doc.indexOf("{") + 1, insertNewlineAndIndent);
  assert.equal(state.doc.toString(), "fn char_count(text: &str) -> usize {\n    \n}");
  assert.equal(state.selection.main.head, doc.indexOf("{") + 6);
});

test("nested newlines add four spaces per level", () => {
  const doc = "fn main() {\n    if true {\n    }\n}";
  const state = applyCommand(doc, doc.indexOf("if true {") + "if true {".length, insertNewlineAndIndent);
  assert.equal(state.doc.toString(), "fn main() {\n    if true {\n        \n    }\n}");
});

test("indent commands use four spaces rather than tabs", () => {
  const state = applyCommand("todo!()", 0, indentMore);
  assert.equal(state.doc.toString(), "    todo!()");
});
