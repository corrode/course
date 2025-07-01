-- Add content hash to submissions to prevent duplicate storage
-- This helps avoid storing identical submissions multiple times

ALTER TABLE submissions ADD COLUMN content_hash TEXT;

-- Note: Existing submissions will have NULL content_hash
-- They will be treated as unique until resubmitted with the new hash system

-- Create an index on the hash for efficient lookups
CREATE INDEX idx_submissions_hash ON submissions(participant_id, exercise_name, content_hash);