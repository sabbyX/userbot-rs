use super::{base::Stringify, styles::FormattedText};

const KEY_VALUE_DELIM: char = ':';

pub struct MentionLink {
    text: String
}

impl MentionLink {
    pub fn new(label: &str, uid: i32) {
        let mention_link = FormattedText::hyperlink(label, &*format!("tg://user?id={}", uid));
        Self {
            text: mention_link
        };
    }
}

impl Stringify for MentionLink {
    fn stringify(&self) -> String {
        self.text.to_string()
    }
}

pub struct KeyValueItem {
    key: Box<dyn Stringify + 'static>,
    value: Box<dyn Stringify + 'static>,
}

impl KeyValueItem {
    pub fn new<T: Stringify + 'static, S: Stringify + 'static>(key: T, value: S) -> Self {
        Self {
            key: Box::new(key),
            value: Box::new(value),
        }
    }
}

impl Stringify for KeyValueItem {
    fn stringify(&self) -> String {
        format!("{}{} {}", self.key.stringify(), KEY_VALUE_DELIM, self.value.stringify()).to_string()
    }
}
