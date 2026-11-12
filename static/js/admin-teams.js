export async function renameTeam(form, {
  request = fetch,
  navigate = (url) => window.location.assign(url),
} = {}) {
  if (form.dataset.saving === "true") return;

  const input = form.elements.namedItem("team_token");
  const button = form.querySelector('button[type="submit"]');
  const error = form.querySelector("[data-rename-error]");
  const label = button.textContent;
  const body = new URLSearchParams({ team_token: input.value });
  form.dataset.saving = "true";
  form.setAttribute("aria-busy", "true");
  input.readOnly = true;
  button.disabled = true;
  button.textContent = "Renaming…";
  error.hidden = true;
  error.textContent = "";

  try {
    const response = await request(form.action, {
      method: "POST",
      body,
      credentials: "same-origin",
    });
    if (response.ok && response.redirected) {
      navigate(response.url);
      return;
    }
    error.textContent = response.ok
      ? "The server did not confirm the rename. Reload the page to check the team name."
      : (await response.text()) || "Could not rename the team. Please try again.";
    error.hidden = false;
  } catch {
    error.textContent = "Could not confirm the rename. Check your connection and reload the page to check the team name before retrying.";
    error.hidden = false;
  } finally {
    delete form.dataset.saving;
    form.removeAttribute("aria-busy");
    input.readOnly = false;
    button.disabled = false;
    button.textContent = label;
  }
}

if (typeof document !== "undefined") {
  document.addEventListener("submit", (event) => {
    const form = event.target;
    if (!form.matches("[data-team-rename]")) return;
    event.preventDefault();
    void renameTeam(form);
  });
}
