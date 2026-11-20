mod boxable;
mod builtin;

use tantivy::tokenizer::TextAnalyzer;

pub use boxable::*;
pub use builtin::*;

pub trait TantivyTokenizer: Default + Send + Sync {
    const TOKENIZER_NAME: &'static str;
    fn new() -> Self {
        Self::default()
    }
    fn text_analyzer(&self) -> TextAnalyzer;
}
