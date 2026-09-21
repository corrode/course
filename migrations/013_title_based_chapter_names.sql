-- Align chapter identifiers with their displayed titles without changing step
-- keys, submission contents, timestamps, or completion flags. Keep previous
-- migrations intact so existing installations can upgrade normally.
CREATE TEMP TABLE chapter_renames (old_name TEXT PRIMARY KEY, new_name TEXT NOT NULL);
INSERT INTO chapter_renames VALUES
    ('00_integers', '00_numbers_in_rust'),
    ('01_strings_and_chars', '01_strings_str_and_chars'),
    ('06_word_count', '06_exercise_break_word_count'),
    ('11_option', '11_option_when_a_value_might_be_missing'),
    ('12_result', '12_result_when_an_operation_might_fail'),
    ('13_question_mark_operator', '13_the_question_mark_operator'),
    ('21_environment_file_parser', '21_parsing_structured_text_and_generics'),
    ('22_csv_parser', '22_state_machines_and_stateful_parsing');

-- Compare the entire chapter component, not LIKE patterns: underscores in the
-- names are literal, and optional chapters such as csv_parser_challenges must
-- not be renamed along with their required counterparts.
UPDATE submissions
SET exercise_name = (
    SELECT new_name || substr(submissions.exercise_name, length(old_name) + 1)
    FROM chapter_renames
    WHERE submissions.exercise_name = old_name
       OR instr(submissions.exercise_name, old_name || '/') = 1
)
WHERE EXISTS (
    SELECT 1 FROM chapter_renames
    WHERE submissions.exercise_name = old_name
       OR instr(submissions.exercise_name, old_name || '/') = 1
);

-- Stored hashes include the old identifier. Submission deduplication also
-- checks exact source for the participant/key, so no source or hash rewrite is
-- needed here.

-- Preserve historical analytics as well. If a UI event already exists under
-- its canonical key, retain the older event unchanged rather than deleting it
-- or failing the upgrade on the per-session uniqueness constraint.
UPDATE OR IGNORE course_events
SET exercise_name = (
    SELECT new_name || substr(course_events.exercise_name, length(old_name) + 1)
    FROM chapter_renames
    WHERE course_events.exercise_name = old_name
       OR instr(course_events.exercise_name, old_name || '/') = 1
)
WHERE EXISTS (
    SELECT 1 FROM chapter_renames
    WHERE course_events.exercise_name = old_name
       OR instr(course_events.exercise_name, old_name || '/') = 1
);

DROP TABLE chapter_renames;
