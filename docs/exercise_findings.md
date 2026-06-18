# Exercise findings

Notes gathered while writing the reference solutions in `solutions/`.

This branch is intentionally **additive**: it does not modify anything
under `examples/`. Instead, anything misleading, subtly wrong, or worth
improving in an exercise is recorded here, so it can become a separate,
focused exercise-fixing pass later.

Severity legend:

- **blocker** - no correct solution can pass the exercise as written
  (e.g. a wrong test). These are raised immediately rather than worked
  around.
- **misleading** - the exercise compiles and is solvable, but the prose,
  naming, or tests point a learner the wrong way.
- **nit** - cosmetic or stylistic; harmless but worth a glance.

## Findings by chapter

### 00_integers
None. The three steps (`number_to_string`, `damage_with_bonus`,
`parse_positive_integer`) are clear and their tests match the prose.

### 09_option
None. All four steps (`fallback`, `transform`, `first_char`,
`find_user`) are solvable with exactly the standard-library methods the
paired `.md` points at.

<!--
Template for new entries:

### NN_chapter
- **<severity>** `N_step.rs`: what's wrong, why it misleads, suggested fix.
-->
