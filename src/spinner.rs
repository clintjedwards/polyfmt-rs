use crate::{
    format_text_by_length, take_and_check_allowed, Displayable, Format, Formatter, IndentGuard,
    Options,
};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex, Weak};
use std::{collections::HashSet, io::Write, time::Duration};

#[derive(Clone)]
pub struct Spinner {
    debug: bool,
    indentation_level: u16,
    max_line_length: usize,
    allowed_formats: HashSet<Format>,

    spinner: ProgressBar,
}

impl Spinner {
    pub fn new(options: Options) -> Arc<Mutex<Self>> {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(Duration::from_millis(120));
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );

        Arc::new(Mutex::new(Spinner {
            debug: options.debug,
            max_line_length: options.max_line_length,
            indentation_level: options.padding,
            spinner,
            allowed_formats: HashSet::new(),
        }))
    }
}

struct Guard {
    spinner: Weak<Mutex<Spinner>>,
}

impl Guard {
    fn new(spinner: Arc<Mutex<Spinner>>) -> Self {
        Self {
            spinner: Arc::downgrade(&spinner),
        }
    }
}

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(spinner) = self.spinner.upgrade() {
            let mut spinner_lock = spinner.lock().unwrap();
            spinner_lock.outdent();
        }
    }
}

impl Spinner {
    fn print(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Spinner, &mut self.allowed_formats) {
            return;
        }

        self.spinner.set_message(msg.to_string());
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Spinner, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        self.spinner.println(
            " ".repeat(self.indentation_level.into()) + lines.first().unwrap_or(&"".to_string()),
        );

        for line in lines.iter().skip(1) {
            self.spinner.println(format!(
                "{}{}",
                " ".repeat(self.indentation_level.into()),
                line
            ));
        }
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Spinner, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 2, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        self.spinner.println(format!(
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "x".red(),
            lines.first().unwrap_or(&"".to_string())
        ));

        for line in lines.iter().skip(1) {
            self.spinner.println(format!(
                "{}{}",
                " ".repeat((self.indentation_level + 2).into()),
                line
            ));
        }
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Spinner, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 2, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        self.spinner.println(format!(
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "✓".green(),
            lines.first().unwrap_or(&"".to_string())
        ));

        for line in lines.iter().skip(1) {
            self.spinner.println(format!(
                "{}{}",
                " ".repeat((self.indentation_level + 2).into()),
                line
            ));
        }
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Spinner, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 2, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        self.spinner.println(format!(
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "!!".yellow(),
            lines.first().unwrap_or(&"".to_string())
        ));

        for line in lines.iter().skip(1) {
            self.spinner.println(format!(
                "{}{}",
                " ".repeat((self.indentation_level + 3).into()),
                line
            ));
        }
    }

    fn indent(spinner: &Arc<Mutex<Self>>) -> Box<dyn IndentGuard> {
        let mut fmt = spinner.lock().unwrap();
        fmt.indentation_level += 1;
        drop(fmt);
        let cloned_spinner = Arc::clone(spinner);
        let guard = Guard::new(cloned_spinner);
        Box::new(guard)
    }

    fn outdent(&mut self) {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
        }
    }

    fn spacer(&mut self) {
        self.spinner.println("");
    }

    fn pause(&mut self) {
        self.spinner.disable_steady_tick();
    }

    fn resume(&mut self) {
        self.spinner.enable_steady_tick(Duration::from_millis(120));
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Spinner, &mut self.allowed_formats) || !self.debug {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 8, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        self.spinner.println(format!(
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "[debug]".dimmed(),
            lines.first().unwrap_or(&"".to_string())
        ));

        for line in lines.iter().skip(1) {
            self.spinner.println(format!(
                "{}{}",
                " ".repeat((self.indentation_level + 8).into()),
                line
            ));
        }
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !take_and_check_allowed(Format::Spinner, &mut self.allowed_formats) {
            return "".to_string();
        }

        let lines = format_text_by_length(msg, self.indentation_level + 2, self.max_line_length);

        let mut input = String::from("");

        self.spinner.suspend(|| {
            if lines.len() == 1 {
                print!(
                    "{}{} {}",
                    " ".repeat(self.indentation_level.into()),
                    "?".magenta(),
                    lines.first().unwrap_or(&"".to_string()),
                );
            } else {
                println!(
                    "{}{} {}",
                    " ".repeat(self.indentation_level.into()),
                    "?".magenta(),
                    lines.first().unwrap_or(&"".to_string()),
                );

                // Print the remaining lines except the last with println!
                let lines_count = lines.len();
                for (index, line) in lines.iter().enumerate().skip(1) {
                    if index + 1 < lines_count {
                        // Not the last line
                        println!("{} {}", " ".repeat(self.indentation_level.into()), line);
                    } else {
                        // Last line, use print! instead
                        print!("{} {}", " ".repeat(self.indentation_level.into()), line);
                    }
                }
            }

            std::io::stdout().flush().unwrap();

            let _ = std::io::stdin().read_line(&mut input);
        });

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut Self {
        self.allowed_formats = types.into_iter().collect();
        self
    }

    fn finish(&self) {
        self.spinner.finish_and_clear();
    }
}

impl Formatter for Arc<Mutex<Spinner>> {
    fn print(&mut self, msg: &dyn Displayable) {
        let mut fmt = self.lock().unwrap();
        fmt.print(msg);
    }

    fn println(&mut self, msg: &dyn Displayable) {
        let mut fmt = self.lock().unwrap();
        fmt.println(msg);
    }

    fn error(&mut self, msg: &dyn Displayable) {
        let mut fmt = self.lock().unwrap();
        fmt.error(msg);
    }

    fn success(&mut self, msg: &dyn Displayable) {
        let mut fmt = self.lock().unwrap();
        fmt.success(msg);
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        let mut fmt = self.lock().unwrap();
        fmt.warning(msg);
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        let mut fmt = self.lock().unwrap();
        fmt.debug(msg);
    }

    fn indent(&mut self) -> Box<dyn IndentGuard> {
        Spinner::indent(self)
    }

    fn outdent(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.outdent();
    }

    fn spacer(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.spacer()
    }

    fn pause(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.pause();
    }

    fn resume(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.resume();
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        let mut fmt = self.lock().unwrap();
        fmt.question(msg)
    }

    fn only(&mut self, types: Vec<Format>) -> &mut dyn Formatter {
        let mut fmt = self.lock().unwrap();
        fmt.only(types);
        drop(fmt);
        self
    }

    fn finish(&self) {
        let fmt = self.lock().unwrap();
        fmt.finish();
    }
}
