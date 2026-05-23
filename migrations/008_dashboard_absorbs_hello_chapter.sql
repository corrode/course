-- Migration 008: dashboard absorbs `00_hello_rust`, on-disk chapters
-- shift down one slot.
--
-- Background. The old `00_hello_rust` chapter was a single-step
-- `println!("Hello, Rust!")` warm-up whose entire purpose was to give
-- visitors a "I ran some Rust" moment before the real material started.
-- That moment now lives directly on the dashboard (`/`), so the on-disk
-- chapter was deleted and every other chapter directory was renamed
-- with its prefix decremented by one:
--
--   01_integers              -> 00_integers
--   02_strings_and_chars     -> 01_strings_and_chars
--   ...
--   21_appendix              -> 20_appendix
--
-- `submissions.exercise_name` stores the directory prefix as part of
-- the key (chapter slug for legacy single-step rows; `<chapter>/<step>`
-- for multi-step). Any pre-existing rows would orphan without a
-- rewrite; this migration handles both formats.
--
-- The DB was wiped as part of the same change, so in practice this
-- migration is a no-op against fresh deployments. It exists for
-- developer DBs that survived the rename and to document the slug
-- shift in the migration history. Each statement is idempotent:
-- the WHERE clauses only match rows that still hold the *old* prefix.

-- Drop any submissions against the deleted hello chapter (single-step
-- key `00_hello_rust` and the multi-step key `00_hello_rust/<step>`).
DELETE FROM submissions WHERE exercise_name = '00_hello_rust'
                           OR exercise_name LIKE '00_hello_rust/%';

-- Shift every other chapter down by one. Two UPDATEs per chapter cover
-- the legacy single-step structure and the multi-step `<chapter>/<step>`
-- structure. We rewrite from the lowest new prefix upward (00, 01, ...)
-- so each statement only sees rows that haven't been touched yet.
UPDATE submissions SET exercise_name = '00_integers'
    WHERE exercise_name = '01_integers';
UPDATE submissions SET exercise_name = '00_integers/' || substr(exercise_name, length('01_integers/') + 1)
    WHERE exercise_name LIKE '01_integers/%';

UPDATE submissions SET exercise_name = '01_strings_and_chars'
    WHERE exercise_name = '02_strings_and_chars';
UPDATE submissions SET exercise_name = '01_strings_and_chars/' || substr(exercise_name, length('02_strings_and_chars/') + 1)
    WHERE exercise_name LIKE '02_strings_and_chars/%';

UPDATE submissions SET exercise_name = '02_conditionals_and_loops'
    WHERE exercise_name = '03_conditionals_and_loops';
UPDATE submissions SET exercise_name = '02_conditionals_and_loops/' || substr(exercise_name, length('03_conditionals_and_loops/') + 1)
    WHERE exercise_name LIKE '03_conditionals_and_loops/%';

UPDATE submissions SET exercise_name = '03_functions'
    WHERE exercise_name = '04_functions';
UPDATE submissions SET exercise_name = '03_functions/' || substr(exercise_name, length('04_functions/') + 1)
    WHERE exercise_name LIKE '04_functions/%';

UPDATE submissions SET exercise_name = '04_enums_and_pattern_matching'
    WHERE exercise_name = '05_enums_and_pattern_matching';
UPDATE submissions SET exercise_name = '04_enums_and_pattern_matching/' || substr(exercise_name, length('05_enums_and_pattern_matching/') + 1)
    WHERE exercise_name LIKE '05_enums_and_pattern_matching/%';

UPDATE submissions SET exercise_name = '05_vectors'
    WHERE exercise_name = '06_vectors';
UPDATE submissions SET exercise_name = '05_vectors/' || substr(exercise_name, length('06_vectors/') + 1)
    WHERE exercise_name LIKE '06_vectors/%';

UPDATE submissions SET exercise_name = '06_hashmaps'
    WHERE exercise_name = '07_hashmaps';
UPDATE submissions SET exercise_name = '06_hashmaps/' || substr(exercise_name, length('07_hashmaps/') + 1)
    WHERE exercise_name LIKE '07_hashmaps/%';

UPDATE submissions SET exercise_name = '07_tuples_and_destructuring'
    WHERE exercise_name = '08_tuples_and_destructuring';
