SELECT paragraph_summaries.*
FROM paragraph_summaries
         LEFT OUTER JOIN orderable_paragraph_summaries
                         ON paragraph_summaries.id = orderable_paragraph_summaries.id
WHERE paragraph_summaries.paragraph_id = ?
ORDER BY pos NULLS LAST;