// Read-only CodeMirror 6 viewer for Rust source blocks on the
// admin and team pages, and the "Reveal the full solution" viewer on
// chapter pages.
//
// Resolves CM6 modules through the shared importmap in
// `templates/base.html`, so this viewer and the editable editor on
// chapter pages share a single instance per CM6 module (no duplicate
// `@codemirror/state`, no broken `instanceof` checks).
//
// Markup contract:
//
//     <div class="rust-editor" data-rust-source>fn main() { ... }</div>
//
// The element's textContent is read once, the element is emptied,
// and a CodeMirror EditorView is mounted in its place. Visually the
// result matches the in-exercise editor: same font, same syntax
// colors (driven by the same `--syn-*` CSS variables), same border
// radius. The viewer is non-editable and read-only.

import { EditorState } from "@codemirror/state";
import {
  EditorView,
  lineNumbers,
  highlightActiveLine,
  highlightActiveLineGutter,
  drawSelection,
} from "@codemirror/view";
import { syntaxHighlighting, bracketMatching } from "@codemirror/language";
import { rust } from "@codemirror/lang-rust";

import { proseHighlightStyle, proseEditorTheme } from "./cm-theme.js";

function mountOne(el) {
  // Idempotent: a second call (e.g. lazy mount when a <details> opens
  // after the page-load auto-run already handled it) is a no-op and
  // returns the existing view so callers can still `requestMeasure()`.
  if (el.dataset.cmMounted === "1") {
    return el.__cmView ?? null;
  }

  const source = el.textContent ?? "";

  let view;
  try {
    el.textContent = "";
    view = new EditorView({
      state: EditorState.create({
        doc: source,
        extensions: [
          EditorView.editable.of(false),
          EditorState.readOnly.of(true),
          EditorView.lineWrapping,
          lineNumbers(),
          highlightActiveLine(),
          highlightActiveLineGutter(),
          drawSelection(),
          bracketMatching(),
          rust(),
          syntaxHighlighting(proseHighlightStyle, { fallback: true }),
          proseEditorTheme,
        ],
      }),
      parent: el,
    });
  } catch (err) {
    // Construction failed: restore the source so the styled fallback box
    // stays readable, and leave the element unmarked so a later call
    // (e.g. the next time the <details> opens) can retry.
    el.textContent = source;
    throw err;
  }

  // Match the chrome the in-exercise editor sets directly on its
  // root DOM node (see `initSection` in `templates/exercise.html`).
  view.dom.style.border = "1px solid var(--color-border)";
  view.dom.style.borderRadius = "12px";
  view.dom.style.overflow = "hidden";
  // Font size is set in CSS (see `base.html`) so the `/settings`
  // page can override it via `html[data-editor-font-size]`.

  // Only mark as mounted once construction succeeded, so a failed mount
  // doesn't permanently block a retry.
  el.dataset.cmMounted = "1";
  el.__cmView = view;
  return view;
}

// Mount a single read-only viewer on demand. Used by chapter pages to
// lazily mount the solution viewer the first time its <details> opens
// (mounting while hidden would measure to zero height). Idempotent.
export function mountReadonlyRustEditor(el) {
  try {
    return mountOne(el);
  } catch (err) {
    console.warn("[corrode] readonly editor mount failed:", err, el);
    return null;
  }
}

export function mountAllReadonlyRustEditors(selector = "[data-rust-source]") {
  const els = document.querySelectorAll(selector);
  for (const el of els) {
    try {
      mountOne(el);
    } catch (err) {
      // One bad block shouldn't take the whole page down. Leave
      // the original textContent visible (the template already
      // wraps it in a styled .rust-editor box) and log so the
      // operator can spot the regression in devtools.
      console.warn("[corrode] readonly editor mount failed:", err, el);
    }
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", () => {
    mountAllReadonlyRustEditors();
  });
} else {
  mountAllReadonlyRustEditors();
}
