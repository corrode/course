// Single CodeMirror 6 mount used by every page that embeds an editor:
// chapter pages (`templates/exercise.html`), the homepage warm-up
// (`templates/dashboard.html`), and the scratchpad
// (`templates/playground.html`).
//
// See `course/TODO/phase_2b_extract_inline_editor.md` for the full design.
//
// DOM contract (every section root must follow it; missing optional roles
// are tolerated):
//
//     <section class="exercise-section"
//              data-step-id="<unique>"
//              data-exercise-key="<chapter>/<step>">
//       <div class="exercise-section-head">
//         <span data-role="action-status"></span>
//         <button data-role="reset-btn">Reset</button>           (optional)
//         <a    data-role="vscode-btn" href="...">Open</a>       (optional)
//         <button data-role="format-btn">Format</button>         (optional)
//         <button data-role="run-btn">▶ Run</button>
//         <button data-role="submit-btn" data-ulid="...">Submit</button>
//                                                                (optional)
//       </div>
//       <div style="position: relative">
//         <textarea data-role="editor-fallback">…starter code…</textarea>
//         <div data-role="editor-mount" style="display: none"></div>
//         <button data-role="copy-btn">…</button>                (optional)
//       </div>
//       <div data-role="run-status-row">
//         <span data-role="run-spinner"></span>
//         <span data-role="run-status"></span>
//       </div>
//       <div data-role="output-panel">
//         <ul data-role="test-list"></ul>                        (optional)
//         <details data-role="output-details">
//           <pre data-role="output-stderr"></pre>
//         </details>
//       </div>
//     </section>
//
// Feature matrix (`features.*` knobs):
//   vim                    -> show vim toggle button + compartment
//   draftKey               -> persist edits to localStorage under this key
//   submit                 -> { ulid, exerciseKey } enables Submit button
//                             that POSTs to /api/submit
//   testResults            -> render `<ul data-role="test-list">` rows and
//                             parse failure snippets out of cargo stderr
//   copyButton             -> wire `data-role="copy-btn"` for clipboard copy
//   urlPlugin              -> middle-click http(s) URLs in the editor open
//                             in a new tab
//   syntaxHighlightOutput  -> mount a read-only CM6 view inside
//                             `data-role="output-stderr"` to colourise
//                             cargo output (defaults to features.testResults)
//   runWithoutTests        -> send `tests: false` to /api/run so the
//                             upstream Playground runs `main()` instead of
//                             `cargo test`. The scratchpad uses this so
//                             `println!` / `dbg!` output actually shows up.
//
// Vim toggle is *global* across every mounted editor on the page (it would
// be jarring to have one section in vim mode and another not). Each mount
// registers itself in `activeMounts`; toggling any vim button flips them
// all. We use a module-local set rather than DOM events so a future page
// can mount editors after the toggle was already flipped.
//
// Fallback path: if any of the CodeMirror imports fail (esm.sh blocked,
// offline, etc.) the module wires Run/Format/Reset/Submit against the
// plain `<textarea data-role="editor-fallback">` and returns the same
// handle so call sites don't need to branch.

import { proseHighlightStyle, proseEditorTheme } from "./cm-theme.js";

const VIM_PREF_KEY = "corrode:editor:vim";
/** @type {Set<{setVim: (on:boolean)=>void}>} */
const activeMounts = new Set();

export function vimGlobalEnabled() {
  try {
    return localStorage.getItem(VIM_PREF_KEY) === "1";
  } catch (_) {
    return false;
  }
}

export function setVimGlobal(enabled) {
  try {
    localStorage.setItem(VIM_PREF_KEY, enabled ? "1" : "0");
  } catch (_) {}
  for (const m of activeMounts) m.setVim(enabled);
}

