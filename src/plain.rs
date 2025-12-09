use crate::{
    format_text_by_length, take_and_check_allowed, Displayable, Format, Formatter, IndentGuard,
    Options,
};
use colored::Colorize;
use std::sync::{Arc, Mutex, Weak};
use std::{collections::HashSet, io::Write};

#[derive(Clone)]
pub struct Plain {
    debug: bool,
    indentation_level: u16,
    max_line_length: usize,
    allowed_formats: HashSet<Format>,
    output_target: Arc<Mutex<dyn Write + Send>>,
}

impl Plain {
    pub fn new(options: Options) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Plain {
            debug: options.debug,
            indentation_level: 0,
            allowed_formats: HashSet::new(),
            max_line_length: options.max_line_length,
            output_target: options.output_target.target,
        }))
    }
}

struct Guard {
    fmtter: Weak<Mutex<Plain>>,
}

impl Guard {
    fn new(fmtter: Arc<Mutex<Plain>>) -> Self {
        Self {
            fmtter: Arc::downgrade(&fmtter),
        }
    }
}

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(fmtter) = self.fmtter.upgrade() {
            let mut fmtter_lock = fmtter.lock().unwrap();
            fmtter_lock.outdent();
        }
    }
}

impl Plain {
    fn print(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Plain, &mut self.allowed_formats) {
            return;
        }

        let mut output_target = self.output_target.lock().unwrap();
        let _ = write!(output_target, "{msg}");
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Plain, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        let mut output_target = self.output_target.lock().unwrap();

        let _ = writeln!(
            output_target,
            "{}{}",
            " ".repeat(self.indentation_level.into()),
            lines.first().unwrap_or(&"".to_string()),
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            let _ = writeln!(
                output_target,
                "{}{}",
                " ".repeat(self.indentation_level.into()),
                line
            );
        }
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Plain, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 2, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        let mut output_target = self.output_target.lock().unwrap();

        let _ = writeln!(
            output_target,
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "x".red(),
            lines.first().unwrap_or(&"".to_string()),
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            let _ = writeln!(
                output_target,
                "{}{}",
                " ".repeat((self.indentation_level + 2).into()),
                line
            );
        }
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Plain, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 2, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        let mut output_target = self.output_target.lock().unwrap();

        let _ = writeln!(
            output_target,
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "✓".green(),
            lines.first().unwrap_or(&"".to_string()),
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            let _ = writeln!(
                output_target,
                "{}{}",
                " ".repeat((self.indentation_level + 2).into()),
                line
            );
        }
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Plain, &mut self.allowed_formats) {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 3, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        let mut output_target = self.output_target.lock().unwrap();

        let _ = writeln!(
            output_target,
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "!!".yellow(),
            lines.first().unwrap_or(&"".to_string()),
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            let _ = writeln!(
                output_target,
                " {}{}",
                " ".repeat((self.indentation_level + 2).into()),
                line
            );
        }
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Plain, &mut self.allowed_formats) || !self.debug {
            return;
        }

        let lines = format_text_by_length(msg, self.indentation_level + 8, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        let mut output_target = self.output_target.lock().unwrap();

        let _ = writeln!(
            output_target,
            "{}{} {}",
            " ".repeat(self.indentation_level.into()),
            "[debug]".dimmed(),
            lines.first().unwrap_or(&"".to_string()),
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            let _ = writeln!(
                output_target,
                "{} {}",
                " ".repeat((self.indentation_level + 7).into()),
                line
            );
        }
    }

    fn indent(fmtter: &Arc<Mutex<Self>>) -> Box<dyn IndentGuard> {
        let mut fmt = fmtter.lock().unwrap();
        fmt.indentation_level += 1;
        drop(fmt);
        let cloned_fmtter = Arc::clone(fmtter);
        let guard = Guard::new(cloned_fmtter);
        Box::new(guard)
    }

    fn outdent(&mut self) {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
        }
    }

    fn spacer(&mut self) {
        let mut output_target = self.output_target.lock().unwrap();

        let _ = writeln!(output_target);
    }

    #[allow(dead_code)]
    fn pause(&mut self) {}

    #[allow(dead_code)]
    fn start(&mut self) {}

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !take_and_check_allowed(Format::Plain, &mut self.allowed_formats) {
            return "".to_string();
        }

        let lines = format_text_by_length(msg, self.indentation_level + 2, self.max_line_length);

        let mut output_target = self.output_target.lock().unwrap();

        if lines.len() == 1 {
            let _ = write!(
                output_target,
                "{}{} {}",
                " ".repeat(self.indentation_level.into()),
                "?".magenta(),
                lines.first().unwrap_or(&"".to_string()),
            );
        } else {
            let _ = writeln!(
                output_target,
                "{}{} {}",
                " ".repeat(self.indentation_level.into()),
                "?".magenta(),
                lines.first().unwrap_or(&"".to_string()),
            );

            // Print the remaining lines except the last with writeln!
            let lines_count = lines.len();
            for (index, line) in lines.iter().enumerate().skip(1) {
                if index + 1 < lines_count {
                    // Not the last line
                    let _ = writeln!(
                        output_target,
                        "{}{}",
                        " ".repeat((self.indentation_level + 2).into()),
                        line
                    );
                } else {
                    // Last line, use print! instead
                    let _ = write!(
                        output_target,
                        "{}{}",
                        " ".repeat((self.indentation_level + 2).into()),
                        line
                    );
                }
            }
        }

        output_target.flush().unwrap();
        drop(output_target);

        let mut input = String::from("");

        let _ = std::io::stdin().read_line(&mut input);

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut Self {
        self.allowed_formats = types.into_iter().collect();
        self
    }

    fn finish(&self) {
        if let Ok(mut out) = self.output_target.lock() {
            let _ = out.flush();
        }
    }
}

impl Formatter for Arc<Mutex<Plain>> {
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
        Plain::indent(self)
    }

    fn outdent(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.outdent();
    }

    fn spacer(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.spacer()
    }

    fn pause(&mut self) {}

    fn resume(&mut self) {}

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
