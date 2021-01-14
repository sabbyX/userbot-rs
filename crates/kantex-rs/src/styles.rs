use std::collections::HashMap;
use strfmt::Format;


pub struct Tags<'a> {
    start: &'a str,
    end: &'a str,
}

impl<'a> Tags<'a> {
    pub fn new(start: &'a str, end: &'a str) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}

pub(crate) struct Entities<'a> {
    pub(crate) bold: Tags<'a>,
    pub(crate) italics: Tags<'a>,
    pub(crate) underline: Tags<'a>,
    pub(crate) strikethrough: Tags<'a>,
    pub(crate) hyperlink: &'a str,
}

impl<'a> Entities<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<'a> Default for Entities<'a> {
    fn default() -> Self {
        if cfg!(markdown) {
            Entities {
                bold: Tags::new("**", "**"),
                italics: Tags::new("__", "__"),
                underline: Tags::new("--", "--"),
                strikethrough: Tags::new("~~", "~~"),
                hyperlink: "[{}]({})",
            }
        } else {
            Entities {
                bold: Tags::new("<b>", "</b>"),
                italics: Tags::new("<i>", "</i>"),
                underline: Tags::new("<u>", "</u>"),
                strikethrough: Tags::new("<s>", "</s>"),
                hyperlink: "<a href=\"{}\">{}</a>",
            }
        }
    }
}

pub struct FormattedText;

impl FormattedText {

    pub fn bold(text: &str) -> String {
        let entities = Entities::new();
        entities.bold.start.to_owned() + text + entities.bold.end
    }

    pub fn italics(text: &str) -> String {
        let entities = Entities::new();
        entities.italics.start.to_owned() + text + entities.italics.end
    }

    pub fn underline(text: &str) -> String {
        let entities = Entities::new();
        entities.underline.start.to_owned() + text + entities.underline.end
    }

    pub fn strikethrough(text: &str) -> String {
        let entities = Entities::new();
        entities.strikethrough.start.to_owned() + text + entities.strikethrough.end
    }

    pub fn hyperlink(label: &str, url: &str) -> String {
        let entities = Entities::new();
        let mut vars = HashMap::new();
        vars.insert("url".to_string(), url.to_string());
        vars.insert("lable".to_string(), label.to_string());
        entities.hyperlink.format(&vars)
            .unwrap_or("<Failed to create mention link>".to_string())
    }
}
