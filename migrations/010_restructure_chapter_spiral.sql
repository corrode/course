-- Migration 010: chapter restructure (ownership spiral + reorder).
--
-- The course was reordered and ownership is now taught as a spiral. This
-- rewrites `submissions.exercise_name` so the ~existing learners keep
-- their progress against the new on-disk chapter layout.
--
-- `exercise_name` is `<chapter>` (legacy single-step) or `<chapter>/<step>`
-- (multi-step, where `<step>` is `<order>_<slug>`). The chapter directory
-- prefix is part of the key, and almost every prefix changed here, so
-- unmapped rows would orphan and show as "not done".
--
-- Unlike migration 009, this one preserves data: nothing is deleted. Each
-- statement matches only rows that still hold the *old* name, so it's
-- idempotent and safe to re-run. Every chapter slug is unique, so the
-- order of statements does not matter (no statement can re-match a row
-- another statement already rewrote).
--
-- Assumption: the production DB currently holds the *current `main`*
-- chapter names (e.g. `06_vectors/2_add_item`, `18_question_mark_operator/
-- 2_add_parsed_numbers`). Verify before deploying with:
--   SELECT DISTINCT exercise_name FROM submissions ORDER BY 1;
-- Any row whose name isn't matched below is simply left untouched (it
-- won't map to a chapter, but no data is lost).
--
-- 00_integers and 01_strings_and_chars keep their prefixes (not listed).

-- ---------------------------------------------------------------------
-- Ownership spiral: the old `11_ownership_and_borrowing` chapter is split.
-- Its move exercise goes to the new Moves & Copy chapter; its borrow
-- exercises go to the new Borrowing & references chapter (renumbered).
-- The consolidation chapter (`15_memory_and_ownership`) is prose-only, so
-- nothing maps to it.
-- ---------------------------------------------------------------------
UPDATE submissions SET exercise_name = '02_moves_and_copy/2_take_ownership'
    WHERE exercise_name = '11_ownership_and_borrowing/2_take_ownership';
UPDATE submissions SET exercise_name = '05_borrowing_and_references/2_borrow_string'
    WHERE exercise_name = '11_ownership_and_borrowing/3_borrow_string';
UPDATE submissions SET exercise_name = '05_borrowing_and_references/3_mutate_string'
    WHERE exercise_name = '11_ownership_and_borrowing/4_mutate_string';
UPDATE submissions SET exercise_name = '05_borrowing_and_references/4_experiments'
    WHERE exercise_name = '11_ownership_and_borrowing/5_experiments';

-- ---------------------------------------------------------------------
-- The `?` operator moves up to sit right after Result. Its multi-error
-- file exercise was reworked into a parse-only `4_sum_numbers`.
-- ---------------------------------------------------------------------
UPDATE submissions SET exercise_name = '13_question_mark_operator'
    WHERE exercise_name = '18_question_mark_operator';
UPDATE submissions SET exercise_name = '13_question_mark_operator/' || substr(exercise_name, length('18_question_mark_operator/') + 1)
    WHERE exercise_name LIKE '18_question_mark_operator/%';
UPDATE submissions SET exercise_name = '13_question_mark_operator/4_sum_numbers'
    WHERE exercise_name = '13_question_mark_operator/4_sum_numbers_in_file';

-- ---------------------------------------------------------------------
-- Straight prefix shifts. Two statements per chapter cover the legacy
-- single-step key and the multi-step `<chapter>/<step>` key.
-- ---------------------------------------------------------------------

-- 02_conditionals_and_loops -> 03
UPDATE submissions SET exercise_name = '03_conditionals_and_loops'
    WHERE exercise_name = '02_conditionals_and_loops';
UPDATE submissions SET exercise_name = '03_conditionals_and_loops/' || substr(exercise_name, length('02_conditionals_and_loops/') + 1)
    WHERE exercise_name LIKE '02_conditionals_and_loops/%';

-- 03_functions -> 04
UPDATE submissions SET exercise_name = '04_functions'
    WHERE exercise_name = '03_functions';
UPDATE submissions SET exercise_name = '04_functions/' || substr(exercise_name, length('03_functions/') + 1)
    WHERE exercise_name LIKE '03_functions/%';

-- 04_word_count -> 06
UPDATE submissions SET exercise_name = '06_word_count'
    WHERE exercise_name = '04_word_count';
UPDATE submissions SET exercise_name = '06_word_count/' || substr(exercise_name, length('04_word_count/') + 1)
    WHERE exercise_name LIKE '04_word_count/%';

-- 05_enums_and_pattern_matching -> 07
UPDATE submissions SET exercise_name = '07_enums_and_pattern_matching'
    WHERE exercise_name = '05_enums_and_pattern_matching';
UPDATE submissions SET exercise_name = '07_enums_and_pattern_matching/' || substr(exercise_name, length('05_enums_and_pattern_matching/') + 1)
    WHERE exercise_name LIKE '05_enums_and_pattern_matching/%';

-- 06_vectors -> 08
UPDATE submissions SET exercise_name = '08_vectors'
    WHERE exercise_name = '06_vectors';
UPDATE submissions SET exercise_name = '08_vectors/' || substr(exercise_name, length('06_vectors/') + 1)
    WHERE exercise_name LIKE '06_vectors/%';

-- 07_hashmaps -> 09
UPDATE submissions SET exercise_name = '09_hashmaps'
    WHERE exercise_name = '07_hashmaps';
