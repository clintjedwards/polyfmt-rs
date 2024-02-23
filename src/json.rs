use crate::{Displayable, Format, Formatter, IndentGuard};
use scopeguard::defer;
use serde_json::json;
use std::sync::{Arc, Mutex, Weak};
use std::{collections::HashSet, io::Write};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Json {
    pub debug: bool,
    allowed_formats: HashSet<Format>,
}

impl Json {
    pub fn new(debug: bool) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Json {
            debug,
            ..Default::default()
        }))
    }
}

struct Guard {
    fmtter: Weak<Mutex<Json>>,
}

impl Guard {
    fn new(fmtter: Arc<Mutex<Json>>) -> Self {
        Self {
            fmtter: Arc::downgrade(&fmtter),
        }
    }
}

impl IndentGuard for Guard {}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(fmtter) = self.fmtter.upgrade() {
            let mut fmtter_lock = fmtter.lock().unwrap();
            fmtter_lock.outdent();
        }
    }
}

impl Json {
    fn print(&mut self, msg: &dyn Displayable) {
        if self.allowed_formats.contains(&Format::Json) {
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
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
        if self.allowed_formats.contains(&Format::Json) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
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
        if self.allowed_formats.contains(&Format::Json) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
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
        if self.allowed_formats.contains(&Format::Json) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
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
        if self.allowed_formats.contains(&Format::Json) && !self.allowed_formats.is_empty() {
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
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
        if (self.allowed_formats.contains(&Format::Json) && !self.allowed_formats.is_empty())
            || !self.debug
        {
            return;
        }

        defer! {
            self.allowed_formats = HashSet::new();
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

    fn indent(fmtter: &Arc<Mutex<Self>>) -> Box<dyn IndentGuard> {
        let cloned_tree = Arc::clone(fmtter);
        let guard = Guard::new(cloned_tree);
        Box::new(guard)
    }

    fn outdent(&mut self) {}

    fn spacer(&mut self) {}

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if self.allowed_formats.contains(&Format::Json) && !self.allowed_formats.is_empty() {
            return "".to_string();
        }

        defer! {
            self.allowed_formats = HashSet::new();
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

    fn only(&mut self, types: Vec<Format>) -> &mut Self {
        self.allowed_formats = types.into_iter().collect();
        self
    }

    fn finish(&self) {
        std::io::stdout().flush().unwrap();
    }
}

impl Formatter for Arc<Mutex<Json>> {
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
        Json::indent(self)
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
