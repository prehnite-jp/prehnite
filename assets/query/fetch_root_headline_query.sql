SELECT *
FROM view_deserializable_item
WHERE item_type = 'headline'
  AND h_parent_id IS NULL
LIMIT ? OFFSET ?
ORDER BY h_headline_pos NULLS LAST;