// Run after `cargo build --bin server`. Requires Chrome and Node 22+;
// uses the real page, native keyboard input, and no browser-test dependency.
// node scripts/check-quiz-browser.mjs --screenshot=target/quiz.png --width=1440
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { mkdtemp, readFile, rm, mkdir, writeFile } from "node:fs/promises";
import { createServer } from "node:net";
import { dirname, resolve } from "node:path";
import { parseArgs } from "node:util";
import { fileURLToPath } from "node:url";

const { values } = parseArgs({ options: {
  chrome: { type: "string", default: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" },
  screenshot: { type: "string" },
  width: { type: "string", default: "1440" },
} });
const root = fileURLToPath(new URL("../", import.meta.url));
await mkdir(resolve(root, "target"), { recursive: true });
const scratch = await mkdtemp(resolve(root, "target/quiz-browser-"));
const children = [];
let socket;
let logs = "";
const delay = (ms) => new Promise((done) => setTimeout(done, ms));
function launch(command, args, env = process.env) {
  const child = spawn(command, args, { cwd: root, env, stdio: ["ignore", "pipe", "pipe"] });
  children.push(child);
  child.stdout.on("data", (data) => { logs += data; });
  child.stderr.on("data", (data) => { logs += data; });
  child.on("error", (error) => { logs += error.message; });
  return child;
}
async function until(fn) {
  const deadline = Date.now() + 15000;
  while (Date.now() < deadline) {
    try { const result = await fn(); if (result) return result; } catch { /* still starting */ }
    await delay(100);
  }
  throw new Error("Timed out waiting for quiz browser setup");
}
// Reserve an ephemeral port without relying on the user's running app.
const reservation = createServer();
await new Promise((done) => reservation.listen(0, "127.0.0.1", done));
const port = reservation.address().port;
await new Promise((done) => reservation.close(done));
try {
  launch(resolve(root, "target/debug/server"), [], {
    ...process.env, PORT: String(port), DATABASE_URL: `sqlite:${scratch}/course.db`,
    CORRODE_ADMIN_TOKEN: "local-quiz-regression-only",
  });
  await until(async () => (await fetch(`http://127.0.0.1:${port}/health`)).ok);
  launch(values.chrome, ["--headless=new", "--disable-gpu", "--no-first-run",
    "--disable-background-networking", "--remote-debugging-port=0",
    `--user-data-dir=${scratch}/chrome`, "about:blank"]);
  const debugPort = await until(async () => (await readFile(`${scratch}/chrome/DevToolsActivePort`, "utf8")).split("\n")[0]);
  const pages = await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json();
  socket = new WebSocket(pages.find((page) => page.type === "page").webSocketDebuggerUrl);
  await new Promise((done, reject) => {
    socket.addEventListener("open", done, { once: true });
    socket.addEventListener("error", reject, { once: true });
  });
  let id = 0;
  const pending = new Map();
  socket.addEventListener("message", ({ data }) => {
    const message = JSON.parse(data);
    const request = pending.get(message.id);
    if (!request) return;
    pending.delete(message.id);
    clearTimeout(request.timer);
    if (message.error) request.reject(new Error(message.error.message));
    else request.resolve(message.result);
  });
  function cdp(method, params = {}) {
    return new Promise((resolve, reject) => {
      const requestId = ++id;
      const timer = setTimeout(() => {
        pending.delete(requestId);
        reject(new Error(`Timed out: ${method}`));
      }, 10000);
      pending.set(requestId, { resolve, reject, timer });
      socket.send(JSON.stringify({ id: requestId, method, params }));
    });
  }
  async function js(expression) {
    const result = await cdp("Runtime.evaluate", { expression, returnByValue: true, awaitPromise: true });
    if (result.exceptionDetails) throw new Error(JSON.stringify(result.exceptionDetails));
    return result.result.value;
  }
  async function key(key, code, number) {
    for (const type of ["keyDown", "keyUp"]) {
      await cdp("Input.dispatchKeyEvent", {
        type, key, code, windowsVirtualKeyCode: number,
        text: type === "keyDown" ? (key === "Enter" ? "\r" : key === " " ? " " : "") : "",
      });
    }
  }
  async function screenshot(path) {
    const image = await cdp("Page.captureScreenshot", { format: "png" });
    await mkdir(dirname(resolve(path)), { recursive: true });
    await writeFile(path, Buffer.from(image.data, "base64"));
  }
  await cdp("Emulation.setDeviceMetricsOverride", {
    width: Number(values.width), height: 1200, deviceScaleFactor: 1, mobile: false,
  });
  await cdp("Page.navigate", { url: `http://127.0.0.1:${port}/exercise/rust_fundamentals_quiz` });
  await until(() => js('document.readyState === "complete" && !!document.querySelector("[data-quiz-answer]")'));
  await js(`
    window.quiz = document.querySelector('[data-quiz]');
    window.cards = [...quiz.querySelectorAll('[data-quiz-card]')];
    window.answers = card => [...card.querySelectorAll('[data-quiz-answer]')];
    window.explanations = card => [...card.querySelectorAll('[data-quiz-explanation]')];
    window.progress = () => quiz.querySelector('[data-quiz-answered]').textContent;
    window.footer = quiz.querySelector('[data-quiz-footer]');
    window.reset = () => quiz.querySelector('[data-quiz-reset]').click();
    window.storageBefore = JSON.stringify([Object.entries(localStorage), Object.entries(sessionStorage)]);
    window.quizRequests = 0;
    window.originalFetch = window.fetch;
    window.fetch = (...args) => { quizRequests++; return originalFetch(...args); };
    window.originalSend = XMLHttpRequest.prototype.send;
    XMLHttpRequest.prototype.send = function (...args) { quizRequests++; return originalSend.apply(this, args); };
    window.originalBeacon = navigator.sendBeacon;
    navigator.sendBeacon = (...args) => { quizRequests++; return originalBeacon.apply(navigator, args); };
  `);
  assert.ok(await js('cards.length >= 20'));
  assert.equal(await js('progress()'), "0");
  assert.equal(await js('cards.every(card => explanations(card).every(exp => exp.hidden))'), true);
  assert.equal(await js('!!quiz.querySelector("[data-quiz-score], [data-quiz-score-wrap]")'), false);
  // Exercise native Tab navigation, then select an incorrect answer with Enter.
  await js('answers(cards[0])[0].focus()');
  await key("Tab", "Tab", 9);
  assert.equal(await js('document.activeElement === answers(cards[0])[1]'), true);
  await js('window.picked = answers(cards[0]).find(btn => btn.dataset.correct === "false"); picked.focus()');
  await key("Enter", "Enter", 13);
  assert.equal(await js('progress()'), "1");
  assert.equal(await js('document.activeElement === picked'), true);
  assert.equal(await js(`explanations(cards[0]).every(exp => !exp.hidden)
    && answers(cards[0]).every(btn => !btn.disabled && btn.tabIndex === 0
      && btn.getAttribute('aria-disabled') === 'true'
      && document.getElementById(btn.getAttribute('aria-describedby')) === btn.nextElementSibling)
    && picked.textContent.includes('Your choice · Not correct')
    && answers(cards[0]).find(btn => btn.dataset.correct === 'true').textContent.includes('Correct answer')`), true);
  await js('answers(cards[0]).find(btn => btn.dataset.correct === "true").focus()');
  await key(" ", "Space", 32);
  assert.equal(await js('progress() === "1" && cards[0].querySelector(".is-chosen") === picked'), true);
  console.log("PASS: native Tab/Enter/Space, stable focus, all explanations, textual labels, locked selection");
  if (values.screenshot) {
    await js('cards[0].scrollIntoView({block: "start"})');
    await screenshot(values.screenshot);
  }
  // Complete out of order; every remaining answer is deliberately incorrect.
  await js('cards.slice(1).reverse().forEach(card => answers(card).find(btn => btn.dataset.correct === "false").click())');
  const completion = await js('quiz.querySelector("[data-quiz-headline]").textContent');
  assert.equal(await js('!footer.hidden && progress() === String(cards.length)'), true);
  assert.match(completion, /answered every question/);
  assert.doesNotMatch(completion, /score|perfect|solid|know your Rust|\d+\s*\//i);
  assert.equal(await js('cards.every(card => explanations(card).every(exp => !exp.hidden))'), true);
  const overflow = await js(`Array.from(quiz.querySelectorAll('*')).filter(el =>
    el.getBoundingClientRect().right > document.documentElement.clientWidth + 1
    && (!el.closest('pre') || el.matches('pre'))).map(el => el.className + ': ' + el.textContent.slice(0, 60))`);
  assert.deepEqual(overflow, [], 'Quiz text should fit the viewport');
  if (await js('document.documentElement.scrollWidth > document.documentElement.clientWidth')) {
    console.log('NOTE: page chrome overflows at this width; quiz content fits the viewport.');
  }
  if (values.screenshot) {
    await js('footer.scrollIntoView({block: "center"})');
    await screenshot(values.screenshot.replace(/\.png$/, "") + "-completion.png");
  }
  await js('reset()');
  assert.equal(await js(`progress() === '0' && footer.hidden
    && document.activeElement === answers(cards[0])[0]
    && cards.every(card => !card.classList.contains('is-answered')
      && explanations(card).every(exp => exp.hidden)
      && answers(card).every(btn => !btn.hasAttribute('aria-disabled')
        && !btn.hasAttribute('aria-describedby')
        && !btn.matches('.is-chosen, .is-correct, .is-wrong-pick')
        && btn.querySelector('[data-quiz-answer-status]').textContent === ''))`), true);
  // A correct answer selected with Space also opens every distractor explanation.
  await js('answers(cards[0]).find(btn => btn.dataset.correct === "true").focus()');
  await key(" ", "Space", 32);
  assert.equal(await js('progress() === "1" && explanations(cards[0]).every(exp => !exp.hidden)'), true);
  await js('cards.slice(1).forEach(card => answers(card).find(btn => btn.dataset.correct === "true").click())');
  assert.equal(await js('quiz.querySelector("[data-quiz-headline]").textContent'), completion);
  assert.equal(await js('JSON.stringify([Object.entries(localStorage), Object.entries(sessionStorage)]) === storageBefore'), true);
  assert.equal(await js('quizRequests'), 0);
  console.log("PASS: completion without grades (all wrong/all correct), full reset, no storage changes or requests");
  await cdp("Page.reload");
  await until(() => js('document.readyState === "complete" && document.querySelector("[data-quiz-answered]")?.textContent === "0"'));
  assert.equal(await js('[...document.querySelectorAll("[data-quiz-explanation]")].every(exp => exp.hidden)'), true);
  console.log("PASS: reload starts unanswered");
} catch (error) {
  console.error(error);
  console.error(logs);
  process.exitCode = 1;
} finally {
  socket?.close();
  for (const child of children.reverse()) {
    if (child.exitCode !== null) continue;
    child.kill("SIGTERM");
    await Promise.race([new Promise(done => child.once("exit", done)), delay(2000)]);
    if (child.exitCode === null) child.kill("SIGKILL");
  }
  await rm(scratch, { recursive: true, force: true, maxRetries: 3, retryDelay: 200 });
}
