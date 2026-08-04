-- Privacy-conscious course analytics.
--
-- Run outcomes are written by the server after it receives the Rust Playground
-- response. UI events use a small server-side allowlist. No source code,
-- participant names, URLs, user agents, or arbitrary metadata are stored here.
CREATE TABLE course_events (
    id TEXT PRIMARY KEY,
    participant_id TEXT,
    session_id TEXT NOT NULL CHECK(length(session_id) BETWEEN 1 AND 64),
    event_type TEXT NOT NULL CHECK(event_type IN (
        'chapter_view',
        'editor_focus',
        'hint_opened',
        'solution_revealed',
        'next_chapter_clicked',
        'exercise_run'
    )),
    exercise_name TEXT,
    result TEXT CHECK(result IS NULL OR result IN (
        'passed',
        'test_failed',
        'compile_failed',
        'upstream_failed'
    )),
    tests_passed INTEGER,
    tests_total INTEGER,
    duration_ms INTEGER CHECK(duration_ms IS NULL OR duration_ms >= 0),
    diagnostic_code TEXT,
    course_version TEXT NOT NULL,
    git_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (participant_id) REFERENCES participants(id) ON DELETE CASCADE
);

CREATE INDEX idx_course_events_created_at ON course_events(created_at);
CREATE INDEX idx_course_events_participant ON course_events(participant_id, created_at);
CREATE INDEX idx_course_events_exercise ON course_events(exercise_name, event_type, created_at);
CREATE INDEX idx_course_events_session ON course_events(session_id, created_at);
CREATE UNIQUE INDEX idx_course_events_ui_once_per_session
    ON course_events(session_id, event_type, exercise_name)
    WHERE event_type != 'exercise_run';

-- Removed or renamed exercises deliberately left behind by migration 010.
-- They no longer exist in the current catalog and otherwise pollute progress
-- analysis. Keep the cleanup explicit so no valid current key can be removed by
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
