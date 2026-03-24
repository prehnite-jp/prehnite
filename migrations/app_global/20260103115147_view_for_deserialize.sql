CREATE VIEW view_deserializable_task_template AS
SELECT task_templates.*,
       task_categories.id                          AS tc_id,
       task_categories.name                        AS tc_name,
       task_categories.autocomplete_paragraph_link AS tc_autocomplete_paragraph_link
FROM task_templates
         LEFT OUTER JOIN task_categories
                         ON task_templates.task_category_id = task_categories.id;

CREATE VIEW view_deserializable_bibliographies AS
SELECT bibliographies.*,
       publishers.id   AS pub_id,
       publishers.name AS pub_name,
       publishers.memo AS pub_memo
FROM main.bibliographies
         LEFT OUTER JOIN main.publishers
                         ON bibliographies.publisher_id = publishers.id;
