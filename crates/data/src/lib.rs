
pub mod data_options;
mod database;
pub mod error;
pub mod manager;
pub mod models;
pub mod repos;
mod tantivy;
mod util;
#[cfg(feature = "gpui")]
use crate::manager::DataManagerStore;
pub use data_options::*;
pub use database::database_id::*;
pub use error::*;
pub use models::*;
pub use repos::*;
pub use tantivy::tokenizer::*;

#[cfg(not(feature = "gpui"))]
pub type TString = String;
#[cfg(feature = "gpui")]
pub type TString = gpui_kit::SharedString;

#[cfg(feature = "gpui")]
pub fn init_prehnite_data(cx: &mut gpui_kit::App) {
    DataManagerStore::init(cx);
    TagStore::init(cx);
}
