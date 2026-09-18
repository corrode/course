// A result belongs to a revision, not merely to equal source text (edit/undo
// while a request is pending must not make that request current again).
export function submissionPassed(data) {
  return (
    data?.success === true &&
    Array.isArray(data.test_results) &&
    data.test_results.length > 0 &&
    data.test_results.every((test) => test.passed === true)
  );
}

export function initialDocument(draft, submitted, starter) {
  return draft ?? submitted ?? starter;
}

export function minimalChange(before, after) {
  let from = 0;
  while (
    from < before.length &&
    from < after.length &&
    before[from] === after[from]
  )
    from++;
  let to = before.length;
  let end = after.length;
  while (to > from && end > from && before[to - 1] === after[end - 1]) {
    to--;
    end--;
  }
  return { from, to, insert: after.slice(from, end) };
}

// Keep unchanged code between whitespace edits intact so CodeMirror can map
// selections through reindentation and line wrapping. Non-whitespace changes
// use one conservative suffix replacement rather than a general diff algorithm.
export function formatChanges(before, after) {
  const changes = [];
  let from = 0;
  let target = 0;
  const whitespace = (text, index) =>
    index < text.length && /\s/.test(text[index]);

  while (from < before.length || target < after.length) {
    if (from < before.length && before[from] === after[target]) {
      from++;
      target++;
      continue;
    }
    if (whitespace(before, from) || whitespace(after, target)) {
      const start = from;
      const targetStart = target;
      while (whitespace(before, from)) from++;
      while (whitespace(after, target)) target++;
      changes.push({
        from: start,
        to: from,
        insert: after.slice(targetStart, target),
      });
      continue;
    }
    const suffix = minimalChange(before.slice(from), after.slice(target));
    changes.push({
      from: from + suffix.from,
      to: from + suffix.to,
      insert: suffix.insert,
    });
    break;
  }
  return changes;
}

export function createWorkflow({
  getValue,
  setValue,
  request,
  submitPayload,
  autoSubmit,
  submitted = null,
  submittedPassed = false,
  runWithoutTests = false,
  formatSource = (text) => text,
  changed = () => {},
  render = () => {},
  onRunSuccess = () => {},
  onSubmit = () => {},
}) {
  let revision = 0;
  let destroyed = false;
  let operation = null;
  let passing = null;
  let saved = submittedPassed && submitted !== null && getValue() === submitted;
  let message = "";
  let tone = "neutral";
  const notify = () => {
    if (!destroyed)
      changed({
        busy: operation?.kind ?? null,
        saved,
        canSubmit: !!passing,
        message,
        tone,
      });
  };
  const status = (text, kind = "neutral") => {
    message = text;
    tone = kind;
    notify();
  };
  const current = (op) =>
    !destroyed && operation === op && op.revision === revision;
  const callback = (fn, value) => {
    try {
      fn(value);
    } catch (error) {
      console.error("Editor callback failed", error);
    }
  };
  function edit() {
    if (destroyed) return;
    revision++;
    passing = null;
    saved = false;
    // Release ownership before aborting. A transport may ignore cancellation
    // indefinitely; its response and finally block must not own the next run.
    const cancelled = operation;
    operation = null;
    cancelled?.controller.abort();
    status("");
  }
  async function save(op, snapshot) {
    if (!current(op) || !submissionPassed(snapshot.data)) return;
    const payload = submitPayload?.(snapshot.code);
    if (!payload?.ulid) {
      status("Register your session before saving progress.", "fail");
      return;
    }
    op.kind = "submit";
    status("Saving progress…");
    const result = await request("submit", payload, op.controller.signal);
    if (!current(op)) return;
    saved = true;
    status("Saved progress.", "pass");
    if (current(op)) callback(onSubmit, { result, payload });
  }
  async function perform(kind) {
    if (destroyed || operation || (kind === "submit" && saved)) return;
    const op = { kind, revision, controller: new AbortController() };
    operation = op;
    const code = getValue();
    status(
      kind === "format"
        ? "Formatting…"
        : kind === "submit" && passing
          ? "Saving progress…"
          : "Running on play.rust-lang.org…",
    );
    try {
      if (kind === "format") {
        const data = await request("format", { code }, op.controller.signal);
        if (!current(op)) return;
        if (!data.success || typeof data.code !== "string")
          throw new Error(
            "rustfmt couldn't parse the code. Fix the syntax error first.",
          );
        const formatted = formatSource(data.code);
        if (formatted !== code) {
          // setValue synchronously emits edit(), invalidating old results.
          setValue(formatted);
          if (
            !destroyed &&
            operation === null &&
            revision === op.revision + 1
          ) {
            status("Formatted.", "pass");
          }
        } else status("Already formatted.");
        return;
      }
      let snapshot = kind === "submit" ? passing : null;
      if (!snapshot) {
        passing = null;
        const data = await request("run", { code }, op.controller.signal);
        if (!current(op)) return;
        snapshot = { code, data, revision };
        if (submissionPassed(data)) passing = snapshot;
        render(data);
        if (!current(op)) return;
        const tests = Array.isArray(data.test_results) ? data.test_results : [];
        const passed = tests.filter((test) => test.passed === true).length;
        const success =
          submissionPassed(data) ||
          (data.success === true && tests.length === 0);
        let summary;
        if (!data.success) {
          summary = "Run failed.";
        } else if (tests.length) {
          summary = `${passed} / ${tests.length} tests passed.`;
        } else {
          summary = runWithoutTests
            ? "Ran successfully."
            : "Compiled. No tests ran.";
        }
        status(summary, success ? "pass" : "fail");
        if (success && current(op)) callback(onRunSuccess, data);
      }
      if (current(op) && passing && !saved && (kind === "submit" || autoSubmit))
        await save(op, snapshot);
    } catch (error) {
      if (current(op) && error.name !== "AbortError")
        status(error.message || `${kind} failed.`, "fail");
    } finally {
      if (operation === op) {
        operation = null;
        notify();
      }
    }
  }
  notify();
  return {
    edit,
    run: () => perform("run"),
    format: () => perform("format"),
    submit: () => perform("submit"),
    destroy() {
      destroyed = true;
      const cancelled = operation;
      operation = null;
      passing = null;
      cancelled?.controller.abort();
    },
  };
}
