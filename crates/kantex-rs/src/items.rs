use super::{base::Stringify, styles::FormattedText};

const KEY_VALUE_DELIM: char = ':';

#[derive(Clone)]
pub struct MentionLink {
    label: String,
    uid: i32,
}

impl MentionLink {
    pub fn new(label: &str, uid: i32) -> Self {
        Self {
            label: label.into(),
            uid
        }
    }
}

impl Stringify for MentionLink {
    fn stringify(&self) -> String {
        FormattedText::hyperlink(self.label.as_str(), format!("tg://user?id={}", self.uid).as_str())
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

crate::implement_to_string!{ MentionLink KeyValueItem }

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
