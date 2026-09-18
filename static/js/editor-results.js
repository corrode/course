import { submissionPassed } from "./editor-workflow.js";
import { highlightOutput } from "./output-highlight.js";

function failureSnippets(raw) {
  const snippets = new Map();
  const pattern =
    /----\s+(\S+)\s+stdout\s+----\n([\s\S]*?)(?=\n----|\nfailures:|\ntest result:|$)/g;
  for (const match of raw.matchAll(pattern))
    snippets.set(match[1], match[2].trim());
  return snippets;
}

function friendlySnippet(text) {
  if (!text) return "";
  if (/not yet implemented/i.test(text))
    return "This function still has `todo!()` in it. Replace it with your implementation, then run again.";
  return text
    .replace(/^(thread .* panicked at.*\n)+/, "")
    .split("\n")
    .slice(0, 6)
    .join("\n")
    .trim();
}

export function cargoOutput(data) {
  return (
    [data.stdout, data.stderr]
      .filter((text) => typeof text === "string" && text.length)
      .join("\n\n") || "(no output)"
  );
}

export function createResultRenderer({
  panel,
  list,
  output,
  details,
  testResults,
}) {
  const staleNotice = panel?.ownerDocument.createElement("p");
  if (staleNotice) {
    staleNotice.className = "editor-stale-results";
    staleNotice.textContent =
      "Results from a previous version. Run again to update.";
    staleNotice.hidden = true;
    panel.prepend(staleNotice);
  }
  let hasResults = false;
  const render = (data) => {
    hasResults = true;
    if (staleNotice) staleNotice.hidden = true;
    if (panel) panel.style.display = "block";
    const raw = cargoOutput(data);
    // Keep logs selectable without adding another editor or focus stop.
    if (output) highlightOutput(output, raw);
    const tests = Array.isArray(data.test_results) ? data.test_results : [];
    if (list) list.replaceChildren();
    if (testResults && list) {
      const snippets = failureSnippets(raw);
      for (const test of tests) {
        const li = list.ownerDocument.createElement("li");
        li.className = "editor-test-result";
        const label = list.ownerDocument.createElement("span");
        label.textContent = `${test.passed === true ? "Passed" : "Failed"}: ${test.name}`;
        label.className =
          test.passed === true ? "editor-test-pass" : "editor-test-fail";
        li.append(label);
        if (test.passed !== true) {
          const snippet = friendlySnippet(snippets.get(test.name));
          if (snippet) {
            const detail = list.ownerDocument.createElement("pre");
            detail.className = "editor-test-detail";
            highlightOutput(detail, snippet);
            li.append(detail);
          }
        }
        list.append(li);
      }
    }
    if (details)
      details.open =
        !data.success || (tests.length > 0 && !submissionPassed(data));
  };
  render.invalidate = () => {
    if (hasResults && staleNotice) staleNotice.hidden = false;
  };
  render.clear = () => {
    hasResults = false;
    if (panel) panel.style.display = "none";
    if (staleNotice) staleNotice.hidden = true;
  };
  render.destroy = () => staleNotice?.remove();
  return render;
}

export function updateProgress(document, data) {
  const meta = document.getElementById("exercise-meta");
  if (meta && !meta.querySelector(".status-perfected, .status-completed")) {
    const badge = document.createElement("span");
    badge.className = "status-badge status-completed";
    badge.textContent = "Passed";
    meta.replaceChildren(badge);
  }
  document
    .getElementById("current-chapter-row")
    ?.classList.add("attempted", "completed");
  if (typeof data?.progress_done !== "number") return;
  const wrapper = document.querySelector(".topbar-progress");
  const count = wrapper?.querySelector(".topbar-progress-count strong");
  if (count) count.textContent = String(data.progress_done);
  if (wrapper && typeof data.progress_total === "number")
    wrapper.setAttribute(
      "aria-label",
      `Progress: ${data.progress_done} of ${data.progress_total} chapters completed`,
    );
}
