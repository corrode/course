-- Move the optional challenges after the parser chapter, preserving step keys
-- and saved progress. Match the whole chapter component, not loose prefixes.
UPDATE submissions
SET exercise_name = '23_csv_parser_challenges' ||
    substr(exercise_name, length('22_csv_parser_challenges') + 1)
WHERE exercise_name = '22_csv_parser_challenges'
   OR instr(exercise_name, '22_csv_parser_challenges/') = 1;

-- Preserve historical analytics; retain conflicting per-session events under
-- their old keys rather than deleting them or failing the upgrade.
UPDATE OR IGNORE course_events
SET exercise_name = '23_csv_parser_challenges' ||
    substr(exercise_name, length('22_csv_parser_challenges') + 1)
WHERE exercise_name = '22_csv_parser_challenges'
   OR instr(exercise_name, '22_csv_parser_challenges/') = 1;
