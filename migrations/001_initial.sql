-- Initial database schema for corrode course system

-- Participants table for storing registered users
CREATE TABLE IF NOT EXISTS participants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Submissions table for tracking exercise submissions
CREATE TABLE IF NOT EXISTS submissions (
    id TEXT PRIMARY KEY,
    participant_id TEXT NOT NULL,
    exercise_name TEXT NOT NULL,
    source_code TEXT NOT NULL,
    tests_passed BOOLEAN NOT NULL,
    clippy_passed BOOLEAN DEFAULT FALSE,
    fmt_passed BOOLEAN DEFAULT FALSE,
    submitted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (participant_id) REFERENCES participants (id),
    UNIQUE(participant_id, exercise_name)
);