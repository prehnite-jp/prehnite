INSERT INTO global_default_task_categories(id, name, autocomplete_paragraph_link)
VALUES (1, '伏線', 1),
       (2, '未解説', 1);

INSERT INTO global_default_task_templates(task_category_id, title, detail)
VALUES (1, '伏線を回収する。', '伏線を立てましたが、回収していません。'),
       (2, '詳細を解説する。', '解説する必要がある内容ですが、この場で解説されていません。')