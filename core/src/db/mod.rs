#![allow(unused)]
#![doc = "アプリケーションのデータベース"]
#[cfg(feature = "backend")]
pub mod migrate;
#[cfg(feature = "backend")]
pub mod query;
pub mod schema;
pub mod util;