UPDATE submissions SET exercise_name = '09_hashmaps/' || substr(exercise_name, length('07_hashmaps/') + 1)
    WHERE exercise_name LIKE '07_hashmaps/%';

-- 08_tuples_and_destructuring -> 10
UPDATE submissions SET exercise_name = '10_tuples_and_destructuring'
    WHERE exercise_name = '08_tuples_and_destructuring';
UPDATE submissions SET exercise_name = '10_tuples_and_destructuring/' || substr(exercise_name, length('08_tuples_and_destructuring/') + 1)
    WHERE exercise_name LIKE '08_tuples_and_destructuring/%';

-- 09_option -> 11
UPDATE submissions SET exercise_name = '11_option'
    WHERE exercise_name = '09_option';
UPDATE submissions SET exercise_name = '11_option/' || substr(exercise_name, length('09_option/') + 1)
    WHERE exercise_name LIKE '09_option/%';

-- 10_result -> 12
UPDATE submissions SET exercise_name = '12_result'
    WHERE exercise_name = '10_result';
UPDATE submissions SET exercise_name = '12_result/' || substr(exercise_name, length('10_result/') + 1)
    WHERE exercise_name LIKE '10_result/%';

-- 12_structs_and_methods -> 14
UPDATE submissions SET exercise_name = '14_structs_and_methods'
    WHERE exercise_name = '12_structs_and_methods';
UPDATE submissions SET exercise_name = '14_structs_and_methods/' || substr(exercise_name, length('12_structs_and_methods/') + 1)
    WHERE exercise_name LIKE '12_structs_and_methods/%';

-- 13_traits -> 16
UPDATE submissions SET exercise_name = '16_traits'
    WHERE exercise_name = '13_traits';
UPDATE submissions SET exercise_name = '16_traits/' || substr(exercise_name, length('13_traits/') + 1)
    WHERE exercise_name LIKE '13_traits/%';

-- 14_smart_pointers -> 23 (now a bonus chapter at the end)
UPDATE submissions SET exercise_name = '23_smart_pointers'
    WHERE exercise_name = '14_smart_pointers';
UPDATE submissions SET exercise_name = '23_smart_pointers/' || substr(exercise_name, length('14_smart_pointers/') + 1)
    WHERE exercise_name LIKE '14_smart_pointers/%';

-- 15_iterators -> 17
UPDATE submissions SET exercise_name = '17_iterators'
    WHERE exercise_name = '15_iterators';
UPDATE submissions SET exercise_name = '17_iterators/' || substr(exercise_name, length('15_iterators/') + 1)
    WHERE exercise_name LIKE '15_iterators/%';

-- 16_word_frequencies -> 18
UPDATE submissions SET exercise_name = '18_word_frequencies'
    WHERE exercise_name = '16_word_frequencies';
UPDATE submissions SET exercise_name = '18_word_frequencies/' || substr(exercise_name, length('16_word_frequencies/') + 1)
    WHERE exercise_name LIKE '16_word_frequencies/%';

-- 17_password_validator -> 19 (bonus)
UPDATE submissions SET exercise_name = '19_password_validator'
    WHERE exercise_name = '17_password_validator';
UPDATE submissions SET exercise_name = '19_password_validator/' || substr(exercise_name, length('17_password_validator/') + 1)
    WHERE exercise_name LIKE '17_password_validator/%';

-- 19_modules_and_visibility -> 20
UPDATE submissions SET exercise_name = '20_modules_and_visibility'
    WHERE exercise_name = '19_modules_and_visibility';
UPDATE submissions SET exercise_name = '20_modules_and_visibility/' || substr(exercise_name, length('19_modules_and_visibility/') + 1)
    WHERE exercise_name LIKE '19_modules_and_visibility/%';

-- 20_environment_file_parser -> 21
UPDATE submissions SET exercise_name = '21_environment_file_parser'
    WHERE exercise_name = '20_environment_file_parser';
UPDATE submissions SET exercise_name = '21_environment_file_parser/' || substr(exercise_name, length('20_environment_file_parser/') + 1)
    WHERE exercise_name LIKE '20_environment_file_parser/%';

-- 21_csv_parser -> 22
UPDATE submissions SET exercise_name = '22_csv_parser'
    WHERE exercise_name = '21_csv_parser';
UPDATE submissions SET exercise_name = '22_csv_parser/' || substr(exercise_name, length('21_csv_parser/') + 1)
    WHERE exercise_name LIKE '21_csv_parser/%';

-- 22_rust_fundamentals_quiz -> 24 (single-step)
UPDATE submissions SET exercise_name = '24_rust_fundamentals_quiz'
    WHERE exercise_name = '22_rust_fundamentals_quiz';
UPDATE submissions SET exercise_name = '24_rust_fundamentals_quiz/' || substr(exercise_name, length('22_rust_fundamentals_quiz/') + 1)
    WHERE exercise_name LIKE '22_rust_fundamentals_quiz/%';

-- 23_appendix -> 25 (prose; included for completeness)
UPDATE submissions SET exercise_name = '25_appendix'
    WHERE exercise_name = '23_appendix';
UPDATE submissions SET exercise_name = '25_appendix/' || substr(exercise_name, length('23_appendix/') + 1)
    WHERE exercise_name LIKE '23_appendix/%';
