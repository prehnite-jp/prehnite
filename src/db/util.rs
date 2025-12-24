pub fn placeholder_helper(placeholder: impl AsRef<str>, count: usize) -> String {
    vec![placeholder.as_ref(); count].join(",")
}
