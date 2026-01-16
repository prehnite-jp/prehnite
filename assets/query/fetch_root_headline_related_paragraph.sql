SELECT *
FROM view_deserializable_item
WHERE item_type = 'paragraph'
  AND p_headline_id IN (SELECT id
                        FROM headlines
                        WHERE parent_id IS NULL
                        ORDER BY headline_pos NULLS LAST
                        LIMIT ? OFFSET ?)
ORDER BY p_paragraph_pos NULLS LAST;