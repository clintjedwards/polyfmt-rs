/// Print a normal message.
///
/// # Examples
///
/// ```
/// # use polyfmt::{print, Format};
/// let name = "Clint";
/// print!("Hello, {name}");
/// print!("Hello Clint");
/// print!("Hello, {}", name);
/// print!("Hello, {}", name; vec![Format::Plain])
/// ```
#[macro_export]
macro_rules! print {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.print(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).print(&format!("{}", format_args!($s, $($args),*)));
    }};
}

/// Print a normal message with a newline and multiple lines support.
///
/// # Examples
///
/// ```
/// # use polyfmt::{println, Format};
/// let name = "Clint";
/// println!("Hello, {name}");
/// println!("Hello Clint");
/// println!("Hello, {}", name);
/// println!("Hello, {}", name; vec![Format::Plain])
/// ```
#[macro_export]
macro_rules! println {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.println(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).println(&format!("{}", format_args!($s, $($args),*)));
    }};
}

/// Print a success message with a check-mark.
///
/// # Examples
///
/// ```
/// # use polyfmt::{success, Format};
/// let name = "Clint";
/// success!("Hello, {name}");
/// success!("Hello Clint");
/// success!("Hello, {}", name);
/// success!("Hello, {}", name; vec![Format::Plain])
/// ```
#[macro_export]
macro_rules! success {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.success(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).success(&format!("{}", format_args!($s, $($args),*)));
    }};
}

/// Print an error message with a red x.
///
/// # Examples
///
/// ```
/// # use polyfmt::{error, Format};
/// let name = "Clint";
/// error!("Hello, {name}");
/// error!("Hello Clint");
/// error!("Hello, {}", name);
/// error!("Hello, {}", name; vec![Format::Plain])
/// ```
#[macro_export]
macro_rules! error {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.error(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).error(&format!("{}", format_args!($s, $($args),*)));
    }};
}

/// Increase the indentation level.
///
/// # Examples
///
/// ```
/// # use polyfmt::{println, indent};
/// println!("Some text");
/// let _guard = indent!();
/// println!("This text is more indented than the above");
/// // Output:
/// // Some text
/// //   This text is more indented than the above
/// ```
#[macro_export]
macro_rules! indent {
    () => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.indent()
    }};
}

/// Print a spacer line, determined by the [`Formatter`].
///
/// # Examples
///
/// ```
/// # use polyfmt::spacer;
/// spacer!();
/// ```
#[macro_export]
macro_rules! spacer {
    () => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.spacer();
    }};
}

/// Print an warning message with an exclamation mark.
///
/// # Examples
///
/// ```
/// # use polyfmt::{warning, Format};
/// let name = "Clint";
/// warning!("Hello, {name}");
/// warning!("Hello Clint");
/// warning!("Hello, {}", name);
/// warning!("Hello, {}", name; vec![Format::Plain])
/// ```
#[macro_export]
macro_rules! warning {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.warning(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).warning(&format!("{}", format_args!($s, $($args),*)));
    }};
}

/// Print a question which waits for user input.
///
/// # Examples
///
/// ```
/// # use polyfmt::{question, Format};
/// let name = "Clint";
/// question!("Hello, {name}");
/// question!("Hello Clint");
/// question!("Hello, {}", name; vec![Format::Plain]);
/// let input = question!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! question {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.question(&format!("{}", format_args!($s, $($arg),*)))
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).question(&format!("{}", format_args!($s, $($args),*)))
    }};
}

/// Print a debug message that only shows up if debug mode is on.
///
/// # Examples
///
/// ```
/// # use polyfmt::{debug, Format};
/// let name = "Clint";
/// debug!("Hello, {name}");
/// debug!("Hello Clint");
/// debug!("Hello, {}", name);
/// debug!("Hello, {}", name; vec![Format::Plain])
/// ```
#[macro_export]
macro_rules! debug {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.debug(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).debug(&format!("{}", format_args!($s, $($args),*)));
    }};
}
