#[macro_export]
macro_rules! print {
    ($displayable:expr) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.print(&$displayable)
    }};
    ($displayable:expr, $($only_array:expr),+) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.only(vec![$($only_array),*]).print(&$displayable)
    }};
}

#[macro_export]
macro_rules! println {
    ($displayable:expr) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.println(&$displayable)
    }};
    ($displayable:expr, $($only_array:expr),+) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.only(vec![$($only_array),*]).println(&$displayable)
    }};
}

#[macro_export]
macro_rules! err {
    ($displayable:expr) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.err(&$displayable)
    }};
    ($displayable:expr, $($only_array:expr),+) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.only(vec![$($only_array),*]).err(&$displayable)
    }};
}

#[macro_export]
macro_rules! success {
    ($displayable:expr) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.success(&$displayable)
    }};
    ($displayable:expr, $($only_array:expr),+) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.only(vec![$($only_array),*]).success(&$displayable)
    }};
}

#[macro_export]
macro_rules! warning {
    ($displayable:expr) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.warning(&$displayable)
    }};
    ($displayable:expr, $($only_array:expr),+) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.only(vec![$($only_array),*]).warning(&$displayable)
    }};
}

#[macro_export]
macro_rules! debugln {
    ($displayable:expr) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.debugln(&$displayable)
    }};
    ($displayable:expr, $($only_array:expr),+) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.only(vec![$($only_array),*]).debugln(&$displayable)
    }};
}

#[macro_export]
macro_rules! question {
    ($displayable:expr) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.question(&$displayable)
    }};
    ($displayable:expr, $($only_array:expr),+) => {{
        let mut formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.only(vec![$($only_array),*]).question(&$displayable)
    }};
}

#[macro_export]
macro_rules! finish {
    () => {{
        let formatter = $crate::get_global_formatter().lock().unwrap();
        formatter.finish()
    }};
}
