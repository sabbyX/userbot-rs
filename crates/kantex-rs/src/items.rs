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

impl Stringify for MentionLink<'static> {
    fn stringify(&self) -> String {
        FormattedText::hyperlink(self.label, format!("tg://user?id={}", self.uid).as_str())
    }
}

#[derive(Clone)]
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

crate::implement_to_string!{ MentionLink<'static> KeyValueItem }

mod tests {

    #[test]
    fn test_key_value() {
        use crate::KeyValueItem;

        let expected = "key: value";
        let actual = KeyValueItem::new("key", "value");
        assert_eq!(actual.to_string(), expected)
    }

    #[test]
    fn test_mention_link() {
        use crate::MentionLink;

        let expected = r##"<a href="tg://user?id=0">user</a>"##;
        let actual = MentionLink::new("user", 0);
        assert_eq!(actual.to_string(), expected);
    }
}
