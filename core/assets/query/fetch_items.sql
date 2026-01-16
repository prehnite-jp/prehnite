SELECT *
FROM view_deserializable_item
WHERE p_headline_id IN (SELECT *
                        FROM headlines
                        WHERE parent_id IS NULL
                        LIMIT ? OFFSET ?)