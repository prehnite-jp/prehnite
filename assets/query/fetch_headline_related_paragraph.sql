SELECT *
FROM view_deserializable_item
         LEFT OUTER JOIN orderable_paragraph
                         ON view_deserializable_item.id = orderable_paragraph.id
WHERE item_type = 'paragraph'
  AND p_headline_id = ?
ORDER BY pos NULLS LAST;