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
//   4. update completion progress (not a grade).
// Reset wipes every card back to its initial state.

(function () {
  "use strict";

  const quizzes = document.querySelectorAll("[data-quiz]");
  quizzes.forEach(initQuiz);

  function initQuiz(quiz) {
    const cards = Array.from(quiz.querySelectorAll("[data-quiz-card]"));
    const answeredEl = quiz.querySelector("[data-quiz-answered]");
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
      if (answeredEl) answeredEl.textContent = String(answered);
      if (answered === total) showFooter();
    }

    function showFooter() {
      if (!footer || !headline) return;
      footer.hidden = false;
      headline.textContent =
        "You've answered every question. Pick an explanation that surprised you and try a small code example to check your understanding. Your answers aren't saved.";
    }

    function resetAll() {
      cards.forEach(resetCard);
      if (answeredEl) answeredEl.textContent = "0";
      if (footer) footer.hidden = true;
      if (headline) headline.textContent = "";
      quiz.querySelector("[data-quiz-answer]")?.focus();
    }
  }

  function wireCard(card, onAnswered) {
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

  function revealCard(card, answers, picked, status) {
    const pickedCorrect = picked.dataset.correct === "true";
    card.classList.add("is-answered");
    card.classList.add(pickedCorrect ? "is-correct" : "is-wrong");
    if (status) {
      status.textContent = pickedCorrect
        ? "Your choice is correct. All explanations are open."
        : "All explanations are open. Compare your choice with the correct answer.";
    }
    answers.forEach((btn) => {
      // Keep native buttons in the tab order so keyboard users can review
      // their choices and descriptions without losing focus on selection.
      btn.setAttribute("aria-disabled", "true");
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
      const label = btn.querySelector("[data-quiz-answer-status]");
      if (label) {
        label.textContent = isPicked
          ? isCorrect
            ? "Your choice · Correct answer"
            : "Your choice · Not correct"
          : isCorrect
            ? "Correct answer"
            : "Not correct";
      }
      const exp = wrap && wrap.querySelector("[data-quiz-explanation]");
      if (exp) {
        exp.hidden = false;
        btn.setAttribute("aria-describedby", exp.id);
      }
    });
  }

  function resetCard(card) {
    card.classList.remove("is-answered", "is-correct", "is-wrong");
    const status = card.querySelector("[data-quiz-status]");
    if (status) status.textContent = "";
    const hint = card.querySelector("[data-quiz-hint]");
    if (hint) hint.open = false;
    const answers = card.querySelectorAll("[data-quiz-answer]");
    answers.forEach((btn) => {
      btn.removeAttribute("aria-disabled");
      btn.removeAttribute("aria-describedby");
      const label = btn.querySelector("[data-quiz-answer-status]");
      if (label) label.textContent = "";
      btn.classList.remove("is-correct", "is-wrong-pick", "is-chosen");
      const wrap = btn.closest(".quiz-answer-wrap");
      wrap && wrap.classList.remove("is-correct", "is-wrong-pick");
      const exp = wrap && wrap.querySelector("[data-quiz-explanation]");
      if (exp) exp.hidden = true;
    });
  }
})();
