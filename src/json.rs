use crate::{Displayable, Format, Formatter, IndentGuard};
use scopeguard::defer;
use serde_json::json;
use std::io::Write;

struct Guard;

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        todo!()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Json {
    pub debug: bool,
    indentation: Vec<String>,
    allowed_formats: Vec<Format>,
}

impl Formatter for Json {
    fn print(&mut self, msg: &dyn Displayable) {
        if self.allowed_formats.contains(&Format::Plain) {
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        let tmp = json!({
            "label": "info",
            "data": msg.as_serialize(),
        });

        match serde_json::to_string(&tmp) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("Error serializing to JSON: {e:?}"),
        }
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if self.allowed_formats.contains(&Format::Plain) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        let tmp = json!({
            "label": "info",
            "data": msg.as_serialize(),
        });

        match serde_json::to_string(&tmp) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("Error serializing to JSON: {e:?}"),
        }
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if self.allowed_formats.contains(&Format::Plain) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        let tmp = json!({
            "label": "error",
            "data": msg.as_serialize(),
        });

        match serde_json::to_string(&tmp) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("Error serializing to JSON: {e:?}"),
        }
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if self.allowed_formats.contains(&Format::Plain) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        let tmp = json!({
            "label": "success",
            "data": msg.as_serialize(),
        });

        match serde_json::to_string(&tmp) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("Error serializing to JSON: {e:?}"),
        }
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if self.allowed_formats.contains(&Format::Plain) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        let tmp = json!({
            "label": "warning",
            "data": msg.as_serialize(),
        });

        match serde_json::to_string(&tmp) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("Error serializing to JSON: {e:?}"),
        }
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if (self.allowed_formats.contains(&Format::Plain) && !self.allowed_formats.is_empty())
            || !self.debug
        {
            return;
        }

        defer! {
            self.allowed_formats = vec![];
        }

        let tmp = json!({
            "label": "debug",
            "data": msg.as_serialize(),
        });

        match serde_json::to_string(&tmp) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("Error serializing to JSON: {e:?}"),
        }
    }

    fn indent(&mut self) -> Box<dyn IndentGuard> {
        self.indentation.push("  ".to_string());
        Box::new(Guard {})
    }

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if self.allowed_formats.contains(&Format::Plain) && !self.allowed_formats.is_empty() {
            return "".to_string();
        }

        defer! {
            self.allowed_formats = vec![];
        }

        let tmp = json!({
            "label": "question",
            "data": msg.as_serialize(),
        });

        match serde_json::to_string(&tmp) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("Error serializing to JSON: {e:?}"),
        }

        std::io::stdout().flush().unwrap();

        let mut input = String::from("");

        let _ = std::io::stdin().read_line(&mut input);

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut dyn Formatter {
        self.allowed_formats = types;
        self
    }

    fn finish(&self) {
        std::io::stdout().flush().unwrap();
    }
}
