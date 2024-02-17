//! # Polyfmt
//!
//! `polyfmt` is a convenience package that provides multiple forms of formatted output.
//! Useful for CLI applications where you might want to provide JSON output for machine users,
//! but pretty output for interactive users.
//!
//! ## Why
//!
//! In a command line application you usually want to provide some well-formatted output to users. This
//! may include progress bars, timers, spinners, or tables so that interactive users can better parse
//! your programs output. For non-interactive users or automation this might make your CLI application
//! difficult to parse, build automation around, or just unnecessarily verbose. To this end, you might want to
//! provide a common serialization format to users who use your CLI app from within a non-interactive environment.

//! Polyfmt aims to simplify the API around multiple formatting options and make it easy to switch between them.
//!
//! ## Usage
//!
//! Polyfmt provides a very simple API full of print functions.
//!
//! ### Using the global formatter
//!
//! The easiest way to use polyfmt is by using the global formatter:
//!
//! ```rust
//! use polyfmt::println;
//!
//! println!("Hello from polyfmt");
//! ```
//!
//! This is good for simple implementations but obviously the whole point of this library is being able to switch
//! formatters. To do this you can still use a global formatter. (Which is available whereever polyfmt is imported)
//!
//!  ### Altering the global formatter
//! Initiate a new formatter instance, passing in what type of formatter you want back. This is usually passed in
//! by your user at runtime via flags or config.
//!
//! ```rust
//! use polyfmt::{new, Format, Options, println};
//!
//! let fmt = polyfmt::new(Format::Plain, Options::default()).unwrap();
//! polyfmt::set_global_formatter(fmt);
//!
//! // Use the returned formatter to print a simple string.
//!
//! println!("something");
//! // Output: `something`
//! ```
//!
//! ### Using a scoped formatter
//!
//! Lastly you might want to just use a scoped formatter for specific instances. To do this you can just directly
//! use the formatter you get back from the new function:
//!
//! ```rust
//! use polyfmt::{new, Format, Options};
//! let mut fmt = polyfmt::new(Format::Plain, Options::default()).unwrap();
//! fmt.print(&"test");
//! ```
//!
//! ### Filtering output
//!
//! Sometimes you'll want to output something only for specific formatters.
//! You can use the [only](Formatter::only) function to list formatters for which
//! the following print command will only print for those formatters.
//!
//! ```rust
//! # use polyfmt::{new, Format, Options};
//! # let mut fmt = polyfmt::new(Format::Plain, Options::default()).unwrap();
//! fmt.only(vec![Format::Plain]).print(&"test");
//!
//! // This will only print the string "test" if the formatter Format is "Plain".
//! ```
//!
//! The global macros also allow you to variadically list formats to whitelist on the fly:
//!
//! ```rust
//! # use polyfmt::{print, Format};
//! print!("test", Format::Plain, Format::Pretty)
//! ```
//!
//! ### Dynamically choosing a format
//! Polyfmt is meant to be used as a formatter that is easy to be changed by the user.
//! So most likely you'll want to automatically figure out which formatter you want from
//! a flag of env_var the user passes in.
//!
//! ```rust
//! # use polyfmt::{new, Format, Options};
//! # use std::str::FromStr;
//! let some_flag = "plain".to_string(); // case-insensitive
//! let format = Format::from_str(&some_flag).unwrap();
//!
//! let mut fmt = new(format, Options::default()).unwrap();
//! ```
//!
//! ### Additional Details
//!
//! * You can turn off color by using the popular `NO_COLOR` environment variable.
//! * Anything to be printed must implement Display and Serialize due to the need to possibly print it into both plain
//! plaintext and json.
//! * When you finish using a formatter you should call the [finish](Formatter::finish) function. This flushes the output
//! buffer and cleans up anything else before your program exists.

mod json;
pub mod macros;
mod plain;
mod silent;
mod spinner;
mod tree;

use anyhow::{bail, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Debug, Display},
    io::Write,
    sync::Mutex,
    time::Duration,
};
use strum::EnumString;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Debug, EnumString, Clone, PartialEq, Eq, Hash)]
#[strum(ascii_case_insensitive)]
pub enum Format {
    /// Outputs text in a humanized fashion without any other additions.
    Plain,

    /// Outputs text in a more humanized fashion, providing tree box graphics on the left hand
    /// side of output.
    Tree,

    /// Outputs text in a more humanized fashion, providing a spinner automatically.
    Spinner,

    /// Outputs json formatted text, mainly suitable to be read by computers.
    Json,

