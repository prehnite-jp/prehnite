use crate::db::schema::{BibliographyAuthor, ItemType};

impl Default for ItemType {
    fn default() -> Self {
        Self::Headline(None)
    }
}
