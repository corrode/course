#!/usr/bin/env python3
"""Smoke-test the workspace on macOS/Linux using Python 3's standard library.

Run `cargo build --workspace --bins`, then `python3 scripts/check-workspace.py`.
Requires installed Cargo, rustfmt, and Clippy. Never builds workspace binaries;
Cargo metadata resolves the target directory (including CARGO_TARGET_DIR), and
binaries must be in its debug/ directory (no cross-target/profile override).
Only a temporary learner crate is compiled. Cargo runs offline; HTTP requests
stay on loopback, without browser/Playground calls. The server currently binds
all interfaces, so run on a trusted machine. Commands time out after 90 seconds,
startup after 20 seconds; temporary processes, DB, and logs are cleaned up.
"""

from contextlib import closing
import html
import json
import os
from pathlib import Path
import re
import shutil
import signal
import socket
import sqlite3
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request


ROOT = Path(__file__).resolve().parent.parent
EXERCISE = "00_numbers_in_rust/3_add_health"
NAME = "Workspace Smoke Learner"
SOURCE = """pub fn add_health(current: u8, gain: u8) -> u8 {
    current.saturating_add(gain)
}

#[test]
fn test_add_health() {
    assert_eq!(add_health(100, 50), 150);
    assert_eq!(add_health(200, 100), 255);
}
"""


def check(condition, message):
    if not condition:
        raise RuntimeError(message)


