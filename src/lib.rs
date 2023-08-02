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
//! ```
//!
//! Alternatively, the global macros allow you to variadically list formats much like the only function:
//!
//! ```rust
//! # use polyfmt::{print, Format};
//! print!("test", Format::Plain, Format::Pretty)
//! ```
//!
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
//! You can turn off color by using the popular `NO_COLOR` environment variable.

mod json;
mod macros;
mod plain;
mod pretty;
mod silent;

use once_cell::sync::Lazy;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::sync::Mutex;
use strum::EnumString;

#[derive(Debug, EnumString, Clone, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
pub enum Format {
    /// Outputs text in a humanized fashion without spinners.
    Plain,

    /// Outputs text in a more humanized fashion and provides spinners for longer actions.
    Pretty,
    // /// Outputs json formatted text, mainly suitable to be read by computers.
    Json,
    // /// Dummy formatter that doesn't print anything, can be used when users don't want any
    // /// output at all.
    Silent,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Options {
    /// Turn on printing for debug lines
    pub debug: bool,
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
/// Most functions inside take just about any type and attempt to print them.
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
    fn err(&mut self, msg: &dyn Displayable);

    /// Prints the message noting it as an error to the user.
    fn success(&mut self, msg: &dyn Displayable);

    /// Prints the message noting it as a warning to the user.
    fn warning(&mut self, msg: &dyn Displayable);

    /// Prints a message only if debug is turned on in the formatter options.
    fn debugln(&mut self, msg: &dyn Displayable);

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
    let format = Format::Pretty;
    let options = Options::default();
    Mutex::new(new(format, options).unwrap())
});

/// Set the global formatter to a custom formatter.
pub fn set_global_formatter(formtter: Box<dyn Formatter>) {
    *GLOBAL_FORMATTER.lock().unwrap() = formtter;
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
    options: Options,
) -> Result<Box<dyn Formatter>, Box<dyn Error + Send + Sync>> {
    match format {
        Format::Plain => {
            let mut formatter = plain::Plain::default();
            formatter.debug = options.debug;
            Ok(Box::new(formatter))
        }
        Format::Pretty => {
            let mut formatter = pretty::Pretty::default();
            formatter.debug = options.debug;
            Ok(Box::new(formatter))
        }
        Format::Json => {
            let mut formatter = json::Json::default();
            formatter.debug = options.debug;
            Ok(Box::new(formatter))
        }
        Format::Silent => {
            let formatter = silent::Silent {};
            Ok(Box::new(formatter))
        }
    }
}

/// Convenience function to determine if format should run based on allowed formats.
fn is_allowed(current_format: Format, allowed_formats: &Vec<Format>) -> bool {
    if !allowed_formats.contains(&current_format) && !allowed_formats.is_empty() {
        return false;
    }

    true
}

// #[cfg(test)]
// mod tests {

//     use serde::Serialize;

//     use super::{print, println, *};
//     use std::{str::FromStr, thread, time};

//     // These tests aren't real tests, I just eyeball things to see if they work.
//     // Maybe I'll write real tests, maybe I wont. Shut-up.
//     #[test]
//     fn global_easy() {
//         let options = Options { debug: true };
//         let ten_millis = time::Duration::from_secs(2);

//         let some_flag = "pretty".to_string();
//         let format = Format::from_str(&some_flag).unwrap();

//         let fmt = new(format, options).unwrap();
//         set_global_formatter(fmt);

//         print!("Demoing!");
//         // question!("Test Question");

//         println!("Hello from polyfmt");

//         thread::sleep(ten_millis);
//         success!("This is a successful message!");
//         thread::sleep(ten_millis);
//         warning!("This is a warning message");
//         thread::sleep(ten_millis);
//         debugln!("This is a debug message");
//         err!("This is an error message");
//         finish!();
//     }

// #[test]
// fn it_works() {
//     #[derive(Debug, Serialize)]
//     pub struct Test {
//         pub test: String,
//     }

//     let options = Options { debug: true };
//     let ten_millis = time::Duration::from_secs(1);

//     let some_flag = "pretty".to_string();
//     let format = Format::from_str(&some_flag).unwrap();

//     let mut fmt = new(format, options).unwrap();

//     fmt.print(&"Demoing!");
//     fmt.println(&"Hello from polyfmt");
//     // fmt.question(&"What is your name?\n");

//     thread::sleep(ten_millis);
//     fmt.success(&"This is a successful message!");
//     thread::sleep(ten_millis);
//     fmt.warning(&"This is a warning message");
//     thread::sleep(ten_millis);
//     fmt.debugln(&"This is a debug message");
//     fmt.err(&"This is an error message");
//     fmt.finish();
// }
// }
