use crate::{is_allowed, Displayable, Format, Formatter};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use scopeguard::defer;
use std::{io::Write, time::Duration};

#[derive(Debug, Clone)]
pub struct Pretty {
    pub debug: bool,
    allowed_formats: Vec<Format>,
    spinner: ProgressBar,
}

impl Default for Pretty {
    fn default() -> Self {
        let spinner = new_spinner();

        Self {
            debug: false,
            allowed_formats: vec![],
            spinner,
        }
    }
}

impl Drop for Pretty {
    fn drop(&mut self) {
        self.finish();
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

impl Formatter for Pretty {
    fn print(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Pretty, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        self.spinner.set_message(msg.to_string());

        defer! {
            self.allowed_formats = vec![];
        }
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Pretty, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        self.spinner.println(format!("{msg}"));

        defer! {
            self.allowed_formats = vec![];
        }
    }

    fn err(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Pretty, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        self.spinner.println(format!("{} {msg}", "x".red()));

        defer! {
            self.allowed_formats = vec![];
        }
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Pretty, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        self.spinner.println(format!("{} {msg}", "✓".green()));

        defer! {
            self.allowed_formats = vec![];
        }
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Pretty, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return;
        }

        self.spinner.println(format!("{} {msg}", "!!".yellow()));

        defer! {
            self.allowed_formats = vec![];
        }
    }

    fn debugln(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Pretty, &self.allowed_formats) || !self.debug {
            self.allowed_formats = vec![];
            return;
        }

        self.spinner
            .println(format!("{} {msg}", "DEBUG".on_yellow().dimmed()));

        defer! {
            self.allowed_formats = vec![];
        }
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !is_allowed(Format::Pretty, &self.allowed_formats) {
            self.allowed_formats = vec![];
            return "".to_string();
        }

        let spinner_copy = self.spinner.clone();

        self.spinner.finish_and_clear();

        print!("{} {msg} ", "?".magenta());

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");

        let _ = std::io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        self.spinner = spinner_copy;

        defer! {
            self.allowed_formats = vec![];
        }

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut dyn Formatter {
        self.allowed_formats = types;
        self
    }

    fn finish(&self) {
        self.spinner.finish_and_clear();
    }
}
