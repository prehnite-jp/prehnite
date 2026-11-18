pub mod boxable;
pub mod builtin;

use tantivy::tokenizer::TextAnalyzer;

pub trait TantivyTokenizer: Default + Send + Sync {
    const TOKENIZER_NAME: &'static str;
    fn new() -> Self {
        Self::default()
    }
    fn text_analyzer(&self) -> TextAnalyzer;
}
