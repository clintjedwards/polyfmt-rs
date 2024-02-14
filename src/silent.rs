use crate::{Displayable, Format, Formatter, IndentGuard};

struct Guard;

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        todo!()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Silent {}

impl Formatter for Silent {
    fn print(&mut self, _msg: &dyn Displayable) {}
    fn println(&mut self, _msg: &dyn Displayable) {}
    fn error(&mut self, _msg: &dyn Displayable) {}
    fn success(&mut self, _msg: &dyn Displayable) {}
    fn warning(&mut self, _msg: &dyn Displayable) {}
    fn debug(&mut self, _msg: &dyn Displayable) {}
    fn indent(&mut self) -> Box<dyn crate::IndentGuard> {
        Box::new(Guard {})
    }

    fn question(&mut self, _msg: &dyn Displayable) -> String {
        "".to_string()
    }

    fn only(&mut self, _types: Vec<Format>) -> &mut dyn Formatter {
        self
    }

    fn finish(&self) {}
}
