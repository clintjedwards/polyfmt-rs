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
//!
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
//! let fmt = polyfmt::new(Format::Plain, Options::default());
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
//! let mut fmt = polyfmt::new(Format::Plain, Options::default());
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
//! # let mut fmt = polyfmt::new(Format::Plain, Options::default());
//! fmt.only(vec![Format::Plain]).print(&"test");
//!
//! // This will only print the string "test" if the formatter Format is "Plain".
//! ```
//!
//! The global macros also allow you to list formats to whitelist on the fly:
//!
//! ```rust
//! # use polyfmt::{print, Format};
//! print!("test"; vec![Format::Plain, Format::Tree])
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
//! let mut fmt = new(format, Options::default());
//! ```
//!
//! ### Indentiation
//! Polyfmt supports indentation also with a similar implementation to spans in the tracing crate
//! You initialize the indent, tie it to a guard, and then once that guard drops out of scope the
//! indentation level will decrement.
//!
//! ```rust
//! # use polyfmt::{indent, println};
//!
//! println!("This line is base level of indentation.");
//! let _guard = indent!();
//! println!("This line has a greater indentation than the previous line.");
//! drop(_guard);
//! println!("This line has the same indentation level as the first.");
//! ```
//!
//! ### Additional Details
//!
//! * You can turn off color by using the popular `NO_COLOR` environment variable.
//! * Anything to be printed must implement Display and Serialize due to the need to possibly print it into both plain
//!   plaintext and json.
//! * When you finish using a formatter you should call the [finish](Formatter::finish) function. This flushes the output
//!   buffer and cleans up anything else before your program exists.
//!

mod json;
pub mod macros;
mod plain;
mod silent;
mod spinner;
mod tree;

use anyhow::{bail, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    io::Write,
    sync::Mutex,
    time::Duration,
};
use strum::EnumString;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\s+)").expect("Invalid regex");
}

#[derive(Debug, Default, EnumString, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum Format {
    #[default]
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
pub trait IndentGuard: Send + Sync {}

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
        let mut max_line_length = termion::terminal_size().unwrap_or((80, 80)).0.into();
        if max_line_length > 25 {
            max_line_length -= 5
        }

        Self {
            debug: Default::default(),
            max_line_length,
            padding: 0,
        }
    }
}

/// Meant to represent types that can both be Serialized to JSON and implement the Display trait.
/// This allows polyfmt to not only print input given to it, but intelligently parse types into JSON when the formatter
/// requires it.
pub trait Displayable: erased_serde::Serialize + Display {
    fn as_serialize(&self) -> &dyn erased_serde::Serialize;
}

