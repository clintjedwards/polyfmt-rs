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
macro_rules! println {
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.println(&format!("{}", format_args!($s, $($arg),*)));
    });

    ($s:expr, $($arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.println(&format!("{}", format_args!($s, $($arg),*)));
    });

    // Variant with format specification, expecting a tuple as the last argument
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
/// ```
#[macro_export]
macro_rules! success {
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.success(&format!("{}", format_args!($s, $($arg),*)));
    });

    ($s:expr, $($arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.success(&format!("{}", format_args!($s, $($arg),*)));
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
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.error(&format!("{}", format_args!($s, $($arg),*)));
    });

    ($s:expr, $($arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.error(&format!("{}", format_args!($s, $($arg),*)));
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
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.warning(&format!("{}", format_args!($s, $($arg),*)));
    });

    ($s:expr, $($arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.warning(&format!("{}", format_args!($s, $($arg),*)));
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
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.question(&format!("{}", format_args!($s, $($arg),*)))
    });

    ($s:expr, $($arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.question(&format!("{}", format_args!($s, $($arg),*)))
    });
}

#[macro_export]
macro_rules! debug {
    ($s:expr $(, $arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.debug(&format!("{}", format_args!($s, $($arg),*)));
    });

    ($s:expr, $($arg:expr),*) => ({
        let global_fmtter = $crate::get_global_formatter();
        let mut fmt = global_fmtter.lock().unwrap();
        fmt.debug(&format!("{}", format_args!($s, $($arg),*)));
    });
}
