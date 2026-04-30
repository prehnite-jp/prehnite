SELECT *
FROM view_deserializable_item
WHERE item_type = 'headline'
  AND h_parent_id IS NULL
LIMIT ? OFFSET ?;