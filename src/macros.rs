/// Print a normal message.
///
/// # Examples
///
/// ```
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

#[macro_export]
macro_rules! indent {
    () => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.indent()
    }};
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
/// let name = "Clint";
/// question!("Hello, {name}");
/// question!("Hello Clint");
/// question!("Hello, {}", name; vec![Format::Plain])
/// let input = question!("Hello, {}", name);
/// ```
#[macro_export]
macro_rules! question {
    // Allows a simple format style string, with some arguments or none.
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.question(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Allows a simple format style string with some arguments or none and also
    // accounts for if the user wants to insert a formatter filter.
    ($s:expr $(, $args:expr)* ; $formats:expr) => {{
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.only($formats).question(&format!("{}", format_args!($s, $($args),*)));
    }};
}

/// Print a debug message that only shows up if debug mode is on.
///
/// # Examples
///
/// ```
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