    /// Dummy formatter that doesn't print anything, can be used when users don't want any
    /// output at all.
    Silent,
}

/// Trait for the indentation guard.
pub trait IndentGuard {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// Turn on printing for debug lines.
    pub debug: bool,

    /// Maximum character length for lines including indentation.
    pub max_line_length: usize,

    /// Amount of spacing between end of window and start of text.
    pub padding: u16,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            debug: Default::default(),
            max_line_length: 80,
            padding: 0,
        }
    }
}

/// Meant to represent types that can both be Serialized to JSON and implement the Display trait.
/// This allows polyfmt to not only print input given to it, but intelligently parse types into JSON when the formatter
/// requires it.
pub trait Displayable: erased_serde::Serialize + Display {
    fn as_serialize(&self) -> &(dyn erased_serde::Serialize);
}

// Blanket implementation for Displayable on any type that implements the combination of traits that equal displayable.
impl<T: erased_serde::Serialize + Display> Displayable for T {
    fn as_serialize(&self) -> &(dyn erased_serde::Serialize) {
        self as &(dyn erased_serde::Serialize)
    }
}

/// The core library trait.
pub trait Formatter: Debug + Send + Sync {
    /// Will attempt to intelligently print objects passed to it.
    ///
    /// Note: For the spinner format this will add a new persistent message to
    /// the spinner but not print a brand new line.
    fn print(&mut self, msg: &dyn Displayable);

    /// Prints the message with same functionality as [`print`](Self::print) but adds a
    /// newline to the end.
    fn println(&mut self, msg: &dyn Displayable);

    /// Prints the message noting it as an error to the user.
    fn error(&mut self, msg: &dyn Displayable);

    /// Prints the message noting it as an error to the user.
    fn success(&mut self, msg: &dyn Displayable);

    /// Prints the message noting it as a warning to the user.
    fn warning(&mut self, msg: &dyn Displayable);

    /// Prints a message only if debug is turned on in the formatter options.
    fn debug(&mut self, msg: &dyn Displayable);

    /// Increases the indentation of output.
    fn indent(&mut self) -> Box<dyn IndentGuard>;

    /// Decreases the indentation of output.
    fn outdent(&mut self);

    /// Prints the message noting it as a question to the user.
    /// It additionally also collects user input and returns it.
    ///
    /// It should be noted that adding filters to this mode might be especially important
    /// since even in a non-tty intended format like JSON, it will still stop and wait
    /// for user input. If filtered out it will return an empty string.
    fn question(&mut self, msg: &dyn Displayable) -> String;

    /// Allows the ability to restrict specific formatter lines to only the
    /// formats mentioned
    fn only(&mut self, types: Vec<Format>) -> &mut dyn Formatter;

    fn finish(&self);
}

/// Instantiates a Global formatter for easy use. This formatter can be altered by the library
/// user using `set_global_formatter`.
static GLOBAL_FORMATTER: Lazy<Mutex<Box<dyn Formatter>>> = Lazy::new(|| {
    let format = Format::Plain;
    let options = Options::default();
    Mutex::new(new(format, Some(options)).unwrap())
});

/// Set the global formatter to a custom formatter.
pub fn set_global_formatter(formatter: Box<dyn Formatter>) {
    *GLOBAL_FORMATTER.lock().unwrap() = formatter;
}

/// Return the current global formatter. Mainly used for macros, should be unneeded for scoped formatters.
pub fn get_global_formatter() -> &'static Mutex<Box<dyn Formatter>> {
    &GLOBAL_FORMATTER
}

/// Constructs a new formatter of your choosing.
///
/// # Example
///
/// ```
/// use polyfmt::{new, Format, Options};
/// let mut fmt = new(Format::Plain, Options::default()).unwrap();
/// fmt.print(&"something");
///
/// // You can also specify that certain lines be printed only when certain formatters are in effect.
/// fmt.only(vec![Format::Plain]).err(&"test");
/// ```
pub fn new(
    format: Format,
    options: Option<Options>,
) -> Result<Box<dyn Formatter>, Box<dyn Error + Send + Sync>> {
    match format {
        Format::Plain => {
            let options = options.unwrap_or_default();
            let formatter = plain::Plain::new(options.debug, options.max_line_length);
            Ok(Box::new(formatter))
        }
        Format::Spinner => {
            let options = options.unwrap_or_default();
            let formatter =
                spinner::Spinner::new(options.debug, options.max_line_length, options.padding);
            Ok(Box::new(formatter))
        }
        Format::Tree => {
            let options = options.unwrap_or_default();
            let formatter = tree::Tree::new(options.debug, options.max_line_length);
            Ok(Box::new(formatter))
        }
        Format::Json => {
            let options = options.unwrap_or_default();
            let formatter = json::Json::new(options.debug);
            Ok(Box::new(formatter))
        }
        Format::Silent => {
            let formatter = silent::Silent {};
            Ok(Box::new(formatter))
        }
    }
}

