// Keep native output/cursor scrolling clear of the section's sticky controls.
export function createEditorChrome(section) {
  const toolbar = section.querySelector('[data-role="editor-toolbar"]');
  const document = section.ownerDocument;
  const topbar = document.querySelector(".topbar");
  let topbarHeight = 0;
  let toolbarHeight = 0;
  let destroyed = false;
  function measure() {
    if (!toolbar || destroyed) return;
    topbarHeight = topbar?.getBoundingClientRect().height || 0;
    toolbarHeight = toolbar.getBoundingClientRect().height;
    section.style.setProperty("--editor-topbar-height", `${topbarHeight}px`);
    section.style.setProperty("--editor-toolbar-height", `${toolbarHeight}px`);
  }
  const Observer = document.defaultView.ResizeObserver;
  const observer = toolbar && Observer ? new Observer(measure) : null;
  if (observer) {
    observer.observe(toolbar);
    if (topbar) observer.observe(topbar);
  }
  measure();
  return {
    measure,
    scrollMargins() {
      if (!toolbar || section.classList.contains("is-fullscreen")) return null;
      // CodeMirror applies this margin to the viewport, not the scroller's top.
      return { top: topbarHeight + toolbarHeight };
    },
    destroy() {
      destroyed = true;
      observer?.disconnect();
      if (toolbar) {
        section.style.removeProperty("--editor-topbar-height");
        section.style.removeProperty("--editor-toolbar-height");
      }
    },
  };
}
