SELECT *
FROM view_deserializable_item
         LEFT OUTER JOIN orderable_headlines
                         ON view_deserializable_item.h_id = orderable_headlines.id
WHERE item_type = 'headline'
  AND h_parent_id IS NULL
ORDER BY pos NULLS LAST
LIMIT ? OFFSET ?;