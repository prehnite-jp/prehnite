mod crud;

use crate::db::schema::*;
use crate::test_util::{RandomValue, RandomValueVec};
use rand::Rng;
use crate::db::schema::app_global::book_search_api::BookSearchApi;

//noinspection RsSuperTraitIsNotImplemented: supress false positive
impl RandomValue for BackgroundInfo {
    fn random_value() -> Self {
        BackgroundInfo {
            id: 0,
            body: RandomValue::random_value(),
            created_at: Default::default(),
            updated_at: Default::default(),
            references: Default::default(),
        }
    }
}

impl RandomValue for BackgroundReference {
    fn random_value() -> Self {
        BackgroundReference {
            id: 0,
            background_info_id: 0,
            bibliography: Default::default(),
            location: RandomValue::random_value(),
        }
    }
}
//noinspection RsSuperTraitIsNotImplemented: supress false positive
impl RandomValue for Bibliography {
    fn random_value() -> Self {
        Bibliography {
            id: 0,
            isbn: RandomValue::random_value(),
            url: RandomValue::random_value(),
            title: RandomValue::random_value(),
            detail: RandomValue::random_value(),
            authors: Default::default(),
            publisher: Default::default(),
            publication_date: RandomValue::random_value(),
            created_at: Default::default(),
            updated_at: Default::default(),
            tmp_registration_id: None,
        }
    }
}
impl RandomValue for BibliographyAuthor {
    fn random_value() -> Self {
        BibliographyAuthor {
            id: 0,
            name: RandomValue::random_value(),
            memo: RandomValue::random_value(),
        }
    }
}
//noinspection RsSuperTraitIsNotImplemented: supress false positive
impl RandomValue for Draft {
    fn random_value() -> Self {
        Draft {
            id: 0,
            paragraph_id: 0,
            draft_pos: RandomValue::random_value(),
            title: RandomValue::random_value(),
            body: RandomValue::random_value(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}
//noinspection RsSuperTraitIsNotImplemented: supress false positive
impl RandomValue for Headline {
    fn random_value() -> Self {
        Headline {
            id: 0,
            item_id: 0,
            parent_id: Default::default(),
            headline_pos: RandomValue::random_value(),
            children: Default::default(),
            paragraph: Default::default(),
        }
    }
}
impl RandomValue for ItemType {
    fn random_value() -> Self {
        if RandomValue::random_value() {
            ItemType::Headline(RandomValue::random_value())
        } else {
            ItemType::Paragraph(RandomValue::random_value())
        }
    }
}

//noinspection RsSuperTraitIsNotImplemented: supress false positive
impl RandomValue for Item {
    fn random_value() -> Self {
        Item {
            id: 0,
            created_at: 0,
            item_type: if RandomValue::random_value() {
                ItemType::Headline(None)
            } else {
                ItemType::Paragraph(None)
            },
            title: RandomValue::random_value(),
            references: Default::default(),
            tags: Default::default(),
            background_info_list: Default::default(),
            tasks: Default::default(),
        }
    }
}
impl RandomValue for ItemReference {
    fn random_value() -> Self {
        ItemReference {
            id: 0,
            item_id: 0,
            bibliography: Default::default(),
            location: RandomValue::random_value(),
        }
    }
}
//noinspection RsSuperTraitIsNotImplemented: supress false positive
impl RandomValue for Paragraph {
    fn random_value() -> Self {
        Paragraph {
            id: 0,
            item_id: 0,
            headline: Default::default(),
            accepted_draft: Default::default(),
            paragraph_pos: RandomValue::random_value(),
            draft: Default::default(),
            summary: Default::default(),
        }
    }
}
impl RandomValue for ParagraphLink {
    fn random_value() -> Self {
        ParagraphLink {
            id: 0,
            from_paragraph: Default::default(),
            to_paragraph: Default::default(),
            task: Default::default(),
            comment: RandomValue::random_value(),
        }
    }
}
//noinspection RsSuperTraitIsNotImplemented: supress false positive
impl RandomValue for ParagraphSummary {
    fn random_value() -> Self {
        ParagraphSummary {
            id: 0,
            paragraph_id: 0,
            title: RandomValue::random_value(),
            detail: RandomValue::random_value(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}
impl RandomValue for Setting {
    fn random_value() -> Self {
        Setting {
            id: 0,
            setting_key: RandomValue::random_value(),
            setting_value: RandomValue::random_value(),
        }
    }
}
impl RandomValue for Publisher {
    fn random_value() -> Self {
        Publisher {
            id: 0,
            name: RandomValue::random_value(),
            memo: RandomValue::random_value(),
        }
    }
}
impl RandomValue for Tag {
    fn random_value() -> Self {
        Tag {
            id: 0,
            name: RandomValue::random_value(),
            memo: RandomValue::random_value(),
        }
    }
}
impl RandomValue for Task {
    fn random_value() -> Self {
        Task {
            id: 0,
            item_id: 0,
            task_category: Default::default(),
            title: RandomValue::random_value(),
            detail: RandomValue::random_value(),
            is_finished: RandomValue::random_value(),
        }
    }
}
impl RandomValue for TaskCategory {
    fn random_value() -> Self {
        TaskCategory {
            id: 0,
            name: RandomValue::random_value(),
            autocomplete_paragraph_link: RandomValue::random_value(),
        }
    }
}
impl RandomValue for TaskTemplate {
    fn random_value() -> Self {
        TaskTemplate {
            id: 0,
            task_category: Default::default(),
            title: RandomValue::random_value(),
            detail: RandomValue::random_value(),
        }
    }
}
impl RandomValue for BookSearchApi{
    fn random_value() -> Self {
        BookSearchApi{
            id: 0,
            name: RandomValue::random_value(),
            detail: RandomValue::random_value(),
            isbn_url: RandomValue::random_value(),
            text_url: RandomValue::random_value(),
            mapping_script: RandomValue::random_value(),
            is_example: false,
        }
    }
}
