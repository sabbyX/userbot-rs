use super::{base::Stringify, styles::FormattedText};

#[derive(Clone)]
pub struct Sections {
    header: String,
    items: Vec<Box<dyn Stringify + 'static>>
}

impl Sections {
    pub fn new(title: &str) -> Self {
        Self {
            header: title.to_owned(),
            items: Vec::new(),
        }
    }

    pub fn include<T: Stringify + 'static>(&mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self.clone()
    }
}

crate::implement_stringify!{ Sections, 4 }

#[derive(Clone)]
pub struct SubSections {
    header: String,
    items: Vec<Box<dyn Stringify + 'static>>
}

impl SubSections {
    pub fn new(header: &str) -> Self {
        Self {
            header: header.to_owned(),
            items: Vec::new(),
        }
    }

    pub fn include<T: Stringify + 'static>(&mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self.clone()
    }
}

crate::implement_stringify!{ SubSections, 8 }

#[derive(Clone)]
pub struct SubSubSections {
    header: String,
    items: Vec<Box<dyn Stringify + 'static>>,
}

impl SubSubSections {

    pub fn new(header: &str) -> Self {
        Self {
            header: header.to_owned(),
            items: Vec::new(),
        }
    }

    pub fn include<T: Stringify + 'static>(&mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self.clone()
    }
}

crate::implement_stringify!{ SubSubSections, 12 }

// auto impl
crate::implement_to_string!{ Sections SubSections SubSubSections }
crate::implement_add_trait!{ Sections SubSections SubSubSections }

mod tests {
    #[test]
    fn test_section() {
        use crate::Sections;
        //                                  ---- 4
        let expected = "<b>title</b>\n    key";
        let actual = Sections::new("title")
            .include("key");
        assert_eq!(actual.to_string(), expected)
    }

    #[test]
    fn test_sub_section() {
        use crate::{Sections, SubSections};
        //                                  ---- 4            -------- 8
        let expected = "<b>title</b>\n    <b>title</b>\n        key";
        let actual = Sections::new("title")
            .include(
                SubSections::new("title")
                    .include("key")
            );
        assert_eq!(actual.to_string(), expected)
    }

    #[test]
    fn test_sub_sub_section() {
        use crate::{Sections, SubSections, SubSubSections};

        //                                  ---- 4            -------- 8            ------------ 12
        let expected = "<b>title</b>\n    <b>title</b>\n        <b>title</b>\n            key";
        let actual = Sections::new("title")
            .include(
                SubSections::new("title")
                    .include(
                        SubSubSections::new("title")
                            .include("key")
                    )
            );
        assert_eq!(actual.to_string(), expected)
    }

    #[test]
    fn test_sections_non_inline() {
        use crate::Sections;

        let expected = "<b>title</b>\n    key\n    key";
        let mut section = Sections::new("title");
        section.include("key");
        section.include("key");
        assert_eq!(section.to_string(), expected);
    }
}
