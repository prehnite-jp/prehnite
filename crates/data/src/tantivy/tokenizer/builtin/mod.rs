pub mod lang_ja;

use crate::tantivy::tokenizer::TantivyTokenizer;
use crate::tantivy::tokenizer::boxable::BoxableTantivyTokenizer;
use crate::tantivy::tokenizer::builtin::lang_ja::TokenizerLangJa;
use debug_ignore::DebugIgnore;
use std::fmt::Debug;
use std::sync::Arc;
use tantivy::tokenizer::TextAnalyzer;

#[derive(Default, Clone, Debug)]
pub enum TantivyTokenizers {
    #[default]
    Default,
    Raw,
    EnStem,
    LangJa,
    Custom(DebugIgnore<Arc<dyn BoxableTantivyTokenizer>>),
}
impl BoxableTantivyTokenizer for TantivyTokenizers {
    fn tokenizer_name(&self) -> &'static str {
        match self {
            TantivyTokenizers::Default => "default",
            TantivyTokenizers::Raw => "raw",
            TantivyTokenizers::EnStem => "en_stem",
            TantivyTokenizers::LangJa => TokenizerLangJa::new().tokenizer_name(),
            TantivyTokenizers::Custom(x) => x.tokenizer_name(),
        }
    }

    fn text_analyzer(&self) -> Option<TextAnalyzer> {
        match self {
            TantivyTokenizers::LangJa => {
                BoxableTantivyTokenizer::text_analyzer(&TokenizerLangJa::new())
            }
            TantivyTokenizers::Custom(x) => x.text_analyzer(),
            _ => None,
        }
    }
}
