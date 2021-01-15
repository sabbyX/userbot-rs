
// TODO: rather use trait Into<String> instead of implementing our own
pub trait Stringify: StringifyClone + Send + Sync {
    fn stringify(&self) -> String;
}

impl Stringify for &'static str {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for String {
    fn stringify(&self) -> String {
        self.clone()
    }
}

pub trait StringifyClone {
    fn clone_box(&self) -> Box<dyn Stringify>;
}

impl<T> StringifyClone for T
    where
        T: 'static + Stringify + Clone
{
    fn clone_box(&self) -> Box<dyn Stringify> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Stringify> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
