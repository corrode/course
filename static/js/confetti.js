let confettiPromise;

function loadConfetti() {
  if (typeof window.confetti === "function") return Promise.resolve(window.confetti);
  if (confettiPromise) return confettiPromise;

  confettiPromise = new Promise((resolve, reject) => {
    const script = document.createElement("script");
    script.src =
      "https://cdn.jsdelivr.net/npm/canvas-confetti@1.9.3/dist/confetti.browser.min.js";
    script.onload = () => resolve(window.confetti);
    script.onerror = () => reject(new Error("canvas-confetti failed to load"));
    document.head.appendChild(script);
  });
  return confettiPromise;
}

export async function celebrateWithConfetti() {
  const confetti = await loadConfetti();
  const fire = (originX) =>
    confetti({
      particleCount: 120,
      spread: 80,
      startVelocity: 55,
      ticks: 250,
      zIndex: 10000,
      origin: { x: originX, y: 0.7 },
    });
  fire(0.2);
  fire(0.5);
  fire(0.8);
}
