//! Interactive TUI helpers for polyfmt.
//!
//! These helpers drive a raw-mode terminal UI directly against stdout using
//! `termion`. They only make sense on a real TTY and are therefore behind the
//! `tui` feature flag.

use anyhow::{bail, Result};
use colored::Colorize;
use std::collections::HashMap;
use std::io::Write;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

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

/// Creates a TUI multiple choice modal.
///
/// The HashMap passed in is the mapping of label to actual raw value. This is helpful when you want the raw value
/// for passing in to another function but the label to display to the user. Returns the (label, value) tuple that
/// the user chose.
///
/// This helper interacts directly with stdout/tty and only works when the `tui` feature is enabled.
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

/// Creates a TUI radio selection modal.
///
/// The values passed in are mutated and the boolean value coupled is changed to true when the user has selected
/// a value.
///
/// This helper interacts directly with stdout/tty and only works when the `tui` feature is enabled.
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
