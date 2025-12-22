CREATE VIEW view_deserializable_paragraph AS
SELECT paragraph.*,
       headlines.id           AS h_id,
       headlines.item_id      AS h_item_id,
       headlines.parent_id    AS h_parent_id,
       headlines.headline_pos AS h_headline_pos,
       draft.id               AS d_id,
       draft.paragraph_id     AS d_paragraph_id,
       draft.draft_pos        AS d_draft_pos,
       draft.title            AS d_title,
       draft.body             AS d_body,
       draft.created_at       AS d_created_at,
       draft.updated_at       AS d_updated_at
FROM paragraph
         LEFT OUTER JOIN headlines
                         ON paragraph.headline_id = headlines.id
         LEFT OUTER JOIN draft
                         ON paragraph.accepted_draft_id = draft.id;

CREATE VIEW view_deserializable_item AS
SELECT items.*,
       headlines.id           AS h_id,
       headlines.item_id      AS h_item_id,
       headlines.parent_id    AS h_parent_id,
       headlines.headline_pos AS h_headline_pos,
       view_deserializable_paragraph.d_id,
       view_deserializable_paragraph.d_paragraph_id,
       view_deserializable_paragraph.d_draft_pos,
       view_deserializable_paragraph.d_title,
       view_deserializable_paragraph.d_body,
       view_deserializable_paragraph.d_created_at,
       view_deserializable_paragraph.d_updated_at
FROM items
         LEFT OUTER JOIN view_deserializable_paragraph
                         ON items.id = view_deserializable_paragraph.item_id
         LEFT OUTER JOIN headlines
                         ON (items.item_type = 'headline' AND items.id = headlines.item_id)
                             OR
                            (items.item_type = 'paragraph' AND
                             view_deserializable_paragraph.h_id = headlines.id);

CREATE VIEW view_deserializable_background_reference AS
SELECT background_references_list.*,
       bibliographies.id         AS b_id,
       bibliographies.isbn       AS b_isbn,
       bibliographies.url        AS b_url,
       bibliographies.title      AS b_title,
       bibliographies.detail     AS b_detail,
       bibliographies.author     AS b_author,
       bibliographies.created_at AS b_created_at,
       bibliographies.updated_at AS b_updated_at
FROM background_references_list
         LEFT OUTER JOIN bibliographies
                         ON background_references_list.bibliography_id = bibliographies.id;

CREATE VIEW view_deserializable_item_reference AS
SELECT item_references_list.*,
       bibliographies.id         AS b_id,
       bibliographies.isbn       AS b_isbn,
       bibliographies.url        AS b_url,
       bibliographies.title      AS b_title,
       bibliographies.detail     AS b_detail,
       bibliographies.author     AS b_author,
       bibliographies.created_at AS b_created_at,
       bibliographies.updated_at AS b_updated_at
FROM item_references_list
         LEFT OUTER JOIN bibliographies
                         ON item_references_list.bibliography_id = bibliographies.id;

CREATE VIEW view_deserializable_task_template AS
SELECT task_templates.*,
       task_categories.id                          AS tc_id,
       task_categories.name                        AS tc_name,
       task_categories.autocomplete_paragraph_link AS tc_autocomplete_paragraph_link
FROM task_templates
         LEFT OUTER JOIN task_categories
                         ON task_templates.task_category_id = task_categories.id;

CREATE VIEW view_deserializable_task AS
SELECT tasks.*,
       task_categories.id                          AS tc_id,
       task_categories.name                        AS tc_name,
       task_categories.autocomplete_paragraph_link AS tc_autocomplete_paragraph_link
FROM tasks
         LEFT OUTER JOIN task_categories
                         ON tasks.task_category_id = task_categories.id;

CREATE VIEW view_deserializable_paragraph_link AS
SELECT paragraph_link.*,
       from_p.id                        AS from_p_id,
       from_p.item_id                   AS from_p_item_id,
       from_p.headline_id               AS from_p_headline_id,
       from_p.paragraph_pos             AS from_p_paragraph_pos,
       from_p.accepted_draft_id         AS from_p_accepted_draft_id,
       from_p.h_id                      AS from_h_id,
       from_p.h_item_id                 AS from_h_item_id,
       from_p.h_parent_id               AS from_h_parent_id,
       from_p.h_headline_pos            AS from_h_headline_pos,
       from_p.d_id                      AS from_d_id,
       from_p.d_paragraph_id            AS from_d_paragraph_id,
       from_p.d_draft_pos               AS from_d_draft_pos,
       from_p.d_title                   AS from_d_title,
       from_p.d_body                    AS from_d_body,
       from_p.d_created_at              AS from_d_created_at,
       from_p.d_updated_at              AS from_d_updated_at,
       to_p.id                          AS to_id,
       to_p.item_id                     AS to_item_id,
       to_p.headline_id                 AS to_headline_id,
       to_p.paragraph_pos               AS to_paragraph_pos,
       to_p.accepted_draft_id           AS to_p_accepted_draft_id,
       to_p.h_id                        AS to_h_id,
       to_p.h_item_id                   AS to_h_item_id,
       to_p.h_parent_id                 AS to_h_parent_id,
       to_p.h_headline_pos              AS to_h_headline_pos,
       to_p.d_id                        AS to_d_id,
       to_p.d_paragraph_id              AS to_d_paragraph_id,
       to_p.d_draft_pos                 AS to_d_draft_pos,
       to_p.d_title                     AS to_d_title,
       to_p.d_body                      AS to_d_body,
       to_p.d_created_at                AS to_d_created_at,
       to_p.d_updated_at                AS to_d_updated_at,
       t.id                             AS t_id,
       t.item_id                        AS t_item_id,
       t.task_category_id               AS t_task_category_id,
       t.title                          AS t_title,
       t.detail                         AS t_detail,
       t.is_finished                    AS t_is_finished,
       t.tc_id                          AS tc_id,
       t.tc_name                        AS tc_name,
       t.tc_autocomplete_paragraph_link AS tc_autocomplete_paragraph_link
FROM paragraph_link
         LEFT OUTER JOIN view_deserializable_paragraph AS from_p
                         ON from_paragraph_id = from_p.id
         LEFT OUTER JOIN view_deserializable_paragraph AS to_p
                         ON to_paragraph_id = to_p.id
         LEFT OUTER JOIN view_deserializable_task AS t
                         ON paragraph_link.task_id = t.id;
