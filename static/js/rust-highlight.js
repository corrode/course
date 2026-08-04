import { parser } from "@lezer/rust";
import { classHighlighter, highlightTree } from "@lezer/highlight";

function escapeHtml(value) {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

export function highlightRust(code) {
  const tree = parser.parse(code);
  let output = "";
  let position = 0;
  highlightTree(tree, classHighlighter, (from, to, className) => {
    if (from > position) output += escapeHtml(code.slice(position, from));
    output += `<span class="${className}">${escapeHtml(code.slice(from, to))}</span>`;
    position = to;
  });
  if (position < code.length) output += escapeHtml(code.slice(position));
  return output;
}
