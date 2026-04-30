SELECT view_deserializable_task.*
FROM view_deserializable_task
         LEFT OUTER JOIN orderable_tasks
                         ON view_deserializable_task.id = orderable_tasks.id
WHERE view_deserializable_task.item_id = ?
ORDER BY pos NULLS LAST;