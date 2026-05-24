// Quiz interaction.
//
// Loaded by exercise.html on chapters that render a `<section data-quiz>`
// block (currently just the Rust Fundamentals Quiz). One self-contained
// module: no exports, no globals, no framework. Initialises every quiz
// section it can find on the page.
//
// The server emits per-answer `data-correct` already, so the runtime
// just needs to:
//   1. lock the card once an answer is clicked,
//   2. paint correct / wrong-pick verdicts on every answer,
//   3. reveal each answer's explanation,
//   4. tick the "answered" counter and, on the last answer, the score.
// Reset wipes every card back to its initial state.

(function () {
  "use strict";

  const quizzes = document.querySelectorAll("[data-quiz]");
  quizzes.forEach(initQuiz);

  function initQuiz(quiz) {
    const cards = Array.from(quiz.querySelectorAll("[data-quiz-card]"));
    const answeredEl = quiz.querySelector("[data-quiz-answered]");
    const scoreEl = quiz.querySelector("[data-quiz-score]");
    const scoreWrap = quiz.querySelector("[data-quiz-score-wrap]");
    const footer = quiz.querySelector("[data-quiz-footer]");
    const headline = quiz.querySelector("[data-quiz-headline]");
    const resetBtn = quiz.querySelector("[data-quiz-reset]");
    const total = cards.length;

    cards.forEach((card) => wireCard(card, onAnswered));
    if (resetBtn) {
      resetBtn.addEventListener("click", resetAll);
    }

    function onAnswered() {
      const answered = cards.filter((c) =>
        c.classList.contains("is-answered"),
      ).length;
      const correct = cards.filter((c) =>
        c.classList.contains("is-correct"),
      ).length;
      if (answeredEl) answeredEl.textContent = String(answered);
      if (answered > 0 && scoreWrap) {
        scoreWrap.hidden = false;
        if (scoreEl) scoreEl.textContent = String(correct);
      }
      if (answered === total) {
        showFooter(correct);
      }
    }

    function showFooter(correct) {
      if (!footer || !headline) return;
      footer.hidden = false;
      const verdict =
        correct === total
          ? `Perfect run \u2014 ${correct}/${total}. You know your Rust.`
          : correct >= Math.ceil(total * 0.8)
            ? `Solid: <strong>${correct}/${total}</strong>. Skim the explanations on the ones you missed.`
            : correct >= Math.ceil(total * 0.5)
              ? `<strong>${correct}/${total}</strong>. Worth another pass through the chapters you stumbled on.`
              : `<strong>${correct}/${total}</strong>. The explanations above are where the learning is \u2014 read them, then try again.`;
      headline.innerHTML = verdict;
      footer.scrollIntoView({ behavior: "smooth", block: "nearest" });
    }

    function resetAll() {
      cards.forEach(resetCard);
      if (answeredEl) answeredEl.textContent = "0";
      if (scoreEl) scoreEl.textContent = "0";
      if (scoreWrap) scoreWrap.hidden = true;
      if (footer) footer.hidden = true;
      const first = quiz.querySelector("[data-quiz-card]");
      if (first) first.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  function wireCard(card, onAnswered) {
    // Shuffle answers so the correct one isn't always in the same
    // slot (the source file tends to put it second). Markers are
    // re-lettered after the shuffle so the visible A/B/C/D order
    // stays sequential.
    shuffleAnswers(card);
    const answers = Array.from(card.querySelectorAll("[data-quiz-answer]"));
    const status = card.querySelector("[data-quiz-status]");
    answers.forEach((btn) => {
      btn.addEventListener("click", () => {
        if (card.classList.contains("is-answered")) return;
        revealCard(card, answers, btn, status);
        onAnswered();
      });
    });
  }

  function shuffleAnswers(card) {
    const list = card.querySelector(".quiz-answers");
    if (!list) return;
    const wraps = Array.from(list.children);
    // Fisher-Yates on the detached array, then reattach in order.
    for (let i = wraps.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [wraps[i], wraps[j]] = [wraps[j], wraps[i]];
    }
    wraps.forEach((wrap, idx) => {
      list.appendChild(wrap);
      const btn = wrap.querySelector("[data-quiz-answer]");
      const marker = wrap.querySelector(".quiz-answer-marker");
      if (btn) btn.dataset.answerIndex = String(idx);
      if (marker) marker.textContent = letterFor(idx);
    });
  }

  function letterFor(i) {
    return i < 26 ? String.fromCharCode(65 + i) : String(i + 1);
  }

  function revealCard(card, answers, picked, status) {
    const pickedCorrect = picked.dataset.correct === "true";
    card.classList.add("is-answered");
    card.classList.add(pickedCorrect ? "is-correct" : "is-wrong");
    if (status) {
      status.textContent = pickedCorrect ? "Correct" : "Incorrect";
    }
    answers.forEach((btn) => {
      btn.disabled = true;
      const wrap = btn.closest(".quiz-answer-wrap");
      const isCorrect = btn.dataset.correct === "true";
      const isPicked = btn === picked;
      if (isCorrect) {
        btn.classList.add("is-correct");
        wrap && wrap.classList.add("is-correct");
      }
      if (isPicked && !isCorrect) {
        btn.classList.add("is-wrong-pick");
        wrap && wrap.classList.add("is-wrong-pick");
      }
      if (isPicked) {
        btn.classList.add("is-chosen");
      }
      // Reveal the picked answer's explanation. If the pick was
      // wrong, also reveal the correct answer's explanation so
      // the visitor sees what they should have chosen. Other
      // distractors stay hidden to keep the card uncluttered.
      const showExplanation = isPicked || (!pickedCorrect && isCorrect);
      const exp = wrap && wrap.querySelector("[data-quiz-explanation]");
      if (exp && showExplanation) exp.hidden = false;
    });
  }

  function resetCard(card) {
    card.classList.remove("is-answered", "is-correct", "is-wrong");
    const status = card.querySelector("[data-quiz-status]");
    if (status) status.textContent = "";
    const answers = card.querySelectorAll("[data-quiz-answer]");
    answers.forEach((btn) => {
      btn.disabled = false;
      btn.classList.remove("is-correct", "is-wrong-pick", "is-chosen");
      const wrap = btn.closest(".quiz-answer-wrap");
      wrap && wrap.classList.remove("is-correct", "is-wrong-pick");
      const exp = wrap && wrap.querySelector("[data-quiz-explanation]");
      if (exp) exp.hidden = true;
    });
  }
})();
