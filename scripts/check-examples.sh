#!/usr/bin/env bash
#
# Verify every exercise chapter is in its intended state.
#
# The course ships exercises as `todo!()` stubs that must still *compile*
# (and lint cleanly) even before a learner fills them in. A couple of
# chapters are the exception: they contain deliberately-broken teaching
# files (a stray semicolon, a privacy error) that are *supposed* to fail
# to compile so the learner reads the compiler error and fixes it.
#
# This script clippy-checks each chapter and asserts the outcome:
#   - normal chapters MUST compile and lint clean (`-D warnings`),
#   - chapters in EXPECTED_BROKEN MUST fail to compile.
#
# If an EXPECTED_BROKEN chapter starts compiling, that's a signal someone
# changed a teaching exercise; update the list below on purpose.
#
# Run locally from the repo root with:
#   ./scripts/check-examples.sh

set -uo pipefail

# Chapters whose `--example` target intentionally does not compile.
# Keyed by the chapter directory name (the cargo example target name).
EXPECTED_BROKEN=(
    "04_functions"               # 3_stray_semicolon.rs, 6_cap_at.rs
    "20_modules_and_visibility"  # 3_calculate.rs (privacy error)
)

is_expected_broken() {
    local name="$1"
    for broken in "${EXPECTED_BROKEN[@]}"; do
        [[ "$name" == "$broken" ]] && return 0
    done
    return 1
}

failures=0

for dir in examples/*/; do
    name="$(basename "$dir")"
    # Only directories with a `main.rs` are cargo example targets.
    [[ -f "${dir}main.rs" ]] || continue

    if cargo clippy --quiet --example "$name" -- -D warnings >/tmp/check-example.log 2>&1; then
        compiled=1
    else
        compiled=0
    fi

    if is_expected_broken "$name"; then
        if [[ "$compiled" -eq 1 ]]; then
            echo "UNEXPECTED PASS  $name (listed as a teaching example that should NOT compile)"
            echo "                 -> if you fixed it on purpose, remove it from EXPECTED_BROKEN."
            failures=$((failures + 1))
        else
            echo "OK (broken)      $name (fails to compile, as intended)"
        fi
    else
        if [[ "$compiled" -eq 1 ]]; then
            echo "OK               $name"
        else
            echo "FAIL             $name (expected to compile + lint clean)"
            echo "----- clippy output -----"
            cat /tmp/check-example.log
            echo "-------------------------"
            failures=$((failures + 1))
        fi
    fi
done

if [[ "$failures" -ne 0 ]]; then
    echo
    echo "check-examples: $failures chapter(s) not in their expected state."
    exit 1
fi

echo
echo "check-examples: all chapters in their expected state."
