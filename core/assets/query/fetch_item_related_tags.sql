SELECT tags.*
FROM tags
         LEFT OUTER JOIN rel_tag_and_item ON tags.id = rel_tag_and_item.tag_id
WHERE item_id = ?;