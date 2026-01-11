SELECT *
FROM draft
WHERE paragraph_id = ?
ORDER BY draft_pos NULLS LAST;