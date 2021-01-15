use super::base::Stringify;
use crate::__kantex_implement_to_string;

#[derive(Clone)]
pub struct Document {
    sections: Vec<Box<dyn Stringify + 'static>>
}

impl Document {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    pub fn add_section<T: Stringify + 'static>(&mut self, section: T) -> Self {
        self.sections.push(Box::new(section));
        self.clone()
    }
}

impl Stringify for Document {
    fn stringify(&self) -> String {
        let mut list: Vec<String> = Vec::new();
        for i in &self.sections {
            let text = i.stringify();
            list.push(text);
        }
        list.join("\n\n")
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

__kantex_implement_to_string!{ Document }

mod tests {
    #[test]
    fn test_doc() {
        use crate::{Document, Sections};

        let expected = "<b>title</b>\n    key\n\n<b>title</b>\n    key";
        let actual = Document::new()
            .add_section(
                Sections::new("title")
                    .include("key")
            )
            .add_section(
                Sections::new("title")
                    .include("key")
            );
        assert_eq!(actual.to_string(), expected);
    }

    #[test]
    fn test_doc_non_inline() {
        use crate::{Document, Sections};

        let expected = "<b>title</b>\n    key\n\n<b>title</b>\n    key";
        let mut doc = Document::new();
        doc.add_section(
            Sections::new("title")
                .include("key")
        );
        doc.add_section(
            Sections::new("title")
                .include("key")
        );
        assert_eq!(doc.to_string(), expected);
    }
}
