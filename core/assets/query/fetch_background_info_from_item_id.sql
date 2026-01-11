SELECT background_info.*
FROM background_info
         LEFT OUTER JOIN rel_background_and_item
                         ON background_info.id = rel_background_and_item.background_info_id
WHERE item_id = ?;