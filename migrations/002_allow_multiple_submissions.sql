-- Allow multiple submissions per exercise per participant
-- This removes the unique constraint that prevented multiple submissions

-- Create a new table without the unique constraint
CREATE TABLE IF NOT EXISTS submissions_new (
    id TEXT PRIMARY KEY,
    participant_id TEXT NOT NULL,
    exercise_name TEXT NOT NULL,
    source_code TEXT NOT NULL,
    tests_passed BOOLEAN NOT NULL,
    clippy_passed BOOLEAN DEFAULT FALSE,
    fmt_passed BOOLEAN DEFAULT FALSE,
    submitted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (participant_id) REFERENCES participants (id)
);

-- Copy existing data
INSERT INTO submissions_new SELECT * FROM submissions;

-- Drop the old table
DROP TABLE submissions;

-- Rename the new table
ALTER TABLE submissions_new RENAME TO submissions;