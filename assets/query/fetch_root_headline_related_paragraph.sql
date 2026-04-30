SELECT *
FROM view_deserializable_item
WHERE item_type = 'paragraph'
  AND p_headline_id IN (SELECT id
                        FROM orderable_headlines
                        WHERE parent_id IS NULL
                        ORDER BY pos NULLS LAST
                        LIMIT ? OFFSET ?);