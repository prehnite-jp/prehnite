CREATE VIEW orderable_headlines AS
WITH RECURSIVE cte(id, p_id, next, pos) AS (SELECT id, parent_id, next_headline_id, 0
                                            FROM headlines
                                            WHERE headlines.prev_headline_id IS NULL
                                            UNION ALL
                                            SELECT headlines.id,
                                                   parent_id,
                                                   headlines.next_headline_id,
                                                   cte.pos + 1
                                            FROM headlines,
                                                 cte
                                            WHERE headlines.id = cte.next
                                              AND coalesce(headlines.parent_id = cte.p_id, headlines.parent_id IS NULL))
SELECT headlines.*, cte.pos
FROM headlines
         LEFT OUTER JOIN cte ON headlines.id == cte.id;

CREATE VIEW orderable_paragraph AS
WITH RECURSIVE cte(id, p_id, next, pos) AS (SELECT id, headline_id, next_paragraph_id, 0
                                            FROM paragraph
                                            WHERE paragraph.prev_paragraph_id IS NULL
                                            UNION ALL
                                            SELECT paragraph.id,
                                                   headline_id,
                                                   paragraph.next_paragraph_id,
                                                   cte.pos + 1
                                            FROM paragraph,
                                                 cte
                                            WHERE paragraph.id = cte.next
                                              AND paragraph.headline_id = cte.p_id)
SELECT paragraph.*, cte.pos
FROM paragraph
         LEFT OUTER JOIN cte ON paragraph.id == cte.id;

CREATE VIEW orderable_draft AS
WITH RECURSIVE cte(id, p_id, next, pos) AS (SELECT id, paragraph_id, next_draft_id, 0
                                            FROM draft
                                            WHERE draft.prev_draft_id IS NULL
                                            UNION ALL
                                            SELECT draft.id,
                                                   paragraph_id,
                                                   draft.next_draft_id,
                                                   cte.pos + 1
                                            FROM draft,
                                                 cte
                                            WHERE draft.id = cte.next
                                              AND draft.paragraph_id = cte.p_id)
SELECT draft.*, cte.pos
FROM draft
         LEFT OUTER JOIN cte ON draft.id == cte.id;

CREATE VIEW orderable_paragraph_summaries AS
WITH RECURSIVE cte(id, p_id, next, pos) AS (SELECT id, paragraph_id, next_summary_id, 0
                                            FROM paragraph_summaries
                                            WHERE paragraph_summaries.prev_summary_id IS NULL
                                            UNION ALL
                                            SELECT paragraph_summaries.id,
                                                   paragraph_id,
                                                   paragraph_summaries.next_summary_id,
                                                   cte.pos + 1
                                            FROM paragraph_summaries,
                                                 cte
                                            WHERE paragraph_summaries.id = cte.next
                                              AND paragraph_summaries.paragraph_id = cte.p_id)
SELECT paragraph_summaries.*, cte.pos
FROM paragraph_summaries
         LEFT OUTER JOIN cte ON paragraph_summaries.id == cte.id;

CREATE VIEW orderable_tasks AS
WITH RECURSIVE cte(id, p_id, next, pos) AS (SELECT id, item_id, next_task_id, 0
                                            FROM tasks
                                            WHERE tasks.prev_task_id IS NULL
                                            UNION ALL
                                            SELECT tasks.id, item_id, tasks.next_task_id, cte.pos + 1
                                            FROM tasks,
                                                 cte
                                            WHERE tasks.id = cte.next
                                              AND tasks.item_id = cte.p_id)
SELECT tasks.*, cte.pos
FROM tasks
         LEFT OUTER JOIN cte ON tasks.id == cte.id;