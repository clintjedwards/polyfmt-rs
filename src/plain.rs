use crate::{format_text_length, is_allowed, Displayable, Format, Formatter, IndentGuard};
use colored::Colorize;
use scopeguard::defer;
use std::{collections::HashSet, io::Write};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Plain {
    debug: bool,
    indentation_level: u16,
    max_line_length: usize,
    allowed_formats: HashSet<Format>,
}

impl Plain {
    pub fn new(debug: bool, max_line_length: usize) -> Plain {
        Plain {
            debug,
            max_line_length,
            ..Default::default()
        }
    }
}

struct Guard;

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        todo!()
    }
}

impl Formatter for Plain {
    fn print(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }

        print!("{msg}");
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        let lines = format_text_length(msg, self.indentation_level, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        println!(
            "{}{}",
            " ".repeat(self.indentation_level.into()),
            lines.first().unwrap(),
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            println!("{} {}", " ".repeat(self.indentation_level.into()), line);
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }

        println!("{msg}");
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }

        println!("{} {msg}", "x".red());
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }

        println!("{} {msg}", "✓".green());
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }

        println!("{} {msg}", "!!".yellow());
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Plain, &self.allowed_formats) || !self.debug {
            self.allowed_formats = HashSet::new();
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }

        println!("{} {msg}", "[debug]".dimmed());
    }

    fn indent(&mut self) -> Box<dyn IndentGuard> {
        self.indentation_level += 1;
        Box::new(Guard {})
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !is_allowed(Format::Plain, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return "".to_string();
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }

        print!("{} {msg} ", "?".magenta());

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");

        let _ = std::io::stdin().read_line(&mut input);

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut dyn Formatter {
        self.allowed_formats = types.into_iter().collect();
        self
    }

    fn finish(&self) {
        std::io::stdout().flush().unwrap();
    }
}
