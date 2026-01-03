#![allow(unused)]

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::hash::Hash;

pub mod cushion_types;

pub struct Prefixer {
    prefix: String,
    result: String,
}

impl Default for Prefixer {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            result: String::new(),
        }
    }
}

impl Prefixer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            ..Default::default()
        }
    }

    pub fn set_prefix(&mut self, prefix: impl Into<String>) {
        self.prefix = prefix.into();
    }

    pub fn prefix(&mut self, col_name: impl AsRef<str>) -> &str {
        self.result = format!("{}{}", self.prefix, col_name.as_ref());
        &self.result
    }
}

pub fn get_optional<K, V>(items: &HashMap<K, V>, key: &Option<K>) -> Option<V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    Some(items.get(&(key.clone()?))?.clone())
}

pub fn utc_parse_format(datetime: Option<String>, format: &str) -> Option<DateTime<Utc>> {
    Some(DateTime::parse_from_str(&datetime?, format).ok()?.to_utc())
}
