
#[macro_export]
/// Macro to implement [`ToString`] for kantex-rs items
macro_rules! __kantex_implement_to_string {
    ($( $types:ty )*) => (
        $(
            impl ToString for $types {
                fn to_string(&self) -> String {
                    self.stringify()
                }
            }
        )*
    )
}

#[macro_export]
macro_rules! __kantex_implement_add_trait {
    ($($type:ty)*) => (
        $(
            impl std::ops::Add for $type {
                type Output = Self;
                fn add(mut self, rhs: Self) -> Self {
                    self.include(rhs)
                }
            }
        )*
    );
}

#[macro_export]
macro_rules! __kantex_implement_stringify {
    ($type:ty, $indent:literal) => {
        impl crate::base::Stringify for $type {
            fn stringify(&self) -> std::string::String {
                let mut item_list: Vec<String> = Vec::new();
                for i in &self.items {
                    let mut text = i.stringify();
                    text.insert_str(0, " ".repeat($indent).as_str());
                    item_list.push(text);
                }
                let header = FormattedText::bold(self.header.as_str());
                item_list.insert(0, header);
                item_list.join("\n")
            }
        }
    };
}