// Curated keyword list: small enough to be useful, large enough to cover
// the prelude bits that show up in nearly every exercise.
const RUST_KEYWORDS = [
  "as",
  "async",
  "await",
  "break",
  "const",
  "continue",
  "crate",
  "dyn",
  "else",
  "enum",
  "extern",
  "false",
  "fn",
  "for",
  "if",
  "impl",
  "in",
  "let",
  "loop",
  "match",
  "mod",
  "move",
  "mut",
  "pub",
  "ref",
  "return",
  "self",
  "Self",
  "static",
  "struct",
  "super",
  "trait",
  "true",
  "type",
  "unsafe",
  "use",
  "where",
  "while",
  "box",
  "try",
  "union",
  "bool",
  "char",
  "str",
  "i8",
  "i16",
  "i32",
  "i64",
  "i128",
  "isize",
  "u8",
  "u16",
  "u32",
  "u64",
  "u128",
  "usize",
  "f32",
  "f64",
  "Some",
  "None",
  "Ok",
  "Err",
  "Option",
  "Result",
  "Vec",
  "String",
  "Box",
  "Rc",
  "Arc",
  "RefCell",
  "Cell",
  "HashMap",
  "HashSet",
  "BTreeMap",
  "BTreeSet",
  "Clone",
  "Copy",
  "Debug",
  "Display",
  "Default",
  "PartialEq",
  "Eq",
  "PartialOrd",
  "Ord",
  "Hash",
  "From",
  "Into",
  "TryFrom",
  "TryInto",
  "AsRef",
  "AsMut",
  "Iterator",
  "IntoIterator",
  "Drop",
  "Send",
  "Sync",
  "Sized",
  "Fn",
  "FnMut",
  "FnOnce",
  "println!",
  "print!",
  "eprintln!",
  "eprint!",
  "format!",
  "write!",
  "writeln!",
  "vec!",
  "panic!",
  "todo!",
  "unimplemented!",
  "unreachable!",
  "assert!",
  "assert_eq!",
  "assert_ne!",
  "debug_assert!",
  "debug_assert_eq!",
  "debug_assert_ne!",
  "matches!",
  "dbg!",
  "include_str!",
  "env!",
];
const IDENT_RE = /[A-Za-z_][A-Za-z0-9_]*/g;

