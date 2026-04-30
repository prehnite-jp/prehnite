SELECT draft.*
FROM draft
         LEFT OUTER JOIN orderable_draft
                         ON draft.id = orderable_draft.id
WHERE draft.paragraph_id = ?
ORDER BY pos NULLS LAST;