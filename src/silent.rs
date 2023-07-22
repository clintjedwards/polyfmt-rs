use crate::{Displayable, Format, Formatter};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Silent {}

impl Formatter for Silent {
    fn print(&mut self, _msg: &dyn Displayable) {}
    fn println(&mut self, _msg: &dyn Displayable) {}
    fn err(&mut self, _msg: &dyn Displayable) {}
    fn success(&mut self, _msg: &dyn Displayable) {}
    fn warning(&mut self, _msg: &dyn Displayable) {}
    fn debugln(&mut self, _msg: &dyn Displayable) {}

    fn question(&mut self, _msg: &dyn Displayable) -> String {
        "".to_string()
    }

    fn only(&mut self, _types: Vec<Format>) -> &mut dyn Formatter {
        self
    }

    fn finish(&self) {}
}
