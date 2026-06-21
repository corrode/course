-- Migration 010: chapter restructure (ownership spiral + reorder).
--
-- Rewrites `submissions.exercise_name` so existing learners keep their
-- progress under the new chapter layout. Derived from the real key set in
-- the production snapshot (`analysis/course.db`, 58 participants), which
-- predates the recent content PRs, so this maps each old key straight to
-- its final home: the chapter reorder, the ownership split into Moves &
-- Copy + Borrowing, the `?` move, and the per-chapter step renumberings
-- (cut exercises, etc.).
--
-- Every statement is an exact-match UPDATE, so the order doesn't matter
-- and no row can be rewritten twice. Nothing is deleted: rows for
-- exercises that were removed (e.g. `count_items`, `countdown`,
-- `fallback`, `frequent_words`, `char_classes`, `advisor`, `status`,
-- `records`, `number_to_string`) are left untouched. They no longer map
-- to a chapter, so they're ignored by the dashboard, but the data stays.
--
-- Idempotent: re-running matches nothing because the old keys are gone.
--
-- 00_integers and 01_strings_and_chars keep their prefixes and surviving
-- step names, so they need no statements.

-- 02_conditionals_and_loops -> 03
UPDATE submissions SET exercise_name = '03_conditionals_and_loops/3_ferris_mood'  WHERE exercise_name = '02_conditionals_and_loops/3_ferris_mood';
UPDATE submissions SET exercise_name = '03_conditionals_and_loops/4_factorial'    WHERE exercise_name = '02_conditionals_and_loops/4_factorial';
UPDATE submissions SET exercise_name = '03_conditionals_and_loops/5_count_evens'  WHERE exercise_name = '02_conditionals_and_loops/5_count_evens';
UPDATE submissions SET exercise_name = '03_conditionals_and_loops/6_digit_count'  WHERE exercise_name = '02_conditionals_and_loops/6_digit_count';

-- 03_functions -> 04 (countdown was cut; sum_to and cap_at renumbered)
UPDATE submissions SET exercise_name = '04_functions/3_stray_semicolon' WHERE exercise_name = '03_functions/3_stray_semicolon';
UPDATE submissions SET exercise_name = '04_functions/4_sum_to'          WHERE exercise_name = '03_functions/5_sum_to';
UPDATE submissions SET exercise_name = '04_functions/5_cap_at'          WHERE exercise_name = '03_functions/6_cap_at';

-- 04_word_count -> 06
UPDATE submissions SET exercise_name = '06_word_count/2_word_count'   WHERE exercise_name = '04_word_count/2_word_count';
UPDATE submissions SET exercise_name = '06_word_count/3_char_count'   WHERE exercise_name = '04_word_count/3_char_count';
UPDATE submissions SET exercise_name = '06_word_count/4_longest_word' WHERE exercise_name = '04_word_count/4_longest_word';

-- 05_enums_and_pattern_matching -> 07
UPDATE submissions SET exercise_name = '07_enums_and_pattern_matching/2_status_code'  WHERE exercise_name = '05_enums_and_pattern_matching/2_status_code';
UPDATE submissions SET exercise_name = '07_enums_and_pattern_matching/3_should_retry' WHERE exercise_name = '05_enums_and_pattern_matching/3_should_retry';

-- 06_vectors -> 08 (count_items cut; add_item/contains_item/shopping_list renumbered)
UPDATE submissions SET exercise_name = '08_vectors/2_add_item'             WHERE exercise_name = '06_vectors/3_add_item';
UPDATE submissions SET exercise_name = '08_vectors/3_contains_item'        WHERE exercise_name = '06_vectors/4_contains_item';
UPDATE submissions SET exercise_name = '08_vectors/4_create_shopping_list' WHERE exercise_name = '06_vectors/5_create_shopping_list';

-- 07_hashmaps -> 09
UPDATE submissions SET exercise_name = '09_hashmaps/2_create_default_config' WHERE exercise_name = '07_hashmaps/2_create_default_config';
UPDATE submissions SET exercise_name = '09_hashmaps/3_set_config_value'      WHERE exercise_name = '07_hashmaps/3_set_config_value';
UPDATE submissions SET exercise_name = '09_hashmaps/4_get_config_value'      WHERE exercise_name = '07_hashmaps/4_get_config_value';
UPDATE submissions SET exercise_name = '09_hashmaps/5_count_words'           WHERE exercise_name = '07_hashmaps/5_count_words';

-- 08_tuples_and_destructuring -> 10
UPDATE submissions SET exercise_name = '10_tuples_and_destructuring/2_get_user_info'           WHERE exercise_name = '08_tuples_and_destructuring/2_get_user_info';
UPDATE submissions SET exercise_name = '10_tuples_and_destructuring/3_rectangle_measurements'  WHERE exercise_name = '08_tuples_and_destructuring/3_rectangle_measurements';
UPDATE submissions SET exercise_name = '10_tuples_and_destructuring/4_get_first_name'          WHERE exercise_name = '08_tuples_and_destructuring/4_get_first_name';
UPDATE submissions SET exercise_name = '10_tuples_and_destructuring/5_swap_values'             WHERE exercise_name = '08_tuples_and_destructuring/5_swap_values';

-- 09_option -> 11 (fallback became prose; transform/first_char/find_user renumbered)
UPDATE submissions SET exercise_name = '11_option/2_transform'  WHERE exercise_name = '09_option/3_transform';
UPDATE submissions SET exercise_name = '11_option/3_first_char' WHERE exercise_name = '09_option/4_first_char';
UPDATE submissions SET exercise_name = '11_option/4_find_user'  WHERE exercise_name = '09_option/5_find_user';

-- 10_result -> 12
UPDATE submissions SET exercise_name = '12_result/2_safe_divide'      WHERE exercise_name = '10_result/2_safe_divide';
UPDATE submissions SET exercise_name = '12_result/3_read_config'      WHERE exercise_name = '10_result/3_read_config';
UPDATE submissions SET exercise_name = '12_result/4_validate_email'   WHERE exercise_name = '10_result/4_validate_email';
UPDATE submissions SET exercise_name = '12_result/5_parse_percentage' WHERE exercise_name = '10_result/5_parse_percentage';

-- 11_ownership_and_borrowing -> split into Moves & Copy + Borrowing
UPDATE submissions SET exercise_name = '02_moves_and_copy/2_take_ownership'           WHERE exercise_name = '11_ownership_and_borrowing/2_take_ownership';
UPDATE submissions SET exercise_name = '05_borrowing_and_references/2_borrow_string'  WHERE exercise_name = '11_ownership_and_borrowing/3_borrow_string';
UPDATE submissions SET exercise_name = '05_borrowing_and_references/3_mutate_string'  WHERE exercise_name = '11_ownership_and_borrowing/4_mutate_string';
UPDATE submissions SET exercise_name = '05_borrowing_and_references/4_experiments'    WHERE exercise_name = '11_ownership_and_borrowing/5_experiments';

-- 12_structs_and_methods -> 14
UPDATE submissions SET exercise_name = '14_structs_and_methods/2_new'                 WHERE exercise_name = '12_structs_and_methods/2_new';
UPDATE submissions SET exercise_name = '14_structs_and_methods/3_display_name'        WHERE exercise_name = '12_structs_and_methods/3_display_name';
UPDATE submissions SET exercise_name = '14_structs_and_methods/4_record_login'        WHERE exercise_name = '12_structs_and_methods/4_record_login';
UPDATE submissions SET exercise_name = '14_structs_and_methods/5_can_access_premium'  WHERE exercise_name = '12_structs_and_methods/5_can_access_premium';

-- 13_traits -> 16
UPDATE submissions SET exercise_name = '16_traits/2_display_temperature' WHERE exercise_name = '13_traits/2_display_temperature';
UPDATE submissions SET exercise_name = '16_traits/3_describable'         WHERE exercise_name = '13_traits/3_describable';
UPDATE submissions SET exercise_name = '16_traits/4_logger'             WHERE exercise_name = '13_traits/4_logger';
UPDATE submissions SET exercise_name = '16_traits/5_validate'           WHERE exercise_name = '13_traits/5_validate';

-- 14_smart_pointers -> 23 (now a bonus chapter at the end)
UPDATE submissions SET exercise_name = '23_smart_pointers/2_boxed_sum' WHERE exercise_name = '14_smart_pointers/2_boxed_sum';
UPDATE submissions SET exercise_name = '23_smart_pointers/3_expr_tree' WHERE exercise_name = '14_smart_pointers/3_expr_tree';
UPDATE submissions SET exercise_name = '23_smart_pointers/4_pipeline'  WHERE exercise_name = '14_smart_pointers/4_pipeline';

-- 15_iterators -> 17
UPDATE submissions SET exercise_name = '17_iterators/3_sum'              WHERE exercise_name = '15_iterators/3_sum';
UPDATE submissions SET exercise_name = '17_iterators/4_map'              WHERE exercise_name = '15_iterators/4_map';
UPDATE submissions SET exercise_name = '17_iterators/5_filter'           WHERE exercise_name = '15_iterators/5_filter';
UPDATE submissions SET exercise_name = '17_iterators/6_filter_to_string' WHERE exercise_name = '15_iterators/6_filter_to_string';

-- 16_word_frequencies -> 18 (frequent_words cut; text_stats renumbered)
UPDATE submissions SET exercise_name = '18_word_frequencies/2_count_words'      WHERE exercise_name = '16_word_frequencies/2_count_words';
UPDATE submissions SET exercise_name = '18_word_frequencies/3_most_common_word' WHERE exercise_name = '16_word_frequencies/3_most_common_word';
UPDATE submissions SET exercise_name = '18_word_frequencies/4_text_stats'       WHERE exercise_name = '16_word_frequencies/5_text_stats';

-- 17_password_validator -> 19 (char_classes + advisor cut; generate/validate renumbered)
UPDATE submissions SET exercise_name = '19_password_validator/3_is_strong' WHERE exercise_name = '17_password_validator/3_is_strong';
UPDATE submissions SET exercise_name = '19_password_validator/4_generate'  WHERE exercise_name = '17_password_validator/5_generate';
UPDATE submissions SET exercise_name = '19_password_validator/5_validate'  WHERE exercise_name = '17_password_validator/7_validate';

-- 18_question_mark_operator -> 13 (sum_numbers_in_file reworked to sum_numbers)
UPDATE submissions SET exercise_name = '13_question_mark_operator/2_add_parsed_numbers' WHERE exercise_name = '18_question_mark_operator/2_add_parsed_numbers';
UPDATE submissions SET exercise_name = '13_question_mark_operator/3_count_file_lines'   WHERE exercise_name = '18_question_mark_operator/3_count_file_lines';
UPDATE submissions SET exercise_name = '13_question_mark_operator/4_sum_numbers'        WHERE exercise_name = '18_question_mark_operator/4_sum_numbers_in_file';

-- 19_modules_and_visibility -> 20 (status cut; settings renumbered)
UPDATE submissions SET exercise_name = '20_modules_and_visibility/3_calculate' WHERE exercise_name = '19_modules_and_visibility/3_calculate';
UPDATE submissions SET exercise_name = '20_modules_and_visibility/4_settings'  WHERE exercise_name = '19_modules_and_visibility/5_settings';

-- 20_environment_file_parser -> 21
UPDATE submissions SET exercise_name = '21_environment_file_parser/2_parse_line' WHERE exercise_name = '20_environment_file_parser/2_parse_line';
UPDATE submissions SET exercise_name = '21_environment_file_parser/3_parse_file' WHERE exercise_name = '20_environment_file_parser/3_parse_file';
UPDATE submissions SET exercise_name = '21_environment_file_parser/4_get_var'    WHERE exercise_name = '20_environment_file_parser/4_get_var';
UPDATE submissions SET exercise_name = '21_environment_file_parser/5_validate'   WHERE exercise_name = '20_environment_file_parser/5_validate';

-- 21_csv_parser -> 22 (records cut)
UPDATE submissions SET exercise_name = '22_csv_parser/3_simple_line' WHERE exercise_name = '21_csv_parser/3_simple_line';
UPDATE submissions SET exercise_name = '22_csv_parser/4_quoted_line' WHERE exercise_name = '21_csv_parser/4_quoted_line';
UPDATE submissions SET exercise_name = '22_csv_parser/5_parse_file'  WHERE exercise_name = '21_csv_parser/5_parse_file';
