-- Migration: rename chapter directories and remap stored submission keys.
--
-- Seven chapter directories were renamed so the on-disk slug matches the
-- exercise's H1 title (which is what learners see in the dashboard, the
-- chapter picker, and the page header). The `submissions.exercise_name`
-- column stores the old directory name as a foreign-key-ish reference,
-- so existing rows would orphan without this rewrite.
--
-- Safe to run multiple times because each UPDATE only matches rows that
-- still hold the *old* name.

UPDATE submissions SET exercise_name = '01_integer_operations'        WHERE exercise_name = '01_integer_handling';
UPDATE submissions SET exercise_name = '03_http_status_handling'      WHERE exercise_name = '03_enums_and_matching';
UPDATE submissions SET exercise_name = '04_vector_basics'             WHERE exercise_name = '04_vectors_basics';
UPDATE submissions SET exercise_name = '06_tuples_and_destructuring'  WHERE exercise_name = '06_tuples';
UPDATE submissions SET exercise_name = '10_user_account_management'   WHERE exercise_name = '10_structs_and_methods';
UPDATE submissions SET exercise_name = '14_modules_and_visibility'    WHERE exercise_name = '14_modules';
UPDATE submissions SET exercise_name = '16_environment_file_parser'   WHERE exercise_name = '16_env_parser';
