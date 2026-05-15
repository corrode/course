-- Migration 007: add optional team_token to participants.
--
-- Workshop / cohort grouping is captured by writing a free-text label into
-- this column when a participant signs up via /signup/{group_slug}.
-- Public signups (/signup) leave the column NULL.
--
-- The column is intentionally unstructured for now: there is no `groups`
-- table and no foreign key. Admin views can `GROUP BY team_token` to roll
-- up cohorts. We can promote this to a real entity later if group metadata
-- (display name, expiry, seat count) becomes necessary.
ALTER TABLE participants ADD COLUMN team_token TEXT;

-- A partial index keeps cohort lookups fast without indexing the (likely
-- large) tail of NULL rows from public signups.
CREATE INDEX IF NOT EXISTS idx_participants_team_token
    ON participants(team_token)
    WHERE team_token IS NOT NULL;
