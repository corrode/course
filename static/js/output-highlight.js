import { parser } from "@lezer/rust";
import { classHighlighter, highlightTree } from "@lezer/highlight";

// Logs mix prose and Rust. Only parse source excerpts, assertion values, and
// backtick-delimited expressions as Rust; leave arbitrary program output alone.
export function highlightOutput(element, text) {
  const document = element.ownerDocument;
  element.replaceChildren();
  const append = (value, className = "") => {
    if (!value) return;
    const span = document.createElement("span");
    span.className = className;
    span.textContent = value;
    element.append(span);
  };
  const rust = (code, expression = true) => {
    const prefix = expression ? "fn main() { let _ = " : "fn main() { ";
    const source = prefix + code + "; }";
    let position = 0;
    highlightTree(parser.parse(source), classHighlighter, (from, to, cls) => {
      from = Math.max(0, from - prefix.length);
      to = Math.min(code.length, to - prefix.length);
      if (to <= from) return;
      append(code.slice(position, from));
      append(code.slice(from, to), cls);
      position = to;
    });
    append(code.slice(position));
  };
  for (const line of text.split(/(?<=\n)/)) {
    const value = line.match(/^(\s*(?:left|right):\s*)(.*)$/s);
    const source = line.match(/^(\s*\d+\s*\|\s?)(.*)$/s);
    if (value || source) {
      const match = value || source;
      append(match[1], "output-location");
      rust(match[2], !!value);
      continue;
    }
    const diagnostic = line.match(/^(\s*)(error(?:\[E\d+\])?|warning|note|help)(:)/);
    if (diagnostic) {
      append(diagnostic[1]);
      append(diagnostic[2], `output-${diagnostic[2].split("[")[0]}`);
    }
    const start = diagnostic ? diagnostic[0].length - 1 : 0;
    const rest = line.slice(start);
    const status = /\b(?:FAILED|panicked|assertion .* failed)\b/.test(line)
      ? "output-error"
      : /^(?:test .* \.\.\. ok|test result: ok)\b/.test(line)
        ? "output-success"
        : /^\s*(?:-->|\||=)/.test(line)
          ? "output-location"
          : "";
    let position = 0;
    for (const match of rest.matchAll(/`([^`\n]+)`/g)) {
      append(rest.slice(position, match.index), status);
      append("`");
      rust(match[1]);
      append("`");
      position = match.index + match[0].length;
    }
    append(rest.slice(position), status);
  }
}
