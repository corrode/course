-- Reset submissions for the multi-step refactor.
--
-- Before this migration, `submissions.exercise_name` was always a chapter
-- slug (`07_option`). Multi-step chapters now post `<chapter>/<step>`
-- (e.g. `07_option/2_fallback`). The user has indicated existing data is
-- meaningless, so we wipe the table rather than backfilling.
--
-- The schema itself is unchanged: `exercise_name TEXT` already accommodates
-- both shapes.
DELETE FROM submissions;
