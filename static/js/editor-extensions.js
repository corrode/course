import { syntaxTree } from "@codemirror/language";

const keywords =
  "as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while box try union";
const types =
  "bool char str i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 Option Result Vec String Box Rc Arc RefCell Cell HashMap HashSet BTreeMap BTreeSet Clone Copy Debug Display Default PartialEq Eq PartialOrd Ord Hash From Into TryFrom TryInto AsRef AsMut Iterator IntoIterator Drop Send Sync Sized Fn FnMut FnOnce";
const macros =
  "println! print! eprintln! eprint! format! write! writeln! vec! panic! todo! unimplemented! unreachable! assert! assert_eq! assert_ne! debug_assert! debug_assert_eq! debug_assert_ne! matches! dbg! include_str! env!";
const standardOptions = [
  ...keywords.split(" ").map((label) => ({ label, type: "keyword" })),
  ...types.split(" ").map((label) => ({ label, type: "type" })),
  ...macros.split(" ").map((label) => ({ label, type: "function" })),
  ...["Some", "None", "Ok", "Err"].map((label) => ({ label, type: "enum" })),
];
const standardNames = new Set(standardOptions.map((option) => option.label));
const literalOrComment = /(?:String|Char|Comment)$/;
const identifier =
  /^(?:BoundIdentifier|Identifier|TypeIdentifier|FieldIdentifier)$/;

export function buildLocalCompletions() {
  // A selection-only transaction shares its immutable document and syntax tree.
  // Cache both, so background parsing can still add newly discovered names.
  let cachedDoc;
  let cachedTree;
  let options;
  return (context) => {
    const tree = syntaxTree(context.state);
    const node = tree.resolveInner(context.pos, -1);
    for (let parent = node; parent; parent = parent.parent) {
      if (literalOrComment.test(parent.name)) return null;
    }
    const word = context.matchBefore(/[A-Za-z_][A-Za-z0-9_]*/);
    if (!word && !context.explicit) return null;
    if (cachedDoc !== context.state.doc || cachedTree !== tree) {
      const seen = new Set(standardNames);
      options = [...standardOptions];
      tree.iterate({
        enter(node) {
          if (literalOrComment.test(node.name)) return false;
          if (!identifier.test(node.name)) return;
          const label = context.state.sliceDoc(node.from, node.to);
          if (label.length < 3 || seen.has(label)) return;
          seen.add(label);
          options.push({
            label,
            type:
              node.name === "TypeIdentifier"
                ? "type"
                : node.name === "FieldIdentifier"
                  ? "property"
                  : "variable",
          });
        },
      });
      cachedDoc = context.state.doc;
      cachedTree = tree;
    }
    return {
      from: word?.from ?? context.pos,
      options,
      validFor: /^[A-Za-z0-9_]*$/,
    };
  };
}

const urlPattern = /\bhttps?:\/\/[^\s<>"'`]+/g;

function trimUrl(text) {
  let url = text.replace(/[.,;:!?]+$/, "");
  // A prose closing delimiter is not part of the URL, but balanced delimiters
  // are (for example Wikipedia paths and IPv6 addresses).
  while (url.length) {
    const closing = url.at(-1);
    const opening = { ")": "(", "]": "[", "}": "{" }[closing];
    if (!opening) break;
    const count = (character) => url.split(character).length - 1;
    if (count(closing) <= count(opening)) break;
    url = url.slice(0, -1).replace(/[.,;:!?]+$/, "");
  }
  return url;
}

export function urlAtPosition(state, position) {
  const line = state.doc.lineAt(position);
  for (const match of line.text.matchAll(urlPattern)) {
    const url = trimUrl(match[0]);
    const from = line.from + match.index;
    if (position >= from && position < from + url.length) return url;
  }
  return null;
}

export function createUrlExtensions({
  EditorView,
  Decoration,
  ViewPlugin,
  MatchDecorator,
}) {
  const mark = Decoration.mark({
    class: "cm-url",
    attributes: { title: "Ctrl/Command-click or middle-click to open" },
  });
  const matcher = new MatchDecorator({
    regexp: urlPattern,
    decorate(add, from, _to, match) {
      const url = trimUrl(match[0]);
      if (url) add(from, from + url.length, mark);
    },
  });
  function openLink(event, view) {
    const position = view.posAtCoords({ x: event.clientX, y: event.clientY });
    if (position === null) return false;
    const url = urlAtPosition(view.state, position);
    if (!url) return false;
    view.dom.ownerDocument.defaultView.open(
      url,
      "_blank",
      "noopener,noreferrer",
    );
    return true;
  }
  return [
    ViewPlugin.fromClass(
      class {
        constructor(view) {
          this.decorations = matcher.createDeco(view);
        }
        update(update) {
          this.decorations = matcher.updateDeco(update, this.decorations);
        }
      },
      {
        decorations: (plugin) => plugin.decorations,
        eventHandlers: {
          mousedown(event, view) {
            if (event.button === 0 && (event.metaKey || event.ctrlKey))
              return openLink(event, view);
            // Suppress autoscroll on links; auxclick opens them on release.
            if (event.button === 1) {
              const position = view.posAtCoords({
                x: event.clientX,
                y: event.clientY,
              });
              return (
                position !== null &&
                urlAtPosition(view.state, position) !== null
              );
            }
            return false;
          },
          auxclick(event, view) {
            return event.button === 1 && openLink(event, view);
          },
        },
      },
    ),
    EditorView.theme({
      ".cm-url": {
        textDecoration: "underline",
        textDecorationColor: "var(--syn-gutter-fg)",
        textUnderlineOffset: "2px",
      },
      ".cm-url:hover": { textDecorationColor: "currentColor" },
    }),
  ];
}
