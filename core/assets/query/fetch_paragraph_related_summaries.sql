SELECT *
FROM paragraph_summaries
WHERE paragraph_id = ?
ORDER BY summary_pos NULLS LAST;