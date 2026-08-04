// Small, privacy-conscious course analytics client. Events contain only a
// generated browser-tab session ID, the participant ID already present in the
// course URL (when signed in), an allowlisted event name, and an exercise key.
// Source code, names, URLs, user agents, and arbitrary metadata are never sent.

const SESSION_KEY = "corrode:analytics-session";

function newSessionId() {
  if (typeof globalThis.crypto?.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }
  return `session-${Date.now().toString(36)}-${Math.random().toString(36).slice(2)}`;
}

export function analyticsSessionId() {
  try {
    let id = sessionStorage.getItem(SESSION_KEY);
    if (!id) {
      id = newSessionId();
      sessionStorage.setItem(SESSION_KEY, id);
    }
    return id;
  } catch (_) {
    return newSessionId();
  }
}

export function participantId(root = document) {
  return root.querySelector("[data-participant-id]")?.dataset.participantId || null;
}

export function trackCourseEvent(eventType, exerciseName = null) {
  const body = JSON.stringify({
    participant_id: participantId(),
    session_id: analyticsSessionId(),
    event_type: eventType,
    exercise_name: exerciseName,
  });

  // UI analytics must never delay navigation or interrupt the course. sendBeacon
  // is reliable during page unload; fetch is the fallback for older browsers.
  if (navigator.sendBeacon) {
    const blob = new Blob([body], { type: "application/json" });
    if (navigator.sendBeacon("/api/events", blob)) return;
  }
  fetch("/api/events", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body,
    keepalive: true,
  }).catch(() => {});
}

export function bindCourseAnalytics(root = document) {
  const container = root.querySelector("[data-course-chapter]");
  if (!container) return;

  const chapter = container.dataset.courseChapter;
  trackCourseEvent("chapter_view", chapter);

  root.querySelectorAll(".exercise-section[data-exercise-key]").forEach((section) => {
    let focused = false;
    section.addEventListener("focusin", (event) => {
      if (
        focused ||
        !event.target.closest(".cm-editor, [data-role=editor-fallback]")
      ) {
        return;
      }
      focused = true;
      trackCourseEvent("editor_focus", section.dataset.exerciseKey);
    });
  });

  root.querySelectorAll(".hints-disclosure").forEach((details) => {
    let tracked = false;
    details.addEventListener("toggle", () => {
      if (!details.open || tracked) return;
      tracked = true;
      const key = details.dataset.exerciseKey || chapter;
      trackCourseEvent(
        details.classList.contains("solution-disclosure")
          ? "solution_revealed"
          : "hint_opened",
        key,
      );
    });
  });

  root.querySelectorAll(".next-chapter-cta a, a.next-chapter-cta").forEach((link) => {
    link.addEventListener("click", () => {
      trackCourseEvent("next_chapter_clicked", chapter);
    });
  });
}
