# Course analytics

The course records a small set of first-party events in SQLite's
`course_events` table. The goal is to answer where learners stop or ask for
help without collecting source code, names, URLs, user agents, or arbitrary
client metadata.

## Events

| Event | Recorded when | Result fields |
|---|---|---|
| `chapter_view` | An exercise chapter loads | — |
| `editor_focus` | An editor first receives focus in a browser-tab session | — |
| `hint_opened` | A hint disclosure is first opened | — |
| `solution_revealed` | A full solution is first opened | — |
| `next_chapter_clicked` | The next-chapter CTA is clicked | — |
| `exercise_run` | The server receives a Rust Playground response | result, tests passed/total, duration, first structured Rust error code |

UI events are deduplicated per `(session_id, event_type, exercise_name)`. Runs
are never deduplicated because repeated runs are the primary difficulty signal.
A session ID is a random UUID kept in `sessionStorage`, so it expires with the
browser tab. `participant_id` is nullable for anonymous learners.

Every row includes `course_version` and `git_hash`, allowing reports to avoid
mixing results from incompatible course revisions.

## Example queries

Exercises with the most repeated unsuccessful runs:

```sql
SELECT
    exercise_name,
    COUNT(*) AS runs,
    COUNT(DISTINCT COALESCE(participant_id, session_id)) AS learners,
    ROUND(AVG(result != 'passed') * 100, 1) AS unsuccessful_pct,
    ROUND(AVG(duration_ms)) AS average_duration_ms
FROM course_events
WHERE event_type = 'exercise_run'
GROUP BY exercise_name
HAVING learners >= 3
ORDER BY unsuccessful_pct DESC, runs DESC;
```

Hint and solution usage:

```sql
SELECT
    exercise_name,
    SUM(event_type = 'hint_opened') AS hint_opens,
    SUM(event_type = 'solution_revealed') AS solution_reveals
FROM course_events
WHERE event_type IN ('hint_opened', 'solution_revealed')
GROUP BY exercise_name
ORDER BY solution_reveals DESC, hint_opens DESC;
```

Most common structured compiler errors:

```sql
SELECT diagnostic_code, COUNT(*) AS occurrences,
       COUNT(DISTINCT COALESCE(participant_id, session_id)) AS learners
FROM course_events
WHERE event_type = 'exercise_run' AND diagnostic_code IS NOT NULL
GROUP BY diagnostic_code
ORDER BY occurrences DESC;
```

Chapter activation from view to editor use:

```sql
WITH events_by_chapter AS (
    SELECT *, CASE
        WHEN instr(exercise_name, '/') > 0
        THEN substr(exercise_name, 1, instr(exercise_name, '/') - 1)
        ELSE exercise_name
    END AS chapter
    FROM course_events
)
SELECT
    chapter,
    COUNT(DISTINCT CASE WHEN event_type = 'chapter_view' THEN session_id END) AS views,
    COUNT(DISTINCT CASE WHEN event_type = 'editor_focus' THEN session_id END) AS editors
FROM events_by_chapter
WHERE event_type IN ('chapter_view', 'editor_focus')
GROUP BY chapter;
```

For reports, prefer a read-only SQLite backup rather than querying the live file.
SQLite's backup API or `.backup` command produces a consistent snapshot without
interrupting the server.
