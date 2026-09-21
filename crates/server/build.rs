/// Emit `GIT_BRANCH` / `GIT_HASH` as compile-time env vars for the server
/// footer.
///
/// Best-effort: prefer build-time overrides from the environment, then fall
/// back to asking `git`. Docker builds exclude `.git` (see `.dockerignore`), so
/// the deploy passes `GIT_BRANCH` / `GIT_HASH` as build args instead; Coolify
/// also injects `SOURCE_COMMIT`. If nothing is available we emit `unknown`.
fn main() {
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-env-changed=GIT_BRANCH");
    println!("cargo:rerun-if-env-changed=GIT_HASH");
    println!("cargo:rerun-if-env-changed=SOURCE_COMMIT");

    let git = |args: &[&str]| -> Option<String> {
        let out = std::process::Command::new("git").args(args).output().ok()?;
        if !out.status.success() {
            return None;
        }
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if s.is_empty() { None } else { Some(s) }
    };

    // A non-empty environment variable, trimmed. Used for build-arg overrides.
    let env_var = |key: &str| -> Option<String> {
        std::env::var(key)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };

    let branch = env_var("GIT_BRANCH")
        .or_else(|| git(&["rev-parse", "--abbrev-ref", "HEAD"]))
        .unwrap_or_else(|| "unknown".into());

    // Hash precedence: explicit GIT_HASH, then Coolify's SOURCE_COMMIT
    // (shortened to 7 chars), then local git, then unknown.
    let hash = env_var("GIT_HASH")
        .or_else(|| env_var("SOURCE_COMMIT").map(|s| s.chars().take(7).collect()))
        .or_else(|| git(&["rev-parse", "--short", "HEAD"]))
        .unwrap_or_else(|| "unknown".into());

    println!("cargo:rustc-env=GIT_BRANCH={branch}");
    println!("cargo:rustc-env=GIT_HASH={hash}");
}