const URL_RE = /\bhttps?:\/\/[^\s<>"'`)\]]+/g;

function buildLocalCompletions() {
  return function localCompletions(context) {
    const word = context.matchBefore(/[A-Za-z_][A-Za-z0-9_]*/);
    if (!word || (word.from === word.to && !context.explicit)) return null;
    const seen = new Set();
    const options = [];
    for (const kw of RUST_KEYWORDS) {
      if (kw !== word.text) {
        seen.add(kw);
        options.push({ label: kw, type: "keyword" });
      }
    }
    const text = context.state.doc.toString();
    IDENT_RE.lastIndex = 0;
    let m;
    while ((m = IDENT_RE.exec(text))) {
      const ident = m[0];
      if (ident.length < 3) continue;
      if (ident === word.text) continue;
      if (seen.has(ident)) continue;
      seen.add(ident);
      options.push({ label: ident, type: "variable" });
    }
    return { from: word.from, options, validFor: /^[A-Za-z0-9_]*$/ };
  };
}

function extractFailureSnippets(raw) {
  const out = {};
  if (!raw) return out;
  const re =
    /----\s+(\S+)\s+stdout\s+----\n([\s\S]*?)(?=\n----|\nfailures:|\ntest result:|$)/g;
  let m;
  while ((m = re.exec(raw)) !== null) {
    out[m[1].trim()] = m[2].trim();
  }
  return out;
}

function friendlyFailureSnippet(snippet) {
  if (!snippet) return null;
  if (/not yet implemented/i.test(snippet)) {
    return (
      "This function still has `todo!()` in it.\n" +
      "Replace `todo!()` with your implementation, then run again."
    );
  }
  const lines = snippet.split("\n");
  while (lines.length && /^thread .* panicked at/.test(lines[0])) {
    lines.shift();
  }
  const trimmed = lines.slice(0, 6).join("\n").trim();
  return trimmed || null;
}

/**
 * Mount a single inline editor inside `section`. Returns a handle:
 *   { getValue, setValue, focus, setVim, destroy }
 *
 * Always resolves (never rejects); on failure the handle still works
 * against the plain textarea fallback.
 */
export async function mountInlineEditor(section, opts = {}) {
  const features = opts.features || {};
  const $ = (role) => section.querySelector(`[data-role="${role}"]`);

  const fallback = $("editor-fallback");
  const mount = $("editor-mount");
  const runBtn = $("run-btn");
  const runStatus = $("run-status");
  const runSpinner = $("run-spinner");
  const outputPanel = $("output-panel");
  const testList = $("test-list");
  const outputStderr = $("output-stderr");
  const outputDetails = $("output-details");
  const submitBtn = $("submit-btn");
  const actionStatus = $("action-status");
  const formatBtn = $("format-btn");
  const resetBtn = $("reset-btn");
  const copyBtn = $("copy-btn");

  const exerciseKey = opts.slug || section.dataset.exerciseKey || "playground";
  const starter =
    opts.starter != null ? opts.starter : fallback ? fallback.value : "";
  const draftKey = features.draftKey || null;
  const wantsTestResults = features.testResults !== false && !!testList;
  const wantsHighlightOutput = features.syntaxHighlightOutput === true;
  const runWithoutTests = features.runWithoutTests === true;

  const persistDraft = (text) => {
    if (!draftKey) return;
    try {
      if (text === starter) localStorage.removeItem(draftKey);
      else localStorage.setItem(draftKey, text);
    } catch (_) {}
  };

  let savedDraft = null;
  if (draftKey) {
    try {
      savedDraft = localStorage.getItem(draftKey);
    } catch (_) {}
  }
  const initialDoc =
    savedDraft != null && savedDraft !== "" ? savedDraft : starter;
  if (fallback) fallback.value = initialDoc;

  // Default fallback handle (used when CodeMirror fails to load).
  let api = {
    getValue: () => (fallback ? fallback.value : initialDoc),
    setValue: (text) => {
      if (fallback) fallback.value = text;
      persistDraft(text);
    },
    focus: () => fallback && fallback.focus(),
    setVim: (_on) => {},
    destroy: () => {},
  };
  let editor = null;
  let outputView = null;
  let cmModules = null;
  let themeCompartment = null;
  let vimCompartment = null;

  try {
    const wantsVim = !!features.vim;
    const wantsUrlPlugin = features.urlPlugin !== false;
    const importPromises = [
      import("@codemirror/state"),
      import("@codemirror/view"),
      import("@codemirror/language"),
      import("@codemirror/commands"),
      import("@codemirror/lang-rust"),
      import("@codemirror/autocomplete"),
    ];
    if (wantsVim) importPromises.push(import("@replit/codemirror-vim"));
    else importPromises.push(Promise.resolve(null));

    const [state, view, lang, commands, langRust, autocomplete, vimMod] =
      await Promise.all(importPromises);
    cmModules = { state, view, lang, commands, langRust, autocomplete, vimMod };

    const { EditorState, Compartment } = state;
    const {
      EditorView,
      keymap,
      highlightActiveLine,
      highlightActiveLineGutter,
      lineNumbers,
      drawSelection,
      Decoration,
      ViewPlugin,
    } = view;
    const { syntaxHighlighting, bracketMatching, indentOnInput, indentUnit } =
      lang;
    const { defaultKeymap, history, historyKeymap, indentWithTab } = commands;
    const {
      autocompletion,
      completionKeymap,
      closeBrackets,
      closeBracketsKeymap,
    } = autocomplete;

    themeCompartment = new Compartment();
    vimCompartment = new Compartment();

    // ---- URL plugin (middle/left click opens link in a new tab) ---
    let urlPlugin = null;
    let urlTheme = null;
    if (wantsUrlPlugin) {
      const urlMark = Decoration.mark({
        class: "cm-url",
        attributes: { title: "Click to open" },
      });
      const buildUrlDecorations = (viewInstance) => {
        const builder = [];
        for (const { from, to } of viewInstance.visibleRanges) {
          const text = viewInstance.state.doc.sliceString(from, to);
          URL_RE.lastIndex = 0;
          let m;
          while ((m = URL_RE.exec(text))) {
            let end = m.index + m[0].length;
            while (end > m.index && ".,;:!?".includes(text[end - 1])) {
              end -= 1;
            }
            if (end > m.index) {
              builder.push(urlMark.range(from + m.index, from + end));
            }
          }
        }
        return Decoration.set(builder, true);
      };
      urlPlugin = ViewPlugin.fromClass(
        class {
          constructor(v) {
            this.decorations = buildUrlDecorations(v);
          }
          update(u) {
            if (u.docChanged || u.viewportChanged) {
              this.decorations = buildUrlDecorations(u.view);
            }
          }
        },
        {
          decorations: (v) => v.decorations,
          eventHandlers: {
            mousedown(e) {
              if (e.button !== 0 && e.button !== 1) return;
              const target =
                e.target && e.target.closest
                  ? e.target.closest(".cm-url")
                  : null;
              if (!target) return;
              e.preventDefault();
              e.stopPropagation();
              window.open(target.textContent, "_blank", "noopener");
            },
          },
        },
      );
      urlTheme = EditorView.theme({
        ".cm-url": {
          textDecoration: "underline",
          textDecorationColor: "rgba(127,127,127,0.6)",
          textUnderlineOffset: "2px",
          cursor: "pointer",
        },
        ".cm-url:hover": { textDecorationColor: "currentColor" },
      });
    }

    const vimEnabled = wantsVim && vimGlobalEnabled();
    const vimExt = wantsVim
      ? vimCompartment.of(vimEnabled ? vimMod.vim() : [])
      : vimCompartment.of([]);

    let saveTimer = null;
    const persistExt = EditorView.updateListener.of((u) => {
      if (!u.docChanged) return;
      // Any edit invalidates Run result: swap Submit back to Run, and clear
      // any prior "Submitted \u2713" state so the next Run/Submit is fresh.
      resetSubmittedIndicator();
      if (submitBtn && submitBtn.style.display !== "none") {
        submitBtn.style.display = "none";
        if (runBtn) runBtn.style.display = "";
      }
      if (actionStatus && actionStatus.classList.contains("is-visible")) {
        actionStatus.classList.remove(
          "is-visible",
          "is-pass",
          "is-fail",
          "celebrate",
        );
        actionStatus.textContent = "";
      }
      if (draftKey) {
        clearTimeout(saveTimer);
        saveTimer = setTimeout(() => persistDraft(u.state.doc.toString()), 300);
      }
    });

    const extensions = [
      vimExt,
      lineNumbers(),
      highlightActiveLineGutter(),
      highlightActiveLine(),
      history(),
      drawSelection(),
      indentOnInput(),
      // Match Zed / rustfmt: 4-space indent. CM6 defaults to 2, which made
      // newlines inside `fn`/`impl` blocks indent half as far as the
      // surrounding (rustfmt'd) code in the same buffer.
      indentUnit.of("    "),
      bracketMatching(),
      syntaxHighlighting(proseHighlightStyle, { fallback: true }),
      closeBrackets(),
      autocompletion({ override: [buildLocalCompletions()] }),
      keymap.of([
        {
          key: "Mod-Enter",
          run: () => {
            // Mirror clicking Run; debounce/inflight guards live
            // on the click handler so hammering this is safe.
            if (runBtn && !runBtn.disabled) runBtn.click();
            return true;
          },
          preventDefault: true,
        },
        ...closeBracketsKeymap,
        ...defaultKeymap,
        ...historyKeymap,
        ...completionKeymap,
        indentWithTab,
      ]),
      langRust.rust(),
      EditorView.lineWrapping,
      ...(urlPlugin ? [urlPlugin, urlTheme] : []),
      themeCompartment.of(proseEditorTheme),
      persistExt,
    ];

    editor = new EditorView({
      state: EditorState.create({ doc: initialDoc, extensions }),
      parent: mount,
    });
    editor.dom.style.border = "1px solid var(--color-border)";
    editor.dom.style.borderRadius = "12px";
    editor.dom.style.overflow = "hidden";
    // Font size is set in CSS (`.cm-editor { font-size: ... }` in
    // `base.html`) so the `/settings` page can override it via the
    // `html[data-editor-font-size]` attribute.

    if (fallback) fallback.style.display = "none";
    if (mount) mount.style.display = "block";

    api = {
      getValue: () => editor.state.doc.toString(),
      setValue: (text) => {
        editor.dispatch({
          changes: {
            from: 0,
            to: editor.state.doc.length,
            insert: text,
          },
        });
        persistDraft(text);
      },
      focus: () => editor.focus(),
      setVim: (on) => {
        if (!wantsVim) return;
        editor.dispatch({
          effects: vimCompartment.reconfigure(on ? vimMod.vim() : []),
        });
      },
      destroy: () => {
        editor.destroy();
        if (outputView) outputView.destroy();
        activeMounts.delete(api);
      },
    };
  } catch (err) {
    console.warn(
      "[corrode] inline editor: CodeMirror failed to load, using textarea fallback",
      err,
    );
  }

  activeMounts.add(api);

  // ---- Reset ---------------------------------------------------------
  if (resetBtn) {
    resetBtn.addEventListener("click", () => {
      if (!confirm("Replace your edits with the original starter code?"))
        return;
      api.setValue(starter);
      if (draftKey) {
        try {
          localStorage.removeItem(draftKey);
        } catch (_) {}
      }
      // Clear stale output/status from the previous run.
      if (outputPanel) outputPanel.style.display = "none";
      if (runStatus) runStatus.textContent = "";
      setActionStatus("", "neutral");
      api.focus();
    });
  }

  // ---- Copy ----------------------------------------------------------
  if (copyBtn && features.copyButton !== false) {
    const reveal = () => (copyBtn.style.opacity = "1");
    const hide = () => (copyBtn.style.opacity = "");
    copyBtn.addEventListener("mouseenter", reveal);
    copyBtn.addEventListener("mouseleave", hide);
    copyBtn.addEventListener("focus", reveal);
    copyBtn.addEventListener("blur", hide);
    copyBtn.addEventListener("click", async () => {
      try {
        await navigator.clipboard.writeText(api.getValue());
        const prevColor = copyBtn.style.color;
        copyBtn.style.color = "var(--color-success, #2e7d32)";
        copyBtn.style.opacity = "1";
        setTimeout(() => {
          copyBtn.style.color = prevColor;
          copyBtn.style.opacity = "";
        }, 1000);
      } catch (err) {
        console.warn("Clipboard write failed", err);
      }
    });
  } else if (copyBtn && features.copyButton === false) {
    copyBtn.style.display = "none";
  }

  // ---- Read-only CM6 view inside the output panel --------------------
  const renderCargoOutput = (text) => {
    if (!outputStderr) return;
    if (!cmModules || !wantsHighlightOutput) {
      outputStderr.textContent = text;
      return;
    }
    const { state, view, lang, langRust } = cmModules;
    const { EditorState } = state;
    const { EditorView } = view;
    const { syntaxHighlighting } = lang;
    // We always tear down and recreate the read-only output view rather
    // than calling `outputView.dispatch(...)` to replace the document.
    //
    // Reusing the view triggered a viewport-measurement crash inside
    // `@codemirror/state` ("can't access property 'length', l is
    // undefined" via `lineInner` -> `lineAt` -> `getViewport`) on some
    // browsers (notably Firefox ESR) whenever the output panel had just
    // been toggled from `display: none` to `display: block`. The crash
    // only reproduced with DevTools closed, because DevTools forces
    // additional layout passes that paper over the missing measurement.
    // See https://github.com/corrode/course/issues/6.
    //
    // Recreating the view is cheap (it happens at most once per Run /
    // Submit) and follows the same constructor path that already works
    // on the first render.
    if (outputView) {
      outputView.destroy();
      outputView = null;
    }
    outputStderr.textContent = "";
    outputStderr.style.padding = "0";
    outputStderr.style.fontFamily = "";
    outputStderr.style.fontSize = "";
    const exts = [
      EditorView.editable.of(false),
      EditorState.readOnly.of(true),
      EditorView.lineWrapping,
      langRust.rust(),
      syntaxHighlighting(proseHighlightStyle, { fallback: true }),
      proseEditorTheme,
      EditorView.theme({
        "&": {
          backgroundColor: "transparent",
          fontSize: "0.8rem",
        },
        ".cm-content": {
          fontFamily: '"SF Mono", Monaco, monospace',
          padding: "0.85rem 1rem",
        },
        ".cm-scroller": { overflow: "auto" },
        "&.cm-focused": { outline: "none" },
      }),
    ];
    outputView = new EditorView({
      state: EditorState.create({ doc: text, extensions: exts }),
      parent: outputStderr,
    });
  };

  function setActionStatus(text, tone) {
    if (!actionStatus) return;
    actionStatus.classList.remove("is-pass", "is-fail", "celebrate");
    if (!text) {
      actionStatus.textContent = "";
      actionStatus.classList.remove("is-visible");
      return;
    }
    if (tone === "pass" || tone === "fail") {
      actionStatus.textContent = "";
      const icon = document.createElement("span");
      icon.className = tone === "pass" ? "icon-check" : "icon-cross";
      icon.setAttribute("aria-hidden", "true");
      icon.style.marginRight = "0.35em";
      actionStatus.appendChild(icon);
      actionStatus.appendChild(document.createTextNode(text));
    } else {
      actionStatus.textContent = text;
    }
    actionStatus.classList.add("is-visible");
    if (tone === "pass") {
      actionStatus.classList.add("is-pass");
      void actionStatus.offsetWidth;
      actionStatus.classList.add("celebrate");
    } else if (tone === "fail") {
      actionStatus.classList.add("is-fail");
    }
  }
  function clearActionStatus() {
    setActionStatus("", "neutral");
  }

  // ---- Result rendering ---------------------------------------------
  function renderResults(data) {
    if (!outputPanel) return;
    outputPanel.style.display = "block";
    const rawOutput = (data.stderr || "") + "\n" + (data.stdout || "");

    if (wantsTestResults) {
      testList.innerHTML = "";
      const failureSnippets = extractFailureSnippets(rawOutput);
      for (const t of data.test_results) {
        const li = document.createElement("li");
        li.style.padding = "0.35rem 0.5rem";
        li.style.fontFamily = "'SF Mono', Monaco, monospace";
        li.style.fontSize = "0.85rem";
        li.style.display = "flex";
        li.style.flexDirection = "column";
        li.style.gap = "0.25rem";
        const row = document.createElement("div");
        row.style.display = "flex";
        row.style.alignItems = "center";
        row.style.gap = "0.5rem";
        const icon = document.createElement("span");
        if (t.passed) {
          icon.innerHTML =
            '<span class="icon-check" aria-hidden="true"></span>';
          icon.style.color = "var(--color-success, #2e7d32)";
          icon.style.fontWeight = "700";
          icon.style.display = "inline-flex";
          icon.style.alignItems = "center";
        } else {
          icon.innerHTML =
            '<span class="icon-cross" aria-hidden="true"></span>';
          icon.style.color = "var(--color-error, #c62828)";
          icon.style.fontWeight = "700";
          icon.style.display = "inline-flex";
          icon.style.alignItems = "center";
        }
        const name = document.createElement("span");
        name.textContent = t.name;
        row.appendChild(icon);
        row.appendChild(name);
        li.appendChild(row);
        if (!t.passed) {
          const snippet = friendlyFailureSnippet(failureSnippets[t.name]);
          if (snippet) {
            const detail = document.createElement("div");
            detail.style.marginLeft = "1.4rem";
            detail.style.padding = "0.4rem 0.6rem";
            detail.style.background = "var(--color-surface)";
            detail.style.border = "1px solid var(--color-border)";
            detail.style.borderRadius = "6px";
            detail.style.color = "var(--color-text)";
            detail.style.whiteSpace = "pre-wrap";
            detail.style.fontSize = "0.8rem";
            detail.textContent = snippet;
            li.appendChild(detail);
          }
        }
        testList.appendChild(li);
      }
      const failed = data.test_results.some((t) => !t.passed);
      const compileFailed = !data.success && data.test_results.length === 0;
      if (outputDetails) outputDetails.open = failed || compileFailed;
    }

    // Output rendering: if syntax highlight is on we use the read-only
    // CM6 view; otherwise the panel may use a plain <pre> so we fall
    // through to its `textContent`.
    if (outputStderr) {
      if (wantsTestResults || wantsHighlightOutput) {
        const raw = data.stderr || data.stdout || "(no output)";
        renderCargoOutput(raw);
      } else {
        const out = data.stdout || "";
        const errStr = data.stderr || "";
        outputStderr.textContent =
          out + (out && errStr ? "\n\n" : "") + errStr || "(no output)";
      }
    }
    outputPanel.scrollIntoView({ behavior: "smooth", block: "nearest" });
  }

  function updateRunStatus(data) {
    const passed = data.test_results.filter((t) => t.passed).length;
    const total = data.test_results.length;
    const succeeded = data.success && (total === 0 || passed === total);

    if (runStatus) {
      if (total === 0) {
        if (runWithoutTests) {
          runStatus.textContent = data.success ? "Ran." : "Did not compile.";
        } else {
          runStatus.textContent = data.success
            ? "Compiled. No tests ran."
            : "Did not compile.";
        }
        runStatus.style.color = data.success
          ? "var(--color-text-muted)"
          : "var(--color-error, #c62828)";
      } else if (passed === total) {
        runStatus.textContent = `All ${total} tests passed.`;
        runStatus.style.color = "var(--color-success, #2e7d32)";
      } else {
        runStatus.textContent = `${passed} / ${total} tests passed.`;
        runStatus.style.color = "var(--color-error, #c62828)";
      }
    }

    if (total === 0) {
      setActionStatus(
        data.success
          ? runWithoutTests
            ? "Ran"
            : "Compiled"
          : "Did not compile",
        data.success ? "neutral" : "fail",
      );
    } else if (passed === total) {
      setActionStatus(
        total === 1 ? "1 test passed" : `${total} tests passed`,
        "pass",
      );
    } else {
      const failed = total - passed;
      setActionStatus(
        failed === 1
          ? `1 test failed (${passed}/${total})`
          : `${failed} tests failed (${passed}/${total})`,
        "fail",
      );
    }

    if (succeeded && typeof opts.onRunSuccess === "function") {
      try {
        opts.onRunSuccess({ section, data });
      } catch (err) {
        console.error("[corrode] onRunSuccess failed:", err);
      }
    }
  }

  // ---- Run via /api/run ---------------------------------------------
  let lastRunAt = 0;
  let inFlight = false;
  async function runOnce(code) {
    if (runStatus) {
      runStatus.textContent = "Running on play.rust-lang.org…";
      runStatus.style.color = "var(--color-text-muted)";
    }
    if (runSpinner) runSpinner.style.display = "inline-block";
    clearActionStatus();
    try {
      const payload = { code, slug: exerciseKey };
      if (runWithoutTests) payload.tests = false;
      const resp = await fetch("/api/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      if (!resp.ok) {
        if (runStatus) {
          if (resp.status === 429) {
            runStatus.textContent =
              "Playground rate-limited us. Try again in a moment.";
          } else if (resp.status === 502) {
            runStatus.textContent =
              "Playground unreachable. Check your connection or try again.";
          } else {
            runStatus.textContent = `Run failed (HTTP ${resp.status}).`;
          }
          runStatus.style.color = "var(--color-error, #c62828)";
        }
        return null;
      }
      const data = await resp.json();
      renderResults(data);
      return data;
    } catch (err) {
      console.error(err);
      if (runStatus) {
        runStatus.textContent = "Run failed: " + err.message;
        runStatus.style.color = "var(--color-error, #c62828)";
      }
      return null;
    } finally {
      if (runSpinner) runSpinner.style.display = "none";
    }
  }

  function showSubmit() {
    if (!submitBtn || !runBtn) return;
    runBtn.style.display = "none";
    submitBtn.style.display = "";
    submitBtn.focus();
  }
  function showRun() {
    if (!submitBtn || !runBtn) return;
    runBtn.style.display = "";
    submitBtn.style.display = "none";
  }
  // Remember the submit button's initial label so we can restore it after
  // the editor is edited again (the persistExt updateListener flips Submit
  // back to Run on any change; we mirror that for the "Submitted" state).
  const submitDefaultLabel = submitBtn ? submitBtn.innerHTML : "";
  function markSubmittedIndicator() {
    if (!submitBtn) return;
    submitBtn.innerHTML =
      'Submitted <span class="icon-check" aria-hidden="true"></span>';
    submitBtn.classList.add("is-submitted");
    submitBtn.disabled = true;
    submitBtn.style.display = "";
    if (runBtn) runBtn.style.display = "none";
  }
  function resetSubmittedIndicator() {
    if (!submitBtn) return;
    if (submitBtn.classList.contains("is-submitted")) {
      submitBtn.classList.remove("is-submitted");
      submitBtn.innerHTML = submitDefaultLabel;
      submitBtn.disabled = false;
    }
  }
  // Filled in below if Submit is enabled. Called by the Run handler to
  // autosubmit when every test passes.
  let autosubmit = null;

  // Persist the "Submitted ✓" state across reloads: the server marks
  // the section with `data-completed="true"` when this step's most
  // recent submission has all tests passing. Any subsequent edit clears
  // it via the `persistExt` updateListener.
  if (submitBtn && section.dataset.completed === "true") {
    markSubmittedIndicator();
  }

  if (runBtn) {
    runBtn.addEventListener("click", async () => {
      const now = Date.now();
      if (inFlight || now - lastRunAt < 750) return;
      lastRunAt = now;
      inFlight = true;
      runBtn.disabled = true;
      try {
        const code = api.getValue();
        const data = await runOnce(code);
        if (!data) return;
        updateRunStatus(data);
        const total = data.test_results.length;
        const passed = data.test_results.filter((t) => t.passed).length;
        const allPassed = total > 0 && passed === total;
        if (submitBtn && features.submit) {
          if (allPassed) {
            // Autosubmit: when every test passes we don't make the user
            // press a second button. Submit in the background and show
            // "Submitted \u2713" as the post-run confirmation. The Submit
            // button stays in the DOM as the indicator. If autosubmit
            // isn't possible (e.g. no participant ulid) we fall back to
            // the previous two-step Run → Submit flow.
            if (typeof autosubmit === "function") {
              await autosubmit(code, passed, total);
            } else {
              showSubmit();
              if (runStatus) {
                runStatus.textContent +=
                  ". Click Submit to save your progress.";
              }
            }
          } else {
            showRun();
          }
        }
      } finally {
        inFlight = false;
        setTimeout(() => {
          runBtn.disabled = false;
        }, 250);
      }
    });
  }

  // ---- Format via /api/format ---------------------------------------
  if (formatBtn) {
    let formatInFlight = false;
    formatBtn.addEventListener("click", async () => {
      if (formatInFlight) return;
      formatInFlight = true;
      formatBtn.disabled = true;
      if (runStatus) {
        runStatus.textContent = "Formatting…";
        runStatus.style.color = "var(--color-text-muted)";
      }
      try {
        const resp = await fetch("/api/format", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            code: api.getValue(),
            slug: exerciseKey,
          }),
        });
        if (!resp.ok) {
          if (runStatus) {
            runStatus.textContent =
              resp.status === 429
                ? "Formatter rate-limited; try again in a moment."
                : resp.status === 502
                  ? "Formatter unreachable."
                  : `Format failed (HTTP ${resp.status}).`;
            runStatus.style.color = "var(--color-error, #c62828)";
          }
          return;
        }
        const data = await resp.json();
        if (!data.success) {
          if (runStatus) {
            runStatus.textContent =
              "rustfmt couldn't parse the code. Fix the syntax error first.";
            runStatus.style.color = "var(--color-error, #c62828)";
          }
          return;
        }
        if (data.code && data.code !== api.getValue()) {
          api.setValue(data.code);
          if (runStatus) {
            runStatus.textContent = "Formatted.";
            runStatus.style.color = "var(--color-success, #2e7d32)";
          }
        } else if (runStatus) {
          runStatus.textContent = "Already formatted.";
          runStatus.style.color = "var(--color-text-muted)";
        }
      } catch (e) {
        console.error(e);
        if (runStatus) {
          runStatus.textContent = "Format failed: " + e.message;
          runStatus.style.color = "var(--color-error, #c62828)";
        }
      } finally {
        formatInFlight = false;
        setTimeout(() => {
          formatBtn.disabled = false;
        }, 250);
      }
    });
  }

  // ---- Submit -------------------------------------------------------
  function markAsCompleted() {
    const meta = document.getElementById("exercise-meta");
    if (meta) {
      const hasPassed = meta.querySelector(
        ".status-perfected, .status-completed",
      );
      if (!hasPassed) {
        meta.innerHTML =
          '<span class="status-badge status-completed" title="Tests passed"><span class="icon-check"></span> Passed</span>';
      }
    }
    const row = document.getElementById("current-chapter-row");
    if (row && !row.classList.contains("completed")) {
      row.classList.add("attempted", "completed");
    }
  }

  // Update the top-bar `progress_done / progress_total` counter in place
  // so learners see their chapter count tick up the instant the last
  // exercise's submission lands, without waiting for the next page nav.
  // `data` is the JSON body returned by `POST /api/submit`.
  function applyProgressUpdate(data) {
    if (!data || typeof data.progress_done !== "number") return;
    const wrapper = document.querySelector(".topbar-progress");
    if (!wrapper) return;
    const strong = wrapper.querySelector(".topbar-progress-count strong");
    if (strong) {
      strong.textContent = String(data.progress_done);
    }
    if (typeof data.progress_total === "number") {
      wrapper.setAttribute(
        "aria-label",
        `Progress: ${data.progress_done} of ${data.progress_total} chapters completed`,
      );
    }
    // Note: the topbar's hairline tracks reading-scroll position, not
    // chapter completion, so we don't touch `.topbar-progress-bar` here.
  }

  if (submitBtn && features.submit) {
    const submitInfo = features.submit;
    async function submitOnce(code, passed, total) {
      const ulid = (submitInfo && submitInfo.ulid) || submitBtn.dataset.ulid;
      const exKey = (submitInfo && submitInfo.exerciseKey) || exerciseKey;
      try {
        const submitResp = await fetch("/api/submit", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            ulid,
            exercise_name: exKey,
            source_code: code,
            tests_passed: true,
            clippy_passed: false,
            fmt_passed: false,
          }),
        });
        if (submitResp.ok) {
          let respData = null;
          try {
            respData = await submitResp.json();
          } catch (_) {
            // Older server builds may not return a JSON body; that's fine,
            // we'll just skip the progress-counter refresh.
          }
          if (runStatus) {
            runStatus.textContent = `Saved progress (${passed}/${total} tests passed).`;
            runStatus.style.color = "var(--color-success, #2e7d32)";
          }
          markAsCompleted();
          markSubmittedIndicator();
          applyProgressUpdate(respData);
          if (typeof opts.onSubmit === "function") {
            try {
              opts.onSubmit({ section, ulid, exerciseKey: exKey });
            } catch (err) {
              console.error("[corrode] onSubmit failed:", err);
            }
          }
          return true;
        } else if (runStatus) {
          if (submitResp.status === 401) {
            runStatus.textContent =
              "Submit rejected: your session is unknown. Try registering again.";
          } else {
            runStatus.textContent = `Submit failed (HTTP ${submitResp.status}).`;
          }
          runStatus.style.color = "var(--color-error, #c62828)";
        }
      } catch (err) {
        console.error(err);
        if (runStatus) {
          runStatus.textContent = "Submit failed: " + err.message;
          runStatus.style.color = "var(--color-error, #c62828)";
        }
      }
      return false;
    }

    // Expose for the Run handler's autosubmit-on-all-green path. Only
    // active when we actually have a ulid; otherwise the run handler
    // falls back to the two-step Run → Submit flow.
    autosubmit = async (code, passed, total) => {
      const ulid = (submitInfo && submitInfo.ulid) || submitBtn.dataset.ulid;
      if (!ulid) {
        showSubmit();
        if (runStatus) {
          runStatus.textContent += ". Click Submit to save your progress.";
        }
        return;
      }
      submitBtn.disabled = true;
      if (runBtn) runBtn.disabled = true;
      try {
        await submitOnce(code, passed, total);
      } finally {
        // submitOnce flips the button into the "Submitted" state on
        // success; on failure we leave Submit clickable so the user
        // can retry without re-running.
        if (!submitBtn.classList.contains("is-submitted")) {
          submitBtn.disabled = false;
          showSubmit();
        }
        if (runBtn) runBtn.disabled = false;
      }
    };

    submitBtn.addEventListener("click", async () => {
      submitBtn.disabled = true;
      if (runBtn) runBtn.disabled = true;
      try {
        const code = api.getValue();
        const data = await runOnce(code);
        if (!data) return;
        updateRunStatus(data);
        const total = data.test_results.length;
        const passed = data.test_results.filter((t) => t.passed).length;
        const tests_passed = total > 0 && passed === total;
        if (!tests_passed) {
          if (runStatus) {
            runStatus.textContent +=
              ". Not saving progress (tests must pass first).";
            runStatus.style.color = "var(--color-error, #c62828)";
          }
          return;
        }
        await submitOnce(code, passed, total);
      } finally {
        submitBtn.disabled = false;
        if (runBtn) runBtn.disabled = false;
      }
    });
  }

  return api;
}