// Blanket implementation for Displayable on any type that implements the combination of traits that equal displayable.
impl<T: erased_serde::Serialize + Display> Displayable for T {
    fn as_serialize(&self) -> &dyn erased_serde::Serialize {
        self as &dyn erased_serde::Serialize
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

    /// Prints a spacer where the type of spacer is determined by the [`Formatter`]
    fn spacer(&mut self);

    /// Temporarily pauses dynamic or animated output.
    ///
    /// This is primarily used by formatters that render animated elements such as
    /// spinners. When paused, the formatter should stop any background updates or
    /// redraw loops so that the terminal can be safely used for blocking or
    /// interactive operations (for example, opening a text editor or prompting
    /// for input).
    ///
    /// For non-animated formatters (like [`Plain`](Format::Plain) or
    /// [`Json`](Format::Json)), this method is typically a no-op.
    fn pause(&mut self);

    /// Resumes dynamic or animated output after a pause.
    ///
    /// This is the counterpart to [`pause`](Self::pause). Implementations that
    /// manage spinners or other periodic redraws should restore the display to
    /// its active state, continuing from where it left off.
    ///
    /// For non-animated formatters, this method is typically a no-op.
    fn resume(&mut self);

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
    Mutex::new(new(format, Options::default()))
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
/// let mut fmt = new(Format::Plain, Options::default());
/// fmt.print(&"something");
///
/// // You can also specify that certain lines be printed only when certain formatters are in effect.
/// fmt.only(vec![Format::Plain]).error(&"test");
/// ```
pub fn new(format: Format, options: Options) -> Box<dyn Formatter> {
    match format {
        Format::Plain => {
            let formatter = plain::Plain::new(options.debug, options.max_line_length);
            Box::new(formatter)
        }
        Format::Spinner => {
            let formatter =
                spinner::Spinner::new(options.debug, options.max_line_length, options.padding);
            Box::new(formatter)
        }
        Format::Tree => {
            let formatter = tree::Tree::new(options.debug, options.max_line_length);
            Box::new(formatter)
        }
        Format::Json => {
            let formatter = json::Json::new(options.debug);
            Box::new(formatter)
        }
        Format::Silent => {
            let formatter = silent::Silent {};
            Box::new(formatter)
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

fn split_on_whitespace_keep_delimiter_grouped(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_chunk = String::new();
    let mut inside_whitespace = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if inside_whitespace {
                // If the current character matches the type of the current whitespace chunk, add it
                if current_chunk.chars().next().unwrap().is_whitespace()
                    && c == current_chunk.chars().next().unwrap()
                {
                    current_chunk.push(c);
                } else {
                    // Different type of whitespace, push the old one, start a new one
                    result.push(current_chunk);
                    current_chunk = c.to_string();
                }
            } else {
                // Transitioning from text to whitespace
                if !current_chunk.is_empty() {
                    result.push(current_chunk);
                }
                current_chunk = c.to_string();
                inside_whitespace = true;
            }
        } else if inside_whitespace {
            // Transitioning from whitespace to text
            result.push(current_chunk);
            current_chunk = c.to_string();
            inside_whitespace = false;
        } else {
            // Continuing with text
            current_chunk.push(c);
        }
    }

    // Don't forget to add the last chunk if there is one
    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }

    result
}

/// Convenience function to chunk lines of text based on the max line length,
/// respecting original whitespace, newlines, and avoiding splitting words across lines.
fn format_text_by_length(
    msg: &dyn Displayable,
    indentation_level: u16,
    max_line_length: usize,
) -> Vec<String> {
    let msg = msg.to_string();
    let indentation_level = usize::from(indentation_level);

    if max_line_length <= indentation_level {
        return vec![];
    }

    let max_line_width = max_line_length - indentation_level;
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in split_on_whitespace_keep_delimiter_grouped(&msg) {
        match word.as_str() {
            // If we encounter a new line character that is a sign to immediately
            // end the line, so we add the character to whatever the line is currently
            // and then add the entire line to the lines vec.
            "\n" => {
                lines.push(current_line.clone());
                current_line = String::new();
            }
            _ => {
                // If the word is just a space character we don't want to preserve it when
                // starting a new line, so we just skip it.
                if current_line.is_empty() && word.len() == 1 && word.starts_with(' ') {
                    continue;
                }

                // If the word we're currently processing doesn't make the line
                // longer than the limit we simply add it to the current_line.
                if (current_line.len() + word.len()) <= max_line_width {
                    current_line += &word;
                    continue;
                }

                // If the word we're processing DOES make the line longer then the
                // limit we first add the current line to the list of lines and then
                // we create a new line and add it to that line.
                lines.push(current_line.clone());
                current_line = String::new();

                // If the word is just a space character we don't want to preserve it when
                // starting a new line, so we just skip it.
                if word.len() == 1 && word.starts_with(' ') {
                    continue;
                }

                current_line += &word;
            }
        }
    }

    // Make sure that the last line is added.
    if !current_line.is_empty() {
        lines.push(current_line.clone());
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
pub fn choose_one(choices: HashMap<String, String>) -> Result<(String, String)> {
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

fn clamp_window(selected: usize, start: usize, len: usize, page_size: usize) -> usize {
    let page = page_size.min(len);
    if len <= page {
        return 0;
    }

    // keep existing start if it’s valid
    let mut s = start.min(len.saturating_sub(page));

    // ensure selected stays visible
    if selected < s {
        s = selected;
    } else if selected >= s + page {
        s = selected + 1 - page;
    }

    s.min(len.saturating_sub(page))
}

fn display_radio_choices(
    choices: &[(&str, bool)],
    selected: usize,
    start_index: usize,
    page_size: usize,
) {
    use std::io::Write;

    let len = choices.len();
    if len == 0 {
        return;
    }

    // Show either the whole list (if it fits) or exactly page_size items.
    let page = page_size.min(len);

    // Clamp the window start so we always have a full page when possible.
    let max_start = len.saturating_sub(page);
    let start_point = start_index.min(max_start);

    // End is start + page (safe because start_point <= max_start).
    let end_point = start_point + page;

    for (i, choice) in choices[start_point..end_point].iter().enumerate() {
        let index = start_point + i; // global index for highlight

        // I know this is weird, but the colored crate doesn't seem to work without
        // doing this hack.
        let prefix = if index == selected {
            ">".blue().to_string()
        } else {
            " ".into()
        };

        let selection = if choice.1 {
            format!("[{}]", "*".green())
        } else {
            "[ ]".into()
        };

        let mut choice_text = if index == selected {
            choice.0.blue().underline().to_string()
        } else {
            choice.0.into()
        };

        if choice.1 && index == selected {
            choice_text = choice.0.green().underline().to_string()
        } else if choice.1 {
            choice_text = choice.0.green().to_string()
        };

        _ = write!(
            std::io::stdout(),
            "{} {} {}\r\n",
            prefix,
            selection,
            choice_text
        );
    }
}

/// Creates a TUI radio selection modal.
/// The values passed in are mutated and the boolean value coupled is changed to true when the user has selected
/// a value.
pub fn choose_many(choices: &mut [(&str, bool)], page_size: usize) -> Result<()> {
    use std::io::Write;

    if choices.is_empty() {
        return Ok(());
    }

    let mut selected_index = 0;
    let mut start_index = clamp_window(selected_index, 0, choices.len(), page_size);

    // initial draw
    display_radio_choices(choices, selected_index, start_index, page_size);

    // Get the standard input stream.
    let stdin = std::io::stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = std::io::stdout().into_raw_mode()?;

    // Always move up by the visible page height
    let up_lines = page_size.min(choices.len()) as u16;

    for key in stdin.keys() {
        match key? {
            Key::Ctrl('c') => break,

            Key::Up if selected_index > 0 => {
                selected_index -= 1;
                start_index = clamp_window(selected_index, start_index, choices.len(), page_size);

                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Up(up_lines),
                    termion::clear::AfterCursor
                )?;
                display_radio_choices(choices, selected_index, start_index, page_size);
            }

            Key::Down if selected_index < choices.len() - 1 => {
                selected_index += 1;
                start_index = clamp_window(selected_index, start_index, choices.len(), page_size);

                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Up(up_lines),
                    termion::clear::AfterCursor
                )?;
                display_radio_choices(choices, selected_index, start_index, page_size);
            }

            Key::Char(' ') => {
                choices[selected_index].1 = !choices[selected_index].1;

                // (harmless) keep window clamped
                start_index = clamp_window(selected_index, start_index, choices.len(), page_size);

                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Up(up_lines),
                    termion::clear::AfterCursor
                )?;
                display_radio_choices(choices, selected_index, start_index, page_size);
            }

            Key::Char('\n') => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Up(up_lines),
                    termion::clear::AfterCursor
                )?;
                write!(stdout, "{}", termion::cursor::Show)?;
                stdout.flush()?;
                return Ok(());
            }

            _ => {}
        }
        stdout.flush()?;
    }

