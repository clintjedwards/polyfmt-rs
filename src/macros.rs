use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;
use std::time::Duration;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::{plain, Formatter};

#[macro_export]
macro_rules! btext {
    ($($arg:expr),+ $(,)?) => {
        let mut formatter = __GLOBAL_FORMATTER.lock().unwrap();

        if formatter.header_printed {
            println!("{}", "┊".magenta());
        }

        let mut first_line_printed = false;

        for (index, msg) in [$($arg),+].iter().enumerate() {
            if index == 0 && !formatter.header_printed {
                println!("{} {}", "┌─".magenta(), msg);
                formatter.header_printed = true;
                continue;
            }

            if !first_line_printed {
                if msg.is_empty() {
                    println!("{}  {}", "│".magenta(), msg);
                    first_line_printed = true;
                    continue
                }

                println!("{} {}", "├─".magenta(), msg);
                first_line_printed = true;
                continue
            }

            println!("{}  {}", "│".magenta(), msg);
        }

        drop(formatter);
    };
}

/// Print a success message with a check-mark.
///
/// # Examples
///
/// ```
/// # use polyfmt::success;
/// let name = "Clint";
/// success!("Hello, {name}");
/// success!("Hello Clint");
/// success!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! bsuccess {
    ($s:expr $(, $arg:expr),*) => ({
        println!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::green("✓"), format_args!($s, $($arg),*))
    });

    ($s:expr, $($arg:expr),*) => ({
        println!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::green("✓"), format_args!($s, $($arg),*))
    });
}

/// Print an error message with a red x.
///
/// # Examples
///
/// ```
/// let name = "Clint";
/// err!("Hello, {name}");
/// err!("Hello Clint");
/// err!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! berr {
    ($s:expr $(, $arg:expr),*) => ({
        println!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::red("x"), format_args!($s, $($arg),*))
    });

    ($s:expr, $($arg:expr),*) => ({
        println!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::red("x"), format_args!($s, $($arg),*))
    });
}

/// Print an warning message with an exclamation mark.
///
/// # Examples
///
/// ```
/// let name = "Clint";
/// warning!("Hello, {name}");
/// warning!("Hello Clint");
/// warning!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! bwarning {
    ($s:expr $(, $arg:expr),*) => ({
        println!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::yellow("!!"), format_args!($s, $($arg),*))
    });

    ($s:expr, $($arg:expr),*) => ({
        println!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::yellow("!!"), format_args!($s, $($arg),*))
    });
}

/// Print a question which waits for user input.
///
/// # Examples
///
/// ```
/// let name = "Clint";
/// question!("Hello, {name}");
/// question!("Hello Clint");
/// let input = question!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! bquestion {
    ($s:expr $(, $arg:expr),*) => ({
        print!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::magenta("?"), format_args!($s, $($arg),*));

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");
        let _ = std::io::stdin().read_line(&mut input);
        input.trim().to_string()
    });

    ($s:expr, $($arg:expr),*) => ({
        print!("{} {} {}", ::colored::Colorize::magenta("├──"), ::colored::Colorize::magenta("?"), format_args!($s, $($arg),*));

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");
        let _ = std::io::stdin().read_line(&mut input);
        input.trim().to_string()
    });
}

/// Print a success message with a check-mark.
///
/// # Examples
///
/// ```
/// let name = "Clint";
/// success!("Hello, {name}");
/// success!("Hello Clint");
/// success!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! success {
    ($s:expr $(, $arg:expr),*) => ({
        println!("{} {}", ::colored::Colorize::green("✓"), format_args!($s, $($arg),*))
    });

    ($s:expr, $($arg:expr),*) => ({
        println!("{} {}", ::colored::Colorize::green("✓"), format_args!($s, $($arg),*))
    });
}

/// Print an error message with a red x.
///
/// # Examples
///
/// ```
/// let name = "Clint";
/// err!("Hello, {name}");
/// err!("Hello Clint");
/// err!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! error {
    ($s:expr $(, $arg:expr),*) => ({
        println!("{} {}", ::colored::Colorize::red("x"), format_args!($s, $($arg),*))
    });

    ($s:expr, $($arg:expr),*) => ({
        println!("{} {}", ::colored::Colorize::red("x"), format_args!($s, $($arg),*))
    });
}