/// Convenience function to determine if format should run based on allowed formats.
fn is_allowed(current_format: Format, allowed_formats: &HashSet<Format>) -> bool {
    if !allowed_formats.contains(&current_format) && !allowed_formats.is_empty() {
        return false;
    }

    true
}

/// Convenience function to chunk lines of text based on the max line length.
fn format_text_length(
    msg: &dyn Displayable,
    indentation_level: u16,
    max_line_length: usize,
) -> Vec<String> {
    let msg = msg.to_string();
    let indentation_level = usize::from(indentation_level);

    // If the indentation level is already past the max line length we can't print anything.
    if max_line_length < indentation_level {
        return vec![];
    }

    let max_line_width = max_line_length - indentation_level;
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in msg.split_whitespace() {
        // Check if adding the next word exceeds the max line width
        if current_line.len() + word.len() + 1 > max_line_width {
            // Finish the current line and start a new one
            lines.push(current_line);
            current_line = String::new();
        }

        if !current_line.is_empty() {
            current_line.push(' '); // Add space before the word if it's not the beginning of a line
        }

        current_line.push_str(word);
    }

    // If the last character is whitespace add it back.
    if msg.split_whitespace().last().unwrap_or_default() == " " {
        current_line.push(' ');
    }

    // Add the last line if it's not empty
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
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

    bail!("display chooser was interrupted before ending properly")
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

#[cfg(test)]
mod tests {
    use crate::{debug, error, println, question, success, warning};
    use std::{str::FromStr, thread, time};

    #[test]
    fn tree() {
        let options = crate::Options {
            debug: true,
            max_line_length: 40,
            padding: 0,
        };

        let some_flag = "tree".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, Some(options)).unwrap();

        fmt.println(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.error(&"Hello from polyfmt, Look at how well it breaks up lines!");

        let _guard = fmt.indent();

        fmt.success(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.warning(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.debug(&"Hello from polyfmt, Look at how well it breaks up lines!");
    }

    #[test]
    fn spinner() {
        let options = crate::Options {
            debug: true,
            max_line_length: 40,
            padding: 1,
        };
        let ten_millis = time::Duration::from_secs(1);

        let some_flag = "spinner".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, Some(options)).unwrap();

        fmt.println(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.success(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.warning(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.debug(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.error(&"Hello from polyfmt, Look at how well it breaks up lines!");
    }

    #[test]
    fn json() {
        let options = crate::Options {
            debug: true,
            max_line_length: 80,
            padding: 0,
        };

        let some_flag = "json".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, Some(options)).unwrap();

        fmt.print(&"Demoing! ");
        fmt.println(&"Hello from polyfmt");
        fmt.success(&"This is a successful message!");
        fmt.warning(&"This is a warning message");
        fmt.debug(&"This is a debug message");
        fmt.error(&"This is an error message");
    }

    #[test]
    fn plain() {
        let options = crate::Options {
            debug: true,
            max_line_length: 40,
            padding: 0,
        };

        let some_flag = "plain".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, Some(options)).unwrap();

        fmt.println(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.error(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.success(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.warning(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.debug(&"Hello from polyfmt, Look at how well it breaks up lines!");
    }

    // These tests aren't real tests, I just eyeball things to see if they work.
    // Maybe I'll write real tests, maybe I wont. Shut-up.
    #[test]
    fn global_easy() {
        let options = crate::Options {
            debug: true,
            max_line_length: 100,
            padding: 1,
        };

        let some_flag = "plain".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, Some(options)).unwrap();
        fmt.question(&"Hello from polyfmt, Look at how well it breaks up lines: ");
        crate::set_global_formatter(fmt);

        println!("Hello from polyfmt, Look at how well it breaks up lines!");
        success!("Hello from polyfmt, Look at how well it breaks up lines!");
        error!("Hello from polyfmt, Look at how well it breaks up lines!");
        debug!("Hello from polyfmt, Look at how well it breaks up lines!");
        warning!("Hello from polyfmt, Look at how well it breaks up lines!");
        println!("testing things to other things");
        question!("Hello from polyfmt, Look at how well it breaks up lines:");
    }
}
