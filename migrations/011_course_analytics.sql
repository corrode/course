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
        'no_tests',
        'ran',
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