/// Print an warning message with an exclamation mark.
///
/// # Examples
///
/// ```
/// let name = "Clint";
/// warning!("Hello, {name}");
/// warning!("Hello Clint");
/// warning!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! warning {
    ($s:expr $(, $arg:expr),*) => ({
        println!("{} {}", ::colored::Colorize::yellow("!!"), format_args!($s, $($arg),*))
    });

    ($s:expr, $($arg:expr),*) => ({
        println!("{} {}", ::colored::Colorize::yellow("!!"), format_args!($s, $($arg),*))
    });
}

/// Print a question which waits for user input.
///
/// # Examples
///
/// ```
/// let name = "Clint";
/// question!("Hello, {name}");
/// question!("Hello Clint");
/// let input = question!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! question {
    ($s:expr $(, $arg:expr),*) => ({
        print!("{} {}", ::colored::Colorize::magenta("?"), format_args!($s, $($arg),*));

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");
        let _ = std::io::stdin().read_line(&mut input);
        input.trim().to_string()
    });

    ($s:expr, $($arg:expr),*) => ({
        print!("{} {}", ::colored::Colorize::magenta("?"), format_args!($s, $($arg),*));

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");
        let _ = std::io::stdin().read_line(&mut input);
        input.trim().to_string()
    });
}

#[macro_export]
macro_rules! debug {
    ($s:expr $(, $arg:expr),*) => ({
        println!("{}: {}", ::colored::Colorize::on_yellow(::colored::Colorize::dimmed("[debug]")), format_args!($s, $($arg),*))
    });

    ($s:expr, $($arg:expr),*) => ({
        println!("{}: {}", ::colored::Colorize::on_yellow(::colored::Colorize::dimmed("[debug]")), format_args!($s, $($arg),*))
    });
}

/// Creates a TUI multiple choice modal.
/// The Hashmap passed in is the mapping of label to actual raw value. This is helpful when you want the raw value
/// for passing in to another function but the label to display to the user.
/// Returns the (label, value) tuple that the user chose.
pub fn display_chooser(choices: HashMap<String, String>) -> Result<(String, String)> {
    let mut labels: Vec<_> = choices.keys().collect();
    labels.sort();

    let mut selected_index = 0;

    display_choices(&labels, selected_index);

    // Get the standard input stream.
    let stdin = std::io::stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = std::io::stdout().into_raw_mode()?;

    for c in stdin.keys() {
        match c? {
            Key::Ctrl('c') => break,
            Key::Up if selected_index > 0 => {
                selected_index -= 1;
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Up(labels.len() as u16),
                    termion::clear::AfterCursor
                )?;
                display_choices(&labels, selected_index);
            }
            Key::Down if selected_index < labels.len() - 1 => {
                selected_index += 1;
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Up(labels.len() as u16),
                    termion::clear::AfterCursor
                )?;
                display_choices(&labels, selected_index);
            }
            Key::Char('\n') => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Up(labels.len() as u16),
                    termion::clear::AfterCursor
                )?;
                write!(stdout, "{}", termion::cursor::Show)?;
                stdout.flush()?;

                return Ok((
                    labels[selected_index].to_string(),
                    choices[labels[selected_index]].clone(),
                ));
            }
            _ => {}
        }
        stdout.flush()?;
    }

    unreachable!("the loop should always return")
}

fn display_choices(choices: &[&String], selected: usize) {
    for (index, choice) in choices.iter().enumerate() {
        if index == selected {
            _ = write!(std::io::stdout(), "{} {}\r\n", ">".green(), choice.green());
        } else {
            _ = write!(std::io::stdout(), "  {}\r\n", choice);
        }
    }
}

/// Enables the spinner to automatically clean itself up, when dropped.
pub struct Spinner {
    internal: ProgressBar,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.internal.finish_and_clear();
    }
}

impl Spinner {
    pub fn create(initial_message: &str) -> Spinner {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(Duration::from_millis(120));
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message(initial_message.to_string());

        Spinner { internal: spinner }
    }

    pub fn set_message(&self, msg: String) {
        self.internal.set_message(msg);
    }

    pub fn suspend<F: FnOnce() -> T, T>(&self, f: F) -> T {
        self.internal.suspend(f)
    }
}
