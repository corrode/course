// Read-only CodeMirror 6 editor for Rust source blocks on the
// admin and team pages.
//
// We deliberately don't use the importmap from `templates/exercise.html`
// here. The exercise page wires up a much heavier editor (vim mode,
// autocomplete, run/submit plumbing) and pins exact versions through
// the importmap; this script only wants the read-only viewer, so it
// imports from plain esm.sh URLs and lets esm.sh resolve transitive
// deps. Two CodeMirror copies on a single page would be a problem for
// the exercise page (state sharing across modules), but admin and
// team pages don't load the heavier module so there's no clash.
//
// Markup contract:
//
//     <div class="rust-editor" data-rust-source>fn main() { ... }</div>
//
// The element's textContent is read once, the element is emptied,
// and a CodeMirror EditorView is mounted in its place. Call
// `mountAllReadonlyRustEditors()` (the default export) once after
// the DOM is ready, or pass an explicit selector for a subset.

const ESM = "https://esm.sh";

async function loadCodeMirror() {
    const [state, view, language, langRust, highlight] = await Promise.all([
        import(`${ESM}/@codemirror/state@6.4.1`),
        import(`${ESM}/@codemirror/view@6.26.3`),
        import(`${ESM}/@codemirror/language@6.10.2`),
        import(`${ESM}/@codemirror/lang-rust@6.0.1`),
        import(`${ESM}/@lezer/highlight@1.2.0`),
    ]);
    return { state, view, language, langRust, highlight };
}

// Reads the same `--syn-*` / CodeMirror CSS variables defined in
// `templates/base.html`, so the read-only viewer here picks up the
// same colours as the run-output editor on the exercise page when
// the user flips light/dark.
function buildTheme(EditorView) {
    return EditorView.theme(
        {
            "&": {
                color: "var(--syn-fg, var(--color-text))",
                backgroundColor: "var(--syn-bg, var(--color-surface))",
                fontSize: "0.9rem",
                borderRadius: "8px",
                border: "1px solid var(--color-border)",
            },
            ".cm-content": {
                fontFamily:
                    "'JetBrains Mono', 'SF Mono', Monaco, Menlo, monospace",
                padding: "0.85rem 0",
                caretColor: "transparent",
            },
            ".cm-gutters": {
                backgroundColor: "transparent",
                color: "var(--color-text-muted)",
                border: "none",
                paddingRight: "0.4rem",
            },
            ".cm-activeLine, .cm-activeLineGutter": {
                backgroundColor: "transparent",
            },
            ".cm-scroller": {
                lineHeight: "1.55",
            },
        },
        { dark: false },
    );
}

function buildHighlightStyle(language, highlight) {
    const { HighlightStyle } = language;
    const { tags } = highlight;
    return HighlightStyle.define([
        { tag: tags.keyword, color: "var(--syn-keyword, #c678dd)" },
        { tag: tags.controlKeyword, color: "var(--syn-keyword, #c678dd)" },
        { tag: tags.operatorKeyword, color: "var(--syn-keyword, #c678dd)" },
        { tag: tags.string, color: "var(--syn-string, #98c379)" },
        { tag: tags.number, color: "var(--syn-number, #d19a66)" },
        { tag: tags.bool, color: "var(--syn-number, #d19a66)" },
        { tag: tags.comment, color: "var(--syn-comment, #7f848e)", fontStyle: "italic" },
        { tag: tags.lineComment, color: "var(--syn-comment, #7f848e)", fontStyle: "italic" },
        { tag: tags.blockComment, color: "var(--syn-comment, #7f848e)", fontStyle: "italic" },
        { tag: tags.function(tags.variableName), color: "var(--syn-function, #61afef)" },
        { tag: tags.function(tags.definition(tags.variableName)), color: "var(--syn-function, #61afef)" },
        { tag: tags.typeName, color: "var(--syn-type, #e5c07b)" },
        { tag: tags.className, color: "var(--syn-type, #e5c07b)" },
        { tag: tags.namespace, color: "var(--syn-type, #e5c07b)" },
        { tag: tags.macroName, color: "var(--syn-macro, #56b6c2)" },
        { tag: tags.attributeName, color: "var(--syn-attr, #d19a66)" },
        { tag: tags.propertyName, color: "var(--syn-fg, inherit)" },
        { tag: tags.variableName, color: "var(--syn-fg, inherit)" },
        { tag: tags.punctuation, color: "var(--syn-fg, inherit)" },
        { tag: tags.bracket, color: "var(--syn-fg, inherit)" },
    ]);
}

export async function mountAllReadonlyRustEditors(selector = "[data-rust-source]") {
    const els = Array.from(document.querySelectorAll(selector));
    if (!els.length) return;

    let mods;
    try {
        mods = await loadCodeMirror();
    } catch (err) {
        // Network blocked / esm.sh down: leave the original textContent
        // visible (already wrapped in <pre><code> by the template) so
        // the page still works without highlighting.
        console.warn("[corrode] read-only editor failed to load:", err);
        return;
    }

    const { state, view, language, langRust, highlight } = mods;
    const { EditorState } = state;
    const { EditorView, lineNumbers, drawSelection } = view;
    const { syntaxHighlighting } = language;
    const theme = buildTheme(EditorView);
    const style = buildHighlightStyle(language, highlight);

    for (const el of els) {
        const source = el.dataset.rustSourceText ?? el.textContent ?? "";
        // Drop the placeholder content before mounting so the editor
        // doesn't render on top of it.
        el.textContent = "";
        const extensions = [
            EditorView.editable.of(false),
            EditorState.readOnly.of(true),
            EditorView.lineWrapping,
            lineNumbers(),
            drawSelection(),
            langRust.rust(),
            syntaxHighlighting(style, { fallback: true }),
            theme,
        ];
        new EditorView({
            state: EditorState.create({ doc: source, extensions }),
            parent: el,
        });
    }
}

// Auto-run on DOM ready so callers only need to drop a
// `<script type="module" src=".../readonly-rust-editor.js"></script>`
// onto the page. The default selector matches every element with a
// `data-rust-source` attribute.
if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => {
        mountAllReadonlyRustEditors();
    });
} else {
    mountAllReadonlyRustEditors();
}