def stop(process):
    """Reap the process and kill its group, including nested Cargo/rustc jobs."""
    try:
        os.killpg(process.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    try:
        process.wait(timeout=3)
    except subprocess.TimeoutExpired:
        pass
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait(timeout=3)


def run(args, cwd, env, stdin=""):
    print("+ " + " ".join(map(str, args)), flush=True)
    # A file avoids pipe deadlocks if a timed-out command leaves children alive.
    with tempfile.TemporaryFile(mode="w+b") as log:
        process = subprocess.Popen(
            args, cwd=cwd, env=env, stdin=subprocess.PIPE,
            stdout=log, stderr=subprocess.STDOUT, text=True,
            start_new_session=True,
        )
        try:
            process.communicate(stdin, timeout=90)
        except subprocess.TimeoutExpired as error:
            raise RuntimeError(f"Timed out after 90s: {args}") from error
        finally:
            stop(process)
        log.seek(0)
        output = log.read().decode("utf-8", errors="replace")
    check(process.returncode == 0,
          f"Command failed ({process.returncode}): {args}\n{output[-12000:]}")
    return output


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise RuntimeError(f"Unexpected HTTP redirect: {req.full_url} -> {newurl}")


def exercise_workflows(learner, db_path, env, get):
    chapter = learner / "examples" / "00_numbers_in_rust"
    chapter.mkdir(parents=True)
    (learner / "src").mkdir()
    (learner / "Cargo.toml").write_text(
        '[package]\nname = "workspace-smoke-learner"\n'
        'version = "0.0.0"\nedition = "2024"\n', encoding="utf-8"
    )
    step = chapter / "3_add_health.rs"
    step.write_text(SOURCE, encoding="utf-8")
    main_body = """mod _3_add_health;

fn main() {
    assert_eq!(_3_add_health::add_health(200, 100), 255);
}
"""
    (chapter / "main.rs").write_text(
        '#[path = "3_add_health.rs"]\n' + main_body, encoding="utf-8"
    )
    # Also expose the exercise to default `cargo clippy` (not just --examples).
    (learner / "src" / "main.rs").write_text(
        '#[path = "../examples/00_numbers_in_rust/3_add_health.rs"]\n' + main_body,
        encoding="utf-8",
    )
    shutil.copyfile(ROOT / "rust-toolchain.toml", learner / "rust-toolchain.toml")

    def cli(*args, stdin=""):
        return run(["cargo", "course", *args], learner, env, stdin)

    with closing(sqlite3.connect(f"{db_path.as_uri()}?mode=ro", uri=True)) as db:
        applied = db.execute(
            "SELECT version FROM _sqlx_migrations WHERE success = 1 ORDER BY version"
        ).fetchall()
        expected = sorted(int(p.name.split("_", 1)[0])
                          for p in (ROOT / "migrations").glob("*.sql"))
        check(applied == [(v,) for v in expected] and expected,
              f"Root migrations not applied: {applied}, expected {expected}")

        cli("init", stdin=NAME + "\n")
        token_file = learner / ".corrode" / "token"
        token = token_file.read_text(encoding="utf-8").strip()
        check(re.fullmatch(r"[0-9A-HJKMNP-TV-Z]{26}", token), "Invalid saved token")
        check(cli("token").strip() == token, "cargo course token differs from saved token")
        check("already registered" in cli("init"), "init was not idempotent")
        token_file.unlink()
        cli("init", "--token", token)
        check(cli("token").strip() == token, "init --token did not restore the token")
        check(db.execute("SELECT id, name FROM participants").fetchall() == [(token, NAME)],
              "Registration/restore/idempotence should leave exactly one participant")
        check("Usage:" in cli("open", "--help"), "open --help missing usage")

        def status():
            progress = json.loads(get(f"/api/status/{token}", "application/json"))["exercises"]
            check(any(e["name"] == "00_numbers_in_rust" for e in progress),
                  "Status is missing root examples catalog")
            output = cli("status")
            for exercise in progress:
                icon = "⭐" if exercise["perfected"] else "✅" if exercise["completed"] else "⏳"
                check(f'{icon} {exercise["name"]}' in output,
                      f"CLI/API status mismatch: {exercise}")
            done = sum(e["completed"] for e in progress)
            check(f"Progress: {done}/{len(progress)} exercises" in output,
                  "CLI/API progress totals differ")
            return progress

        initial = status()
        check(all(not e["completed"] and not e["perfected"] for e in initial),
              "New participant unexpectedly has progress")
        cli("submit", f"examples/{EXERCISE}.rs", "--pedantic")
        rows = db.execute(
            "SELECT participant_id, exercise_name, source_code, tests_passed, "
            "clippy_passed, fmt_passed FROM submissions"
        ).fetchall()
        check(rows == [(token, EXERCISE, SOURCE, 1, 1, 1)],
              f"Pedantic submission did not store passing test/clippy/fmt results: {rows}")

        # Different source avoids deduplication masking a broken batch upload.
        batch_source = SOURCE + "\n// Batch submission smoke check.\n"
        step.write_text(batch_source, encoding="utf-8")
        check("Successfully submitted: 1" in cli("submit", "--all"),
              "Batch submit did not report exactly one successful exercise")
        rows = db.execute(
            "SELECT participant_id, exercise_name, tests_passed, clippy_passed, fmt_passed "
            "FROM submissions WHERE source_code = ?", (batch_source,)
        ).fetchall()
        check(rows == [(token, EXERCISE, 1, 0, 0)], f"Batch submission incorrect: {rows}")
        check(db.execute("SELECT count(*) FROM submissions").fetchone() == (2,),
              "Expected two submissions; aggregator main.rs must not be submitted")
        # One step need not complete the chapter; compare CLI with API rollups.
        status()

        for path, marker in [
            ("/", "/static/css/base.css"),
            (f"/dashboard/{token}", NAME),
            ("/exercise/numbers_in_rust", f'data-exercise-key="{EXERCISE}"'),
            (f"/exercise/{token}/numbers_in_rust", "Batch submission smoke check."),
            ("/admin?token=workspace-smoke-admin", NAME),
        ]:
            page = html.unescape(get(path, "text/html").decode("utf-8"))
            check(marker in page, f"{path}: missing rendered content {marker!r}")
            if path == "/exercise/numbers_in_rust":
                check('class="solution-source"' in page and "current.saturating_add(gain)" in page,
                      "Reference solutions were not loaded from root solutions/")
        for asset, content_type in [
            ("css/base.css", "text/css"),
            ("js/analytics.js", "javascript"),
            ("assets/corrode-logo.svg", "image/svg+xml"),
        ]:
            body = get(f"/static/{asset}", content_type)
            check(body == (ROOT / "static" / asset).read_bytes() and body,
                  f"Static asset differs from root static/{asset}")


def main():
    check(os.name == "posix", "This smoke test requires macOS/Linux process groups")
    env = os.environ.copy()
    env.update(CARGO_NET_OFFLINE="true", CARGO_TERM_COLOR="never")
    metadata = json.loads(run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        ROOT, env,
    ))
    packages = {package["name"]: package for package in metadata["packages"]}
    check(not packages["course-exercises"]["dependencies"],
          "Learner exercise targets must not pull in application dependencies")
    check(Path(packages["cargo-course"]["manifest_path"]) == ROOT / "crates/cli/Cargo.toml",
          "The CLI must remain a separate workspace member")
    bin_dir = Path(metadata["target_directory"]) / "debug"
    for binary in ("cargo-course", "server"):
        check((bin_dir / binary).is_file() and os.access(bin_dir / binary, os.X_OK),
              f"Missing executable: {bin_dir / binary}. Run cargo build --workspace --bins first.")

    # tower-http and syn are also client/serde dependencies, not server-only.
    forbidden = {"course-server", "axum", "axum-core", "askama", "askama_derive",
                 "sqlx", "sqlx-core", "sqlx-sqlite", "dotenvy", "env_logger",
                 "pulldown-cmark", "ulid", "chrono"}
    for package in ("cargo-course", "course-types"):
        tree = run(["cargo", "tree", "--offline", "--locked", "-p", package,
                    "--prefix", "none", "--format", "{p}"], ROOT, env)
        names = {line.split()[0] for line in tree.splitlines() if line.strip()}
        check(package in names, f"Empty/unrecognized dependency tree for {package}")
        check(not names & forbidden,
              f"Server-only dependencies leaked into {package}: {sorted(names & forbidden)}")

    with tempfile.TemporaryDirectory(prefix="course-workspace-smoke-") as temporary:
        temp = Path(temporary)
        # Cargo can search CARGO_HOME/bin before PATH. An empty temporary home
        # prevents an installed cargo-course or user Cargo alias from winning.
        env.update(CARGO_HOME=str(temp / "cargo-home"),
                   CARGO_TARGET_DIR=str(temp / "target"),
                   PATH=str(bin_dir) + os.pathsep + env.get("PATH", ""),
                   NO_PROXY="127.0.0.1,localhost", no_proxy="127.0.0.1,localhost")
        check(shutil.which("cargo-course", path=env["PATH"]) == str(bin_dir / "cargo-course"),
              "PATH does not resolve to the workspace cargo-course")
        with socket.socket() as reservation:
            reservation.bind(("127.0.0.1", 0))
            port = reservation.getsockname()[1]
        base_url = f"http://127.0.0.1:{port}"
        db_path = temp / "course.db"
        env.update(CORRODE_SERVER_URL=base_url, DATABASE_URL=f"sqlite:{db_path}",
                   CORRODE_ADMIN_TOKEN="workspace-smoke-admin", PORT=str(port), RUST_LOG="info")
        opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())

        def get(path, content_type="text/plain"):
            with opener.open(base_url + path, timeout=2) as response:
                check(response.status == 200, f"{path}: HTTP {response.status}")
                check(content_type in response.headers.get("Content-Type", ""),
                      f"{path}: unexpected Content-Type {response.headers.get('Content-Type')}")
                return response.read()

        with (temp / "server.log").open("w+b") as log:
            server = subprocess.Popen([str(bin_dir / "server")], cwd=ROOT, env=env,
                                      stdin=subprocess.DEVNULL, stdout=log, stderr=log,
                                      start_new_session=True)
            try:
                deadline = time.monotonic() + 20
                while True:
                    check(server.poll() is None, f"Server exited early ({server.returncode})")
                    try:
                        if get("/health") == b"ok":
                            break
                    except (urllib.error.URLError, TimeoutError, ConnectionError):
                        pass
                    check(time.monotonic() < deadline, "Server health did not become ready within 20s")
                    time.sleep(0.1)
                exercise_workflows(temp / "learner", db_path, env, get)
                check(server.poll() is None, "Server exited during smoke checks")
            except BaseException:
                log.flush()
                log.seek(max(0, log.seek(0, os.SEEK_END) - 12000))
                print("\n--- server log ---\n" + log.read().decode("utf-8", errors="replace"),
                      file=sys.stderr)
                raise
            finally:
                stop(server)
    print("PASS: dependency isolation, CLI workflows, migrations, and root web assets")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("FAIL: interrupted (child processes cleaned up)", file=sys.stderr)
        sys.exit(130)
    except Exception as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
