use super::{base::Stringify, styles::FormattedText};

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

    pub fn include<T: Stringify + 'static>(mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self
    }
}

crate::implement_stringify!{ Sections, 4 }

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

    pub fn include<T: Stringify + 'static>(mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self
    }
}

crate::implement_stringify!{ SubSections, 8 }

pub struct SubSubSections {
    header: String,
    items: Vec<Box<dyn Stringify + 'static>>,
}

impl SubSubSections {
    pub fn include<T: Stringify + 'static>(mut self, text: T) -> Self {
        self.items.push(Box::new(text));
        self
    }
}

crate::implement_stringify!{ SubSubSections, 8 }

// auto impl
crate::implement_to_string!{ Sections SubSections SubSubSections }
crate::implement_add_trait!{ Sections SubSections SubSubSections }
