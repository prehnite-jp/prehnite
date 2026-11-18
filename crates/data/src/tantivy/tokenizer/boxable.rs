use crate::tantivy::tokenizer::TantivyTokenizer;
use tantivy::Index;
use tantivy::tokenizer::TextAnalyzer;

pub trait BoxableTantivyTokenizer: Send + Sync {
    fn tokenizer_name(&self) -> &'static str;
    fn text_analyzer(&self) -> Option<TextAnalyzer>;
}

impl<T: TantivyTokenizer> BoxableTantivyTokenizer for T {
    fn tokenizer_name(&self) -> &'static str {
        T::TOKENIZER_NAME
    }

    fn text_analyzer(&self) -> Option<TextAnalyzer> {
        Some(T::text_analyzer(self))
    }
}

impl dyn BoxableTantivyTokenizer {
    pub fn register_to_index(&self, index: &Index) {
        if let Some(tokenizer) = self.text_analyzer() {
            index
                .tokenizers()
                .register(self.tokenizer_name(), tokenizer);
        }
    }
}
