
// TODO: rather use trait Into<String> instead of implementing our own
pub trait Stringify: Send + Sync {
    fn stringify(&self) -> String;
}

impl Stringify for &str {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for String {
    fn stringify(&self) -> String {
        self.clone()
    }
}
