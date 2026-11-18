use crate::builtin::TantivyTokenizers;
use derive_builder::Builder;
use std::env::current_dir;
use std::path::PathBuf;

#[derive(Builder, Clone, Debug)]
pub struct PrehniteDataOptions {
    #[builder(setter(into), default)]
    pub tokenizer: TantivyTokenizers,
    #[builder(setter(into))]
    pub index_cache_base_dir: PathBuf,
}

impl Default for PrehniteDataOptions {
    fn default() -> Self {
        PrehniteDataOptions {
            tokenizer: Default::default(),
            index_cache_base_dir: current_dir().unwrap().join(".index_cache"),
        }
    }
}
