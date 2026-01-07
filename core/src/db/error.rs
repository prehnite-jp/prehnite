#![allow(unused)]
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ItemTypeDecodeError(pub &'static str);

impl Display for ItemTypeDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for ItemTypeDecodeError {}
