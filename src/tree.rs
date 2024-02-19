use crate::{format_text_length, is_allowed, Displayable, Format, Formatter, IndentGuard};
use colored::Colorize;
use scopeguard::defer;
use std::collections::HashSet;
use std::io::Write;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tree {
    debug: bool,
    indentation_level: u16,
    max_line_length: usize,
    allowed_formats: HashSet<Format>,

    header_printed: bool,
}

impl Tree {
    pub fn new(debug: bool, max_line_length: usize) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Tree {
            debug,
            max_line_length,
            ..Default::default()
        }))
    }
}

struct Guard {
    tree: Weak<Mutex<Tree>>,
}

impl Guard {
    fn new(tree: Arc<Mutex<Tree>>) -> Self {
        Self {
            tree: Arc::downgrade(&tree),
        }
    }
}

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(tree) = self.tree.upgrade() {
            let mut tree_lock = tree.lock().unwrap();
            tree_lock.outdent();
        }
    }
}

impl Tree {
    fn print(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Tree, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        print!("{} {msg}", "├─".magenta());

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Tree, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        let lines = format_text_length(msg, self.indentation_level, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        if self.header_printed {
            println!(
                "{}{} {}",
                "├─".magenta(),
                format!("{}", "─".magenta()).repeat(self.indentation_level.into()),
                lines.first().unwrap_or(&"".to_string()),
            );
        } else {
            println!(
                "{}{} {}",
                "┌─".magenta(),
                format!("{}", "─".magenta()).repeat(self.indentation_level.into()),
                lines.first().unwrap_or(&"".to_string()),
            );
            self.header_printed = true;
        }

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            println!(
                "{}{} {}",
                "│ ".magenta(),
                " ".repeat(self.indentation_level.into()),
                line
            );
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Tree, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        let lines = format_text_length(msg, self.indentation_level + 2, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        println!(
            "{}{} {} {}",
            "├─".magenta(),
            format!("{}", "─".magenta()).repeat(self.indentation_level.into()),
            "x".red(),
            lines.first().unwrap_or(&"".to_string())
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            println!(
                "{}{} {}",
                "│ ".magenta(),
                " ".repeat(self.indentation_level.into()),
                line
            );
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Tree, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        let lines = format_text_length(msg, self.indentation_level + 2, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        println!(
            "{}{} {} {}",
            "├─".magenta(),
            format!("{}", "─".magenta()).repeat(self.indentation_level.into()),
            "✓".green(),
            lines.first().unwrap_or(&"".to_string())
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            println!(
                "{}{} {}",
                "│ ".magenta(),
                " ".repeat(self.indentation_level.into()),
                line
            );
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Tree, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return;
        }

        let lines = format_text_length(msg, self.indentation_level + 3, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        println!(
            "{}{} {} {}",
            "├─".magenta(),
            format!("{}", "─".magenta()).repeat(self.indentation_level.into()),
            "!!".yellow(),
            lines.first().unwrap_or(&"".to_string())
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            println!(
                "{}{} {}",
                "│ ".magenta(),
                " ".repeat(self.indentation_level.into()),
                line
            );
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if !is_allowed(Format::Tree, &self.allowed_formats) || !self.debug {
            self.allowed_formats = HashSet::new();
            return;
        }

        let lines = format_text_length(msg, self.indentation_level + 8, self.max_line_length);

        if lines.is_empty() {
            return;
        }

        println!(
            "{}{} {} {}",
            "├─".magenta(),
            format!("{}", "─".magenta()).repeat(self.indentation_level.into()),
            "[debug]".dimmed(),
            lines.first().unwrap_or(&"".to_string())
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            println!(
                "{}{} {}",
                "│ ".magenta(),
                " ".repeat(self.indentation_level.into()),
                line
            );
        }

        defer! {
            self.allowed_formats = HashSet::new();
        }
    }

    fn indent(tree: &Arc<Mutex<Self>>) -> Box<dyn IndentGuard> {
        let mut fmt = tree.lock().unwrap();
        fmt.indentation_level += 1;
        drop(fmt);
        let cloned_tree = Arc::clone(tree);
        let guard = Guard::new(cloned_tree);
        Box::new(guard)
    }

    fn outdent(&mut self) {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
        }
    }

    fn spacer(&mut self) {
        println!("{}", "┊".magenta(),);
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !is_allowed(Format::Tree, &self.allowed_formats) {
            self.allowed_formats = HashSet::new();
            return "".to_string();
        }

        let lines = format_text_length(msg, self.indentation_level + 2, self.max_line_length);

        println!(
            "{}{} {} {}",
            "├─".magenta(),
            format!("{}", "─".magenta()).repeat(self.indentation_level.into()),
            "?".magenta(),
            lines.first().unwrap_or(&"".to_string())
        );

        // Print the remaining lines
        for line in lines.iter().skip(1) {
            println!(
                "{}{} {}",
                "│ ".magenta(),
                " ".repeat(self.indentation_level.into()),
                line
            );
        }

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");

        let _ = std::io::stdin().read_line(&mut input);

        defer! {
            self.allowed_formats = HashSet::new();
        }

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut Self {
        self.allowed_formats = types.into_iter().collect();
        self
    }

    fn finish(&self) {
        std::io::stdout().flush().unwrap();
    }
}

impl Formatter for Arc<Mutex<Tree>> {
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
        Tree::indent(self)
    }

    fn outdent(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.outdent();
    }

    fn spacer(&mut self) {
        let mut fmt = self.lock().unwrap();
        fmt.spacer();
    }

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
