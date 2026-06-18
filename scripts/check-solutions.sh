#!/usr/bin/env bash
#
# Verify the exercise solutions in `solutions/`.
#
# Each file under `solutions/` mirrors an exercise step under `examples/`
# by path (e.g. `solutions/09_option/2_fallback.rs` solves
# `examples/09_option/2_fallback.rs`). A solution is the exercise file
# with every `todo!()` replaced by a correct body; the exercise's own
# `#[test]` blocks travel with it, so passing them proves correctness.
#
# Exercises are std-only, so each solution is a self-contained, self-
# testing file we can compile and run with plain `rustc --test` (no
# cargo target wiring, no `fn main()` needed).
#
# This script:
#   1. Orphan check  - every solution maps to a real exercise file.
#   2. Correctness   - every solution compiles and its tests pass.
#   3. Coverage      - reports how many exercise steps have a solution.
#                      Only fatal when REQUIRE_COMPLETE=1 (flip that on
#                      once all chapters are solved).
#
# Run locally from the repo root with:
#   ./scripts/check-solutions.sh
# Verbose missing list:
#   VERBOSE=1 ./scripts/check-solutions.sh
# Enforce full coverage:
#   REQUIRE_COMPLETE=1 ./scripts/check-solutions.sh

set -uo pipefail

EDITION=2024
fail=0
checked=0

tmpdir="$(mktemp -d)"
log="$tmpdir/rustc.log"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -d solutions ]]; then
    echo "check-solutions: no solutions/ directory found."
    exit 1
fi

# 1 + 2: orphan check and correctness, per solution file.
while IFS= read -r sol; do
    rel="${sol#solutions/}"
    exercise="examples/$rel"

    if [[ ! -f "$exercise" ]]; then
        echo "ORPHAN   $sol (no matching $exercise)"
        fail=1
        continue
    fi

    checked=$((checked + 1))
    bin="$tmpdir/$(echo "$rel" | tr '/.' '__')"

    if ! rustc --test --edition "$EDITION" "$sol" -o "$bin" >"$log" 2>&1; then
        echo "COMPILE  $sol"
        sed 's/^/    /' "$log"
        fail=1
        continue
    fi
    if ! "$bin" >"$log" 2>&1; then
        echo "TEST     $sol"
        sed 's/^/    /' "$log"
        fail=1
        continue
    fi
    echo "OK       $sol"
done < <(find solutions -name '*.rs' | sort)

# 3: coverage over every exercise step (excluding generated main.rs).
total=0
missing=0
while IFS= read -r exercise; do
    total=$((total + 1))
    rel="${exercise#examples/}"
    if [[ ! -f "solutions/$rel" ]]; then
        missing=$((missing + 1))
        [[ "${VERBOSE:-0}" == "1" ]] && echo "MISSING  solutions/$rel"
    fi
done < <(find examples -name '*.rs' ! -name 'main.rs' | sort)

solved=$((total - missing))
echo
echo "check-solutions: $solved/$total exercise steps solved; $checked solution file(s) verified."

if [[ "${REQUIRE_COMPLETE:-0}" == "1" && "$missing" -ne 0 ]]; then
    echo "check-solutions: $missing exercise step(s) still missing a solution (REQUIRE_COMPLETE=1)."
    fail=1
fi

if [[ "$fail" -ne 0 ]]; then
    echo "check-solutions: FAILED"
    exit 1
fi

echo "check-solutions: OK"
