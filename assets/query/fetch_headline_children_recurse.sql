WITH RECURSIVE children(p_id) AS (VALUES (?)
                                  UNION ALL
                                  SELECT headlines.id
                                  FROM headlines
                                           LEFT OUTER JOIN children ON headlines.parent_id = children.p_id
                                  WHERE headlines.parent_id = p_id)
SELECT headlines.*
FROM headlines
         LEFT OUTER JOIN orderable_headlines
                         ON headlines.id = orderable_headlines.id
WHERE headlines.id IN (SELECT * FROM children)
ORDER BY parent_id, pos NULLS LAST;