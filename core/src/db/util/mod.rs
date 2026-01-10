#![allow(unused)]

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::hash::Hash;

pub mod cushion_types;
pub(crate) mod prefixer;

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

#[macro_export]
macro_rules! to_hash_map_key_name {
    ($v:expr) => {
        $v.into_iter().map(|v| (v.name.clone(), v)).collect()
    };
}

#[macro_export]
macro_rules! to_hash_map_key_id {
    ($v:expr) => {
        $v.into_iter().map(|v| (v.id.clone(), v)).collect()
    };
}
