use super::base::Stringify;

pub struct Document {
    sections: Vec<Box<dyn Stringify + 'static>>
}

impl Document {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    pub fn add_section<T: Stringify + 'static>(mut self, section: T) -> Self {
        self.sections.push(Box::new(section));
        self
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
