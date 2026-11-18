use lindera::dictionary::load_dictionary;
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera_tantivy::tokenizer::LinderaTokenizer;
use tantivy::tokenizer::TextAnalyzer;
use crate::tantivy::tokenizer::TantivyTokenizer;

#[derive(Default)]
pub struct TokenizerLangJa;

impl TantivyTokenizer for TokenizerLangJa {
    const TOKENIZER_NAME: &'static str = "lang_ja";

    fn text_analyzer(&self) -> TextAnalyzer {
        // TODO: (仮)
        let mode = Mode::Normal;
        let dictionary = load_dictionary("embedded://ipadic").unwrap();
        let segmenter = Segmenter::new(mode, dictionary, None);
        let tokenizer = LinderaTokenizer::from_segmenter(segmenter);
        tokenizer.into()
    }
}