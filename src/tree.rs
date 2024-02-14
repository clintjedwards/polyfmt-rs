use crate::{is_allowed, Displayable, Format, Formatter, IndentGuard};
use colored::Colorize;
use scopeguard::defer;
use std::io::Write;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tree {
    pub debug: bool,
    pub header_printed: bool,
    pub indentation: Vec<String>,
    allowed_formats: Vec<Format>,
}

struct Guard;

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        todo!()
    }
}

impl Formatter for Tree {
    fn print(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        print!("{} {msg}", "├──".magenta());
    }

    fn println(&mut self, msgs: Vec<&dyn Displayable>) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        if msgs.len() == 0 {
            return;
        }

        println!("{} {}", "├──".magenta(), msgs.first().unwrap());

        for individual_message in msgs {
            println!("{} {}", "   ".magenta(), individual_message);
        }
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        println!("{} {msg}", "x".red());
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        println!("{} {msg}", "✓".green());
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        println!("{} {msg}", "!!".yellow());
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) || !self.debug {
            self.allowed_formats = vec![];
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        println!("{} {msg}", "[debug]".dimmed());
    }

    fn indent(&mut self) -> Box<dyn IndentGuard> {
        self.indentation.push("  ".to_string());
        Box::new(Guard {})
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return "".to_string();
        }

        defer! {
            self.allowed_formats = vec![];
        }

        print!("{} {msg} ", "?".magenta());

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");

        let _ = std::io::stdin().read_line(&mut input);

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut dyn Formatter {
        self.allowed_formats = types;
        self
    }

    fn finish(&self) {
        std::io::stdout().flush().unwrap();
    }
}
