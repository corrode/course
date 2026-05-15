// Shared CodeMirror 6 theme + Lezer highlight style.
//
// Reads the same `--syn-*` CSS variables defined in
// `templates/base.html`, so any editor that uses these picks up
// light/dark mode for free when the user flips `[data-theme]` on
// the document element.
//
// Used by:
//   - templates/exercise.html (the editable per-step editor and the
//     read-only run-output editor)
//   - static/js/readonly-rust-editor.js (admin/team submissions)
//
// Kept as a module-level singleton so a page can have any number of
// editors sharing one style/theme without per-mount allocation.

import { EditorView } from "@codemirror/view";
import { HighlightStyle } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";

export const proseHighlightStyle = HighlightStyle.define([
    { tag: t.keyword, color: "var(--syn-keyword)" },
    { tag: t.controlKeyword, color: "var(--syn-keyword)" },
    { tag: t.moduleKeyword, color: "var(--syn-keyword)" },
    {
        tag: [t.typeName, t.className, t.namespace],
        color: "var(--syn-type)",
    },
    {
        tag: [t.string, t.special(t.string), t.regexp],
        color: "var(--syn-string)",
    },
    {
        tag: [t.number, t.bool, t.atom, t.literal],
        color: "var(--syn-number)",
    },
    {
        tag: [
            t.function(t.definition(t.variableName)),
            t.definition(t.variableName),
            t.definition(t.propertyName),
        ],
        color: "var(--syn-definition)",
    },
    {
        tag: [t.macroName, t.meta, t.annotation],
        color: "var(--syn-macro)",
    },
    {
        tag: [t.comment, t.lineComment, t.blockComment, t.docComment],
        color: "var(--syn-comment)",
        fontStyle: "italic",
    },
    {
        tag: [t.operator, t.punctuation, t.separator, t.bracket],
        color: "var(--syn-punctuation)",
    },
    { tag: t.invalid, color: "var(--syn-invalid)" },
]);

export const proseEditorTheme = EditorView.theme({
    "&": {
        color: "var(--syn-fg)",
        backgroundColor: "var(--syn-bg)",
    },
    ".cm-content": { caretColor: "var(--syn-cursor)" },
    ".cm-cursor, .cm-dropCursor": {
        borderLeftColor: "var(--syn-cursor)",
    },
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection":
        {
            backgroundColor: "var(--syn-selection-bg)",
        },
    ".cm-activeLine": { backgroundColor: "var(--syn-active-line)" },
    ".cm-gutters": {
        backgroundColor: "var(--syn-gutter-bg)",
        color: "var(--syn-gutter-fg)",
        border: "none",
    },
    ".cm-activeLineGutter": {
        backgroundColor: "var(--syn-active-line)",
    },
    ".cm-tooltip": {
        backgroundColor: "var(--syn-bg)",
        color: "var(--syn-fg)",
        border: "1px solid var(--syn-selection-bg)",
        borderRadius: "4px",
        boxShadow: "0 4px 12px rgba(0,0,0,0.25)",
    },
    ".cm-tooltip.cm-tooltip-autocomplete > ul": {
        fontFamily: "inherit",
        maxHeight: "14rem",
    },
    ".cm-tooltip.cm-tooltip-autocomplete > ul > li": {
        color: "var(--syn-fg)",
        padding: "2px 6px",
    },
    ".cm-tooltip-autocomplete ul li[aria-selected]": {
        backgroundColor: "var(--syn-selection-bg)",
        color: "var(--syn-fg)",
    },
    ".cm-completionLabel": { color: "var(--syn-fg)" },
    ".cm-completionDetail": {
        color: "var(--syn-gutter-fg)",
        fontStyle: "italic",
    },
    ".cm-completionIcon": {
        color: "var(--syn-keyword)",
        opacity: 0.85,
    },
});
