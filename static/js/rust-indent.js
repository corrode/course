import { EditorState } from "@codemirror/state";
import { ensureSyntaxTree, indentRange, indentUnit } from "@codemirror/language";
import { rust } from "@codemirror/lang-rust";

export const rustIndentUnit = "  ";

// Reindent source, not its rendering: copying and running use the same text.
export function indentRustSource(source) {
  const state = EditorState.create({
    doc: source,
    extensions: [rust(), indentUnit.of(rustIndentUnit)],
  });
  const tree = ensureSyntaxTree(state, state.doc.length, 1000);
  if (!tree) return source;

  const protectedRanges = [];
  tree.iterate({
    enter(node) {
      // Indentation inside multiline literals is data, not code. Preserve
      // comments too, including deliberately aligned diagrams and examples.
      if (/String|Char|Comment/.test(node.name)) {
        protectedRanges.push({ from: node.from, to: node.to });
        return false;
      }
    },
  });
  const changes = [];
  indentRange(state, 0, state.doc.length).iterChanges((from, to, _, __, insert) => {
    if (!protectedRanges.some((range) => from > range.from && from < range.to)) {
      changes.push({ from, to, insert });
    }
  });
  return state.changes(changes).apply(state.doc).toString();
}