UPDATE submissions SET exercise_name = '07_tuples_and_destructuring/' || substr(exercise_name, length('08_tuples_and_destructuring/') + 1)
    WHERE exercise_name LIKE '08_tuples_and_destructuring/%';

UPDATE submissions SET exercise_name = '08_option'
    WHERE exercise_name = '09_option';
UPDATE submissions SET exercise_name = '08_option/' || substr(exercise_name, length('09_option/') + 1)
    WHERE exercise_name LIKE '09_option/%';

UPDATE submissions SET exercise_name = '09_result'
    WHERE exercise_name = '10_result';
UPDATE submissions SET exercise_name = '09_result/' || substr(exercise_name, length('10_result/') + 1)
    WHERE exercise_name LIKE '10_result/%';

UPDATE submissions SET exercise_name = '10_ownership_and_borrowing'
    WHERE exercise_name = '11_ownership_and_borrowing';
UPDATE submissions SET exercise_name = '10_ownership_and_borrowing/' || substr(exercise_name, length('11_ownership_and_borrowing/') + 1)
    WHERE exercise_name LIKE '11_ownership_and_borrowing/%';

UPDATE submissions SET exercise_name = '11_structs_and_methods'
    WHERE exercise_name = '12_structs_and_methods';
UPDATE submissions SET exercise_name = '11_structs_and_methods/' || substr(exercise_name, length('12_structs_and_methods/') + 1)
    WHERE exercise_name LIKE '12_structs_and_methods/%';

UPDATE submissions SET exercise_name = '12_iterators'
    WHERE exercise_name = '13_iterators';
UPDATE submissions SET exercise_name = '12_iterators/' || substr(exercise_name, length('13_iterators/') + 1)
    WHERE exercise_name LIKE '13_iterators/%';

UPDATE submissions SET exercise_name = '13_password_validator'
    WHERE exercise_name = '14_password_validator';
UPDATE submissions SET exercise_name = '13_password_validator/' || substr(exercise_name, length('14_password_validator/') + 1)
    WHERE exercise_name LIKE '14_password_validator/%';

UPDATE submissions SET exercise_name = '14_question_mark_operator'
    WHERE exercise_name = '15_question_mark_operator';
UPDATE submissions SET exercise_name = '14_question_mark_operator/' || substr(exercise_name, length('15_question_mark_operator/') + 1)
    WHERE exercise_name LIKE '15_question_mark_operator/%';

UPDATE submissions SET exercise_name = '15_modules_and_visibility'
    WHERE exercise_name = '16_modules_and_visibility';
UPDATE submissions SET exercise_name = '15_modules_and_visibility/' || substr(exercise_name, length('16_modules_and_visibility/') + 1)
    WHERE exercise_name LIKE '16_modules_and_visibility/%';

UPDATE submissions SET exercise_name = '16_word_counter'
    WHERE exercise_name = '17_word_counter';
UPDATE submissions SET exercise_name = '16_word_counter/' || substr(exercise_name, length('17_word_counter/') + 1)
    WHERE exercise_name LIKE '17_word_counter/%';

UPDATE submissions SET exercise_name = '17_environment_file_parser'
    WHERE exercise_name = '18_environment_file_parser';
UPDATE submissions SET exercise_name = '17_environment_file_parser/' || substr(exercise_name, length('18_environment_file_parser/') + 1)
    WHERE exercise_name LIKE '18_environment_file_parser/%';

UPDATE submissions SET exercise_name = '18_csv_parser'
    WHERE exercise_name = '19_csv_parser';
UPDATE submissions SET exercise_name = '18_csv_parser/' || substr(exercise_name, length('19_csv_parser/') + 1)
    WHERE exercise_name LIKE '19_csv_parser/%';

UPDATE submissions SET exercise_name = '19_rust_fundamentals_quiz'
    WHERE exercise_name = '20_rust_fundamentals_quiz';

UPDATE submissions SET exercise_name = '20_appendix'
    WHERE exercise_name = '21_appendix';
UPDATE submissions SET exercise_name = '20_appendix/' || substr(exercise_name, length('21_appendix/') + 1)
    WHERE exercise_name LIKE '21_appendix/%';
