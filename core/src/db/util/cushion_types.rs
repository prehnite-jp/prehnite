use rhai::{Array, Dynamic};

pub struct OptionString(Option<String>);

impl From<Option<String>> for OptionString {
    fn from(value: Option<String>) -> Self {
        OptionString(value)
    }
}

impl From<Dynamic> for OptionString {
    fn from(value: Dynamic) -> Self {
        if value.is_string() {
            Some(value.into_string().unwrap_or_default()).into()
        } else {
            None.into()
        }
    }
}

impl From<OptionString> for Option<String> {
    fn from(value: OptionString) -> Self {
        value.0
    }
}

pub struct VecString(Vec<String>);

impl From<Vec<String>> for VecString {
    fn from(value: Vec<String>) -> Self {
        VecString(value)
    }
}

impl From<Dynamic> for VecString {
    fn from(value: Dynamic) -> Self {
        if value.is_array() {
            match value.into_array() {
                Ok(v) => v.into(),
                Err(_) => VecString(vec![]),
            }
        } else {
            VecString(vec![])
        }
    }
}

impl From<Array> for VecString {
    fn from(value: Array) -> Self {
        value
            .into_iter()
            .filter_map(|v| {
                let res = v.into_string().unwrap_or_default();
                if res.is_empty() { None } else { Some(res) }
            })
            .collect::<Vec<String>>()
            .into()
    }
}

impl From<VecString> for Vec<String> {
    fn from(value: VecString) -> Self {
        value.0
    }
}