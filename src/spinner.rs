use crate::{is_allowed, Displayable, Format, Formatter, IndentGuard};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use scopeguard::defer;
use std::{collections::HashSet, io::Write, time::Duration};

#[derive(Debug, Clone)]
pub struct Spinner {
    debug: bool,
    allowed_formats: HashSet<Format>,
    max_line_length: usize,
    indentation_level: u16,
    spinner: ProgressBar,
}

impl Spinner {
    pub fn new(debug: bool, max_line_length: usize) -> Spinner {
        let spinner = new_spinner();

        Spinner {
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

impl Default for Spinner {
    fn default() -> Self {
        let spinner = new_spinner();

        Self {
            debug: false,
            allowed_formats: HashSet::new(),
            max_line_length: 80,
            indentation_level: 0,
            spinner,
        }
    }
}

fn new_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(120));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner
}

impl Formatter for Spinner {
    fn print(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Spinner, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        self.spinner.set_message(msg.to_string());

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Spinner, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        self.spinner.println(format!("{msg}"));

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Spinner, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        self.spinner.println(format!("{} {msg}", "x".red()));

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Spinner, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        self.spinner.println(format!("{} {msg}", "✓".green()));

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Spinner, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        self.spinner.println(format!("{} {msg}", "!!".yellow()));

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn indent(&mut self) -> Box<dyn IndentGuard> {
        self.indentation_level += 1;
        Box::new(Guard {})
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Spinner, &self.allowed_formats) || !self.debug {
            self.allowed_formats = HashSet::new();
            return;
        }

        self.spinner
            .println(format!("{} {msg}", "[debug]".dimmed()));

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !is_allowed(Format::Spinner, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return "".to_string();
        }

        let mut input = String::from("");

        self.spinner.suspend(|| {
            print!("{} {msg}", "?".magenta());

            std::io::stdout().flush().unwrap();

            let _ = std::io::stdin().read_line(&mut input);
        });

        defer! {
            self.allowed_formats = HashSet::new();
        }

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut dyn Formatter {
        self.allowed_formats = types.into_iter().collect();
        self
    }

    fn finish(&self) {
        self.spinner.finish_and_clear();
    }
}
