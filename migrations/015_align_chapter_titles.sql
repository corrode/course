-- Align chapter identifiers with their titles while preserving saved progress.
CREATE TEMP TABLE chapter_renames (old_name TEXT PRIMARY KEY, new_name TEXT NOT NULL);
INSERT INTO chapter_renames VALUES
    ('05_borrowing_and_references', '05_borrowing_and_ownership'),
    ('15_memory_and_ownership', '15_a_primer_on_lifetimes'),
    ('23_csv_parser_challenges', '23_even_more_csv_parsing');

-- Match the whole chapter component, retaining step keys and submission data.
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

-- Keep conflicting per-session events under their old keys rather than losing
-- historical analytics or failing the upgrade.
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