    bail!("display chooser was interrupted before ending properly")
}

#[cfg(test)]
mod tests {
    use crate::{error, format_text_by_length, indent, println, question};
    use rstest::rstest;
    use std::{str::FromStr, thread, time};

    #[test]
    #[ignore]
    fn tree() {
        let options = crate::Options {
            debug: true,
            max_line_length: 40,
            padding: 0,
        };

        let some_flag = "tree".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, options);

        fmt.println(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.error(&"Hello from polyfmt, Look at how well it breaks up lines!");

        let _guard = fmt.indent();
        fmt.spacer();

        fmt.success(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.warning(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.debug(&"Hello from polyfmt, Look at how well it breaks up lines!");
    }

    #[test]
    #[ignore]
    fn spinner() {
        let options = crate::Options {
            debug: true,
            max_line_length: 40,
            padding: 1,
        };
        let ten_millis = time::Duration::from_secs(2);

        let some_flag = "spinner".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, options);

        fmt.print(&"Processing line");
        fmt.println(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.success(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.warning(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.print(&"Look I can stop spinning");
        fmt.println(&"And I can still print things");
        fmt.pause();
        thread::sleep(ten_millis);

        fmt.print(&"Okay lets keep going");
        fmt.resume();
        fmt.println(&"Testing again");
        thread::sleep(ten_millis);

        fmt.debug(&"Hello from polyfmt, Look at how well it breaks up lines!");
        thread::sleep(ten_millis);

        fmt.error(&"Hello from polyfmt, Look at how well it breaks up lines!");
    }

    #[test]
    #[ignore]
    fn json() {
        let options = crate::Options {
            debug: true,
            max_line_length: 80,
            padding: 0,
        };

        let some_flag = "json".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, options);

        fmt.print(&"Demoing! ");
        fmt.println(&"Hello from polyfmt");
        fmt.success(&"This is a successful message!");
        fmt.warning(&"This is a warning message");
        fmt.debug(&"This is a debug message");
        fmt.error(&"This is an error message");
    }

    #[test]
    #[ignore]
    fn plain() {
        let options = crate::Options {
            debug: true,
            max_line_length: 40,
            padding: 0,
        };

        let some_flag = "plain".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let mut fmt = crate::new(format, options);

        fmt.println(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.error(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.success(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.warning(&"Hello from polyfmt, Look at how well it breaks up lines!");
        fmt.debug(&"Hello from polyfmt, Look at how well it breaks up lines!");
    }

    // These tests aren't real tests, I just eyeball things to see if they work.
    // Maybe I'll write real tests, maybe I wont. Shut-up.
    #[test]
    #[ignore]
    fn global_easy() {
        use crate::{indent, print, println, spacer};
        let options = crate::Options {
            debug: true,
            max_line_length: 40,
            padding: 1,
        };

        let some_flag = "tree".to_string();
        let format = crate::Format::from_str(&some_flag).unwrap();

        let fmt = crate::new(format, options);
        crate::set_global_formatter(fmt);

        _ = question!("To setup a brand: ");
        _ = question!("To setup a brand: ");
        println!("Hello, we have a thing that we do");
        _ = question!("To setup a brand: ");

        println!("{} service setup", "Astra");
        spacer!();
        print!("\n");
        println!("{}", "To setup a brand new service we'll need to initialize the infrastructure that allows you \
        to manage and deploy the service. We'll set up the following for your service:\n".to_owned() +
           &format!("  • An {} repository for container management.\n", "ECR (Elastic Container Registry)") +
           &format!("  • {} along with appropriate target groups for efficient traffic distribution.\n", "Load balancers") +
           &format!("  • An {} service definition to manage your containerized applications.\n", "ECS (Elastic Container Service)") +
           &format!("  • A {} to initialize your containers and define core functionality like logging.\n", "starter ECS task definition") +
           &format!("  • Necessary {} for secure, reliable connectivity.\n", "DNS entries and TLS certificates")
        );
        spacer!();
        println!(
            "Astra will attempt to use Terraform to create this infrastructure on your behalf."
        );

        struct Customer {
            id: String,
            name: String,
        }

        let customer = Customer {
            id: "id".to_string(),
            name: "name".to_string(),
        };

        let _guard = indent!();
        error!("Created customer [{}] {}", customer.id, customer.name);
    }

    #[rstest]
    #[case::group_similar_whitespace("Hello, there   beautiful", vec!["Hello,", " ", "there", "   ", "beautiful"])]
    #[case::differ_between_whitespace_types("The quick\nbrown fox", vec!["The", " ", "quick", "\n", "brown", " ", "fox"])]
    #[case::leading_spaces("  Leading spaces", vec!["  ", "Leading", " ", "spaces"])]
    #[case::trailing_spaces("Trailing space ", vec!["Trailing", " ", "space", " "])]
    #[case::only_spaces("   ", vec!["   "])] // Only spaces
    #[case::tabs_and_spaces("Mixed   \t tabs and spaces", vec!["Mixed", "   ", "\t", " ", "tabs", " ", "and", " ", "spaces"])]
    #[case::empty_string("", vec![])] // Empty string
    #[case::trailing_newlines("Sentence then trailingnewlines\n\n", vec!["Sentence", " ", "then", " ", "trailingnewlines", "\n\n"])] // trailing newlines
    fn test_split_on_whitespace_keep_delimiter_grouped(
        #[case] input: &str,
        #[case] expected: Vec<&str>,
    ) {
        let result = crate::split_on_whitespace_keep_delimiter_grouped(input);
        let expected_str: Vec<String> = expected.into_iter().map(String::from).collect();
        assert_eq!(result, expected_str);
    }

    #[rstest]
    #[case::simple("Hello", vec!["Hello"])]
    #[case::proper_length_splitting_on_word("The greatest glory in living lies not in never falling", vec!["The greatest glory in living lies not in", "never falling"])]
    #[case::preserve_new_lines("The greatest\n glory in living\n lies not in never falling", vec!["The greatest", "glory in living", "lies not in never falling"])]
    #[case::preserve_multiple_spaces_on_newline("Hello\n  • Some bullet point here", vec!["Hello", "  • Some bullet point here"])]
    fn test_format_text_length(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert_eq!(format_text_by_length(&input, 0, 40), expected)
    }
}
