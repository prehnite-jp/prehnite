SELECT *
FROM main.rel_bibliography_authors
         LEFT OUTER JOIN main.bibliography_authors author
                         ON rel_bibliography_authors.bibliography_author_id = author.id
WHERE rel_bibliography_authors.bibliography_id = ?;