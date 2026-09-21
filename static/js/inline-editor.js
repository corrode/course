// Shared CodeMirror mount, with a functional textarea fallback when loading
// fails.
import { proseHighlightStyle, proseEditorTheme } from "./cm-theme.js";
import { rustIndentUnit } from "./rust-indent.js";
import {
  buildLocalCompletions,
  createUrlExtensions,
} from "./editor-extensions.js";
import {
  createWorkflow,
  initialDocument,
  minimalChange,
  formatChanges,
} from "./editor-workflow.js";
import { createResultRenderer, updateProgress } from "./editor-results.js";
import { createController } from "./editor-controller.js";

const VIM_PREF_KEY = "corrode:editor:vim";
const activeMounts = new Set();
const mounts = new WeakMap();
let nextEditorId = 0;

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
  for (const mount of activeMounts) mount.setVim(enabled);
}

// Cache the promise as well as the eventual handle: concurrent mounts must not
// construct duplicate editors or register duplicate network handlers.
export function mountInlineEditor(section, opts = {}) {
  if (mounts.has(section)) return mounts.get(section);
  const promise = createMount(section, opts);
  mounts.set(section, promise);
  promise.catch(() => {
    if (mounts.get(section) === promise) mounts.delete(section);
  });
  return promise;
}

async function createMount(section, opts) {
  const features = opts.features || {};
  const $ = (role) => section.querySelector(`[data-role="${role}"]`);
  const fallback = $("editor-fallback");
  const mount = $("editor-mount");
  const runBtn = $("run-btn");
  const submitBtn = $("submit-btn");
  const formatBtn = $("format-btn");
  const resetBtn = $("reset-btn");
  const copyBtn = $("copy-btn");
  const runStatus = $("run-status");
  const actionStatus = $("action-status");
  const spinner = $("run-spinner");
  const panel = $("output-panel");
  const document = section.ownerDocument;
  const window = document.defaultView;
  const exerciseKey = opts.slug || section.dataset.exerciseKey || "playground";
  const starter = opts.starter ?? fallback?.value ?? "";
  const submitted = opts.submitted ?? null;
  const draftKey = features.draftKey || null;
  let draft = null;
  try {
    if (draftKey) draft = localStorage.getItem(draftKey);
  } catch (_) {}
  let value = initialDocument(draft, submitted, starter);
  if (fallback) fallback.value = value;
  let editor = null;
  let replaceEditorValue = null;
  let setEditorVim = () => {};
  let destroyed = false;
  let workflow;
  const submitLabel = submitBtn?.textContent || "Submit";
  const submitInfo = features.submit;
  const ulid = () => submitInfo?.ulid || submitBtn?.dataset.ulid;
  const getValue = () =>
    editor ? editor.state.doc.toString() : fallback ? fallback.value : value;
  const controller = createController({
    window,
    document,
    read: getValue,
    persist(text) {
      if (!draftKey) return;
      // Keep even an empty or starter-valued draft: deleting it would resurrect
      // an older server submission on the next mount.
      try {
        localStorage.setItem(draftKey, text);
      } catch (_) {}
    },
  });
  const listen = controller.listen;
  function edited() {
    workflow?.edit();
    controller.schedule();
    render.invalidate();
  }
  const api = {
    getValue,
    setValue(text) {
      if (destroyed || text === getValue()) return;
      if (editor) replaceEditorValue(text);
      else {
        const before = getValue();
        value = text;
        if (fallback) {
          const change = minimalChange(before, text);
          fallback.setRangeText(
            change.insert,
            change.from,
            change.to,
            "preserve",
          );
        }
        edited();
      }
    },
    focus() {
      if (!destroyed) editor ? editor.focus() : fallback?.focus();
    },
    setVim(on) {
      if (!destroyed) setEditorVim(on);
    },
    destroy() {
      if (destroyed) return;
      value = getValue();
      controller.destroy();
      destroyed = true;
      workflow.destroy();
      render.destroy();
      editor?.destroy();
      editor = null;
      if (fallback) {
        fallback.value = value;
        fallback.style.display = "";
      }
      if (mount) mount.style.display = "none";
      hint.remove();
      copyFeedback.remove();
      if (fallback) {
        if (originalDescription === null)
          fallback.removeAttribute("aria-describedby");
        else fallback.setAttribute("aria-describedby", originalDescription);
      }
      if (spinner) spinner.style.display = "none";
      if (copyBtn) copyBtn.disabled = false;
      activeMounts.delete(api);
      mounts.delete(section);
    },
  };

  const label = opts.editorLabel || `Rust code editor: ${exerciseKey}`;
  const hint = document.createElement("span");
  hint.id = `editor-shortcuts-${++nextEditorId}`;
  hint.textContent =
    (runBtn ? "Run: Ctrl+Enter or Command+Enter. " : "") +
    "In the code editor, press Escape then Tab to move focus out.";
  hint.className = "editor-shortcuts";
  (mount?.parentNode || section).append(hint);
  const originalDescription =
    fallback?.getAttribute("aria-describedby") ?? null;
  const description = [originalDescription, hint.id].filter(Boolean).join(" ");
  if (fallback) {
    fallback.setAttribute("aria-label", label);
    fallback.setAttribute("aria-describedby", description);
    listen(fallback, "input", edited);
    listen(fallback, "keydown", (event) => {
      if (runBtn && event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
        event.preventDefault();
        workflow.run();
      }
    });
  }
  const liveStatus = runStatus || actionStatus;
  liveStatus?.setAttribute("role", "status");
  liveStatus?.setAttribute("aria-live", "polite");
  if (runStatus && actionStatus) {
    actionStatus.removeAttribute("role");
    actionStatus.removeAttribute("aria-live");
    actionStatus.setAttribute("aria-hidden", "true");
  }
  const copyFeedback = document.createElement("span");
  copyFeedback.setAttribute("role", "status");
  copyFeedback.setAttribute("aria-live", "polite");
  copyFeedback.className = "editor-copy-feedback";
  (copyBtn?.parentNode || section).append(copyFeedback);
  const render = createResultRenderer({
    panel,
    list: $("test-list"),
    output: $("output-stderr"),
    details: $("output-details"),
    testResults: features.testResults !== false,
  });
  render.clear();
  workflow = createWorkflow({
    getValue,
    setValue: api.setValue,
    submitted,
    submittedPassed: opts.submittedPassed === true,

    runWithoutTests: features.runWithoutTests === true,
    autoSubmit: !!(submitBtn && submitInfo && ulid()),
    submitPayload:
      submitInfo && submitBtn
        ? (code) => ({
            ulid: ulid(),
            exercise_name: submitInfo.exerciseKey || exerciseKey,
            source_code: code,
            tests_passed: true,
            clippy_passed: false,
            fmt_passed: false,
          })
        : null,
    async request(kind, payload, signal) {
      if (kind !== "submit") payload = { ...payload, slug: exerciseKey };
      if (kind === "run") {
        if (features.runWithoutTests) payload.tests = false;
        if (features.analytics) {
          payload.participant_id = features.analytics.participantId;
          payload.session_id = features.analytics.sessionId;
        }
      }
      const action = { run: "Run", format: "Format", submit: "Submit" }[kind];
      let response;
      try {
        response = await fetch(`/api/${kind}`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(payload),
          signal,
        });
      } catch (error) {
        if (error.name === "AbortError") throw error;
        throw new Error(
          `${action} failed: ${error.message || "Network error"}. Check your connection and try again.`,
        );
      }
      if (!response.ok) {
        if (response.status === 401) {
          throw new Error(
            `${action} rejected: your session is unknown or expired. Try registering again.`,
          );
        }
        if (response.status === 429) {
          throw new Error(`${action} rate-limited. Try again in a moment.`);
        }
        if (response.status === 502) {
          const service =
            kind === "submit"
              ? "Progress service"
              : kind === "format"
                ? "Formatter"
                : "Playground";
          throw new Error(
            `${service} unreachable. Check your connection or try again.`,
          );
        }
        throw new Error(
          `${action} failed (HTTP ${response.status}). Try again.`,
        );
      }
      if (kind === "submit") {
        try {
          return await response.json();
        } catch (_) {
          return null;
        }
      }
      return response.json();
    },
    changed({ busy, saved, canSubmit, message, tone }) {
      if (runBtn) {
        runBtn.style.display = "";
        runBtn.disabled = !!busy;
      }
      if (formatBtn) formatBtn.disabled = !!busy;
      if (spinner)
        spinner.style.display = busy === "run" ? "inline-block" : "none";
      if (submitBtn) {
        submitBtn.style.display =
          submitInfo && (saved || canSubmit) ? "" : "none";
        submitBtn.disabled = !!busy || saved;
        submitBtn.textContent = saved ? "Submitted ✓" : submitLabel;
        submitBtn.classList.toggle("is-submitted", saved);
      }
      if (runStatus) {
        runStatus.textContent = message;
        runStatus.style.color =
          tone === "fail"
            ? "var(--color-error)"
            : tone === "pass"
              ? "var(--color-success)"
              : "var(--color-text-muted)";
      }
      if (actionStatus) {
        actionStatus.textContent = !runStatus
          ? message
          : !message
            ? ""
            : busy
              ? { run: "Running…", format: "Formatting…", submit: "Saving…" }[
                  busy
                ]
              : saved
                ? "Saved"
                : tone === "pass"
                  ? "Done"
                  : tone === "fail"
                    ? "Failed"
                    : "";
        actionStatus.classList.toggle("is-visible", !!message);
        actionStatus.classList.toggle("is-pass", tone === "pass");
        actionStatus.classList.toggle("is-fail", tone === "fail");
        actionStatus.classList.remove("celebrate");
      }
    },
    render(data) {
      render(data);
      let scrollOutput = true;
      try {
        scrollOutput = localStorage.getItem("corrode:editor:scroll-output") !== "0";
      } catch (_) {}
      if (scrollOutput) {
        const reducedMotion = window.matchMedia?.(
          "(prefers-reduced-motion: reduce)",
        ).matches;
        panel?.scrollIntoView?.({
          behavior: reducedMotion ? "instant" : "smooth",
          block: "start",
          inline: "nearest",
        });
      }
    },
    onRunSuccess: (data) => opts.onRunSuccess?.({ section, data }),
    onSubmit({ result, payload }) {
      updateProgress(document, result);
      opts.onSubmit?.({
        section,
        ulid: payload.ulid,
        exerciseKey: payload.exercise_name,
      });
    },
  });
  listen(runBtn, "click", () => workflow.run());
  listen(formatBtn, "click", () => workflow.format());
  if (submitInfo) listen(submitBtn, "click", () => workflow.submit());
  listen(resetBtn, "click", () => {
    if (!window.confirm("Replace your edits with the original starter code?"))
      return;
    api.setValue(starter);
    render.clear();
    controller.flush();
    api.focus();
  });
  if (copyBtn && features.copyButton !== false) {
    const reveal = () => {
      copyBtn.style.opacity = "1";
    };
    const hide = () => {
      copyBtn.style.opacity = "";
    };
    listen(copyBtn, "mouseenter", reveal);
    listen(copyBtn, "mouseleave", hide);
    listen(copyBtn, "focus", reveal);
    listen(copyBtn, "blur", hide);
    listen(copyBtn, "click", async () => {
      if (copyBtn.disabled) return;
      copyBtn.disabled = true;
      copyFeedback.textContent = "Copying…";
      try {
        await window.navigator.clipboard.writeText(getValue());
        if (!destroyed) copyFeedback.textContent = "Code copied.";
      } catch (_) {
        if (!destroyed)
          copyFeedback.textContent =
            "Could not copy. Select the code and copy it manually.";
      } finally {
        if (!destroyed) copyBtn.disabled = false;
      }
    });
  } else if (copyBtn) copyBtn.style.display = "none";

  try {
    if (!mount) throw new Error("Missing editor mount");
    const wantsVim = !!features.vim;
    const [state, view, lang, commands, langRust, autocomplete, vimMod] =
      await Promise.all([
        import("@codemirror/state"),
        import("@codemirror/view"),
        import("@codemirror/language"),
        import("@codemirror/commands"),
        import("@codemirror/lang-rust"),
        import("@codemirror/autocomplete"),
        wantsVim ? import("@replit/codemirror-vim") : Promise.resolve(null),
      ]);
    const cmModules = {
      state,
      view,
      lang,
      commands,
      langRust,
      autocomplete,
      vimMod,
    };
    const { EditorState, Compartment, Transaction } = state;
    const {
      EditorView,
      keymap,
      highlightActiveLine,
      highlightActiveLineGutter,
      lineNumbers,
      drawSelection,
      Decoration,
      ViewPlugin,
      MatchDecorator,
    } = view;
    const { syntaxHighlighting, bracketMatching, indentOnInput, indentUnit } =
      lang;
    const {
      defaultKeymap,
      history,
      historyKeymap,
      indentWithTab,
      isolateHistory,
    } = commands;
    const {
      autocompletion,
      completionKeymap,
      closeBrackets,
      closeBracketsKeymap,
      acceptCompletion,
      completionStatus,
    } = autocomplete;
    const vimCompartment = new Compartment();
    const extensions = [
      vimCompartment.of(wantsVim && vimGlobalEnabled() ? vimMod.vim() : []),
      lineNumbers(),
      highlightActiveLineGutter(),
      highlightActiveLine(),
      history(),
      drawSelection(),
      indentOnInput(),
      indentUnit.of(rustIndentUnit),
      EditorState.tabSize.of(rustIndentUnit.length),
      bracketMatching(),
      syntaxHighlighting(proseHighlightStyle, { fallback: true }),
      closeBrackets(),
      autocompletion({ override: [buildLocalCompletions()] }),
      keymap.of([
        {
          key: "Mod-Enter",
          run: () => {
            if (runBtn) workflow.run();
            return true;
          },
          preventDefault: true,
        },
        ...closeBracketsKeymap,
        ...defaultKeymap,
        ...historyKeymap,
        ...completionKeymap,
        {
          key: "Tab",
          run: (v) =>
            completionStatus(v.state) === "active" && acceptCompletion(v),
        },
        indentWithTab,
      ]),
      langRust.rust(),
      EditorView.lineWrapping,
      proseEditorTheme,
      features.urlPlugin !== false
        ? createUrlExtensions({
            EditorView,
            Decoration,
            ViewPlugin,
            MatchDecorator,
          })
        : [],
      // Vim consumes Escape before CodeMirror's default handler can enable
      // this.
      EditorView.domEventObservers({
        keydown(event, editor) {
          if (event.key === "Escape") editor.setTabFocusMode(2000);
        },
      }),
      EditorView.editorAttributes.of({ class: "inline-rust-editor" }),
      EditorView.contentAttributes.of({
        "aria-label": label,
        "aria-describedby": description,
      }),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) edited();
      }),
      typeof features.buildExtraExtensions === "function"
        ? features.buildExtraExtensions(cmModules) || []
        : [],
    ];
    // Read at installation time, not before the imports: the fallback is live
    // and the learner may already have typed while the chunks were loading.
    const doc = getValue();
    editor = new EditorView({
      state: EditorState.create({ doc, extensions }),
      parent: mount,
    });

    replaceEditorValue = (text) =>
      editor.dispatch({
        changes: formatChanges(editor.state.doc.toString(), text),
        annotations: [
          Transaction.userEvent.of("input.format"),
          isolateHistory.of("full"),
        ],
      });
    setEditorVim = (on) => {
      if (wantsVim)
        editor.dispatch({
          effects: vimCompartment.reconfigure(on ? vimMod.vim() : []),
        });
    };
    if (fallback) fallback.style.display = "none";
    mount.style.display = "block";
  } catch (error) {
    if (editor && fallback) fallback.value = editor.state.doc.toString();
    editor?.destroy();
    editor = null;
    if (fallback) fallback.style.display = "";
    if (mount) mount.style.display = "none";
    console.warn(
      "[corrode] CodeMirror unavailable; using textarea fallback",
      error,
    );
  }
  activeMounts.add(api);
  return api;
}
