use super::{base::Stringify, styles::FormattedText};

const KEY_VALUE_DELIM: char = ':';

#[derive(Copy, Clone)]
pub struct MentionLink<'life> {
    label: &'life str,
    uid: i32,
}

impl<'life> MentionLink<'life> {
    pub fn new(label: &'life str, uid: i32) -> Self {
        Self {
            label,
            uid
        }
    }
}

impl Stringify for MentionLink<'_> {
    fn stringify(&self) -> String {
        FormattedText::hyperlink(self.label, format!("tg://user?id={}", self.uid).as_str())
    }
}

pub struct KeyValueItem {
    key: String,
    value: String,
}

impl KeyValueItem {
    pub fn new<T: Into<String>, S: Into<String>>(key: T, value: S) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Stringify for KeyValueItem {
    fn stringify(&self) -> String {
        format!("{}{} {}", self.key, KEY_VALUE_DELIM, self.value)
    }
}

crate::implement_to_string!(
    MentionLink<'_>,
    KeyValueItem
);
