-- Migration: rename chapter directories again, this time to put the
-- *Rust concept* in the slug (and H1) rather than the example domain.
-- "HTTP Status Handling" → "Enums and Pattern Matching", etc.
--
-- This rewrite chains off migration 004: a row that was renamed by 004
-- but never updated again still holds the intermediate name on disk
-- (e.g. `03_http_status_handling`), so we map both the *original*
-- historical key and the *intermediate* one to the new key. Each
-- UPDATE only matches rows that still hold one of the old names, so
-- the migration is idempotent and safe to re-run.

-- 01: integer_handling → integer_operations → integers
UPDATE submissions SET exercise_name = '01_integers' WHERE exercise_name = '01_integer_operations';
UPDATE submissions SET exercise_name = '01_integers' WHERE exercise_name = '01_integer_handling';

-- 03: enums_and_matching → http_status_handling → enums_and_pattern_matching
UPDATE submissions SET exercise_name = '03_enums_and_pattern_matching' WHERE exercise_name = '03_http_status_handling';
UPDATE submissions SET exercise_name = '03_enums_and_pattern_matching' WHERE exercise_name = '03_enums_and_matching';

-- 04: vectors_basics → vector_basics → vectors
UPDATE submissions SET exercise_name = '04_vectors' WHERE exercise_name = '04_vector_basics';
UPDATE submissions SET exercise_name = '04_vectors' WHERE exercise_name = '04_vectors_basics';

-- 07: option_handling → option
UPDATE submissions SET exercise_name = '07_option' WHERE exercise_name = '07_option_handling';

-- 08: result_handling → result
UPDATE submissions SET exercise_name = '08_result' WHERE exercise_name = '08_result_handling';

-- 09: ownership_basics → ownership_and_borrowing
UPDATE submissions SET exercise_name = '09_ownership_and_borrowing' WHERE exercise_name = '09_ownership_basics';

-- 10: structs_and_methods → user_account_management → structs_and_methods
-- Only the intermediate (004) key needs rewriting; the original name is
-- already what we want.
UPDATE submissions SET exercise_name = '10_structs_and_methods' WHERE exercise_name = '10_user_account_management';

-- 11: iterator_patterns → iterators
UPDATE submissions SET exercise_name = '11_iterators' WHERE exercise_name = '11_iterator_patterns';
