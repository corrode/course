import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { EditorState } from "@codemirror/state";
import { insertNewlineAndIndent } from "@codemirror/commands";
import { indentUnit } from "@codemirror/language";
import { rust } from "@codemirror/lang-rust";
import { indentRustSource, rustIndentUnit } from "./rust-indent.js";

test("the HashMap starter uses two spaces, including its tests", () => {
  const source = readFileSync(new URL("../../examples/09_hashmaps/3_set_config_value.rs", import.meta.url), "utf8");
  const result = indentRustSource(source);
  assert.ok(result.includes("\n  todo!()\n"));
  assert.ok(result.includes("\n  let mut config = HashMap::new();"));
  assert.equal(indentRustSource(result), result);
});

test("formatter output gets two spaces per nesting level", () => {
  assert.equal(
    indentRustSource("fn main() {\n    if true {\n        todo!()\n    }\n}\n"),
    "fn main() {\n  if true {\n    todo!()\n  }\n}\n",
  );
});

test("multiline strings, raw strings, and comment contents are preserved", () => {
  for (const literal of ['"hello\n    world\n        end"', 'r#"hello\n    world\n        end"#', '/* hello\n    world\n        end */']) {
    const result = indentRustSource(`fn main() {\n    ${literal};\n}\n`);
    assert.ok(result.includes(`\n  ${literal};\n`));
  }
});

test("newlines match the normalized starter indentation", () => {
  const doc = indentRustSource("fn main() {\n    todo!()\n}\n");
  let state = EditorState.create({
    doc,
    selection: { anchor: doc.indexOf("todo!()") + "todo!()".length },
    extensions: [rust(), indentUnit.of(rustIndentUnit)],
  });
  insertNewlineAndIndent({ state, dispatch: (transaction) => { state = transaction.state; } });
  assert.equal(state.doc.toString(), "fn main() {\n  todo!()\n  \n}\n");
});

test("empty and incomplete exercise code is supported", () => {
  assert.equal(indentRustSource(""), "");
  assert.equal(indentRustSource("fn main() {\n    todo!()"), "fn main() {\n  todo!()");
});
