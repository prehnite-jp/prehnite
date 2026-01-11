SELECT *
FROM view_deserializable_task
WHERE item_id = ?
ORDER BY task_pos NULLS LAST;