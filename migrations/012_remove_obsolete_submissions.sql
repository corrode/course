-- Remove submissions for exercises that no longer exist in the catalog.
--
-- Migration 010 intentionally retained these rows during the chapter rewrite.
-- They have since been verified against the current catalog and a production
-- snapshot. Keep the cleanup explicit so no valid current key can be removed by
-- a broad numeric-prefix match.
DELETE FROM submissions WHERE exercise_name IN (
    '00_integers/3_number_to_string',
    '00_integers/4_calculate_total_with_tax',
    '03_functions/4_countdown',
    '06_vectors/2_count_items',
    '09_option/2_fallback',
    '16_word_frequencies/4_frequent_words',
    '17_password_validator/4_char_classes',
    '17_password_validator/6_advisor',
    '19_modules_and_visibility/4_settings',
    '19_modules_and_visibility/4_status',
    '20_environment_file_parser/2_parse_line',
    '20_environment_file_parser/3_parse_file',
    '20_environment_file_parser/4_get_var',
    '20_environment_file_parser/5_validate',
    '21_csv_parser/3_simple_line',
    '21_csv_parser/4_quoted_line',
    '21_csv_parser/5_parse_file',
    '21_csv_parser/6_records',
    '3_display_name.md'
);
