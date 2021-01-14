use super::{base::Stringify, styles::FormattedText};

const SECTION_INDENT: usize = 4;
const SUB_SECTION_INDENT: usize = 8;
const SUB_SUB_SECTION_INDET: usize = 12;
const WHITESPACE: &str = " ";

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
    pub fn add<T: Stringify + 'static>(mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self
    }
}

impl Stringify for Sections {
    fn stringify(&self) -> String {
        let mut list: Vec<String> = Vec::new();
        for i in &self.items {
            let mut text = i.stringify();
            text.insert_str(0, &*WHITESPACE.repeat(SECTION_INDENT));
            list.push(text);
        }
        let header = FormattedText::bold(&*self.header);
        list.insert(0, header);
        list.join("\n")
    }
}

pub struct SubSections {
    header: String,
    items: Vec<Box<dyn Stringify + 'static>>
}

impl Stringify for SubSections {
    fn stringify(&self) -> String {
        let mut list: Vec<String> = Vec::new();
        for i in &self.items {
            let mut text = i.stringify();
            text.insert_str(0, &*WHITESPACE.repeat(SUB_SECTION_INDENT));
            list.push(text);
        }
        let header = FormattedText::bold(&*self.header);
        list.insert(0, header);
        list.join("\n")
    }
}

impl SubSections {
    pub fn new(header: &str) -> Self {
        Self {
            header: header.to_owned(),
            items: Vec::new(),
        }
    }

    pub fn add<T: Stringify + 'static>(mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self
    }
}

pub struct SubSubSections {
    header: String,
    items: Vec<Box<dyn Stringify + 'static>>,
}

impl Stringify for SubSubSections {
    fn stringify(&self) -> String {
        let mut list: Vec<String> = Vec::new();
        for i in &self.items {
            let mut text = i.stringify();
            text.insert_str(0, &*WHITESPACE.repeat(SUB_SUB_SECTION_INDET));
            list.push(text);
        }
        let header = FormattedText::bold(&*self.header);
        list.insert(0, header);
        list.join("\n")
    }
}

impl SubSubSections {
    pub fn add<T: Stringify + 'static>(mut self, text: T) -> Self {
        // let text = WHITESPACE.repeat(SUB_SUB_SECTION_INDET) + &*text.into();
        self.items.push(Box::new(text));
        self
    }
}
