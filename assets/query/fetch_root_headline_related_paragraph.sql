SELECT *
FROM view_deserializable_item
         LEFT OUTER JOIN orderable_paragraph
                         ON view_deserializable_item.p_id = orderable_paragraph.id
WHERE item_type = 'paragraph'
  AND p_headline_id IN (SELECT id
                        FROM orderable_headlines
                        WHERE parent_id IS NULL
                        ORDER BY pos NULLS LAST
                        LIMIT ? OFFSET ?)
ORDER BY pos NULLS LAST;