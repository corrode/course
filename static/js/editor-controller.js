// Own every listener and deferred draft write for a mount, including fallback.
export function createController({
  window,
  document,
  read,
  persist,
  delay = 300,
}) {
  const removers = [];
  let timer = null;
  let destroyed = false;
  let dirty = false;
  function listen(target, type, handler) {
    if (!target || destroyed) return;
    target.addEventListener(type, handler);
    removers.push(() => target.removeEventListener(type, handler));
  }
  function flush() {
    clearTimeout(timer);
    timer = null;
    if (!destroyed && dirty) {
      persist(read());
      dirty = false;
    }
  }
  listen(window, "pagehide", flush);
  listen(document, "visibilitychange", () => {
    if (document.visibilityState === "hidden") flush();
  });
  return {
    listen,
    flush,
    schedule() {
      if (destroyed) return;
      dirty = true;
      clearTimeout(timer);
      timer = setTimeout(flush, delay);
    },
    destroy() {
      if (destroyed) return;
      flush();
      destroyed = true;
      for (const remove of removers) remove();
      removers.length = 0;
    },
  };
}
