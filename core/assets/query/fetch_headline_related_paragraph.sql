SELECT *
FROM view_deserializable_item
WHERE item_type = 'paragraph'
  AND p_headline_id = ?
ORDER BY p_paragraph_pos NULLS LAST;