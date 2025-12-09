use crate::{take_and_check_allowed, Displayable, Format, Formatter, IndentGuard, Options};
use serde_json::json;
use std::sync::{Arc, Mutex, Weak};
use std::{collections::HashSet, io::Write};

#[derive(Clone)]
pub struct Json {
    pub debug: bool,
    allowed_formats: HashSet<Format>,
    output_target: Arc<Mutex<dyn Write + Send>>,
}

impl Json {
    pub fn new(options: Options) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Json {
            debug: options.debug,
            allowed_formats: HashSet::new(),
            output_target: options.output_target.target,
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
        if !take_and_check_allowed(Format::Json, &mut self.allowed_formats) {
            return;
        }

        let tmp = json!({
            "label": "info",
            "data": msg.as_serialize(),
        });

        let mut output_target = self.output_target.lock().unwrap();

        let _ = match serde_json::to_string(&tmp) {
            Ok(s) => writeln!(output_target, "{s}"),
            Err(e) => writeln!(output_target, "Error serializing to JSON: {e:?}"),
        };
    }

    fn println(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Json, &mut self.allowed_formats) {
            return;
        }

        let tmp = json!({
            "label": "info",
            "data": msg.as_serialize(),
        });

        let mut output_target = self.output_target.lock().unwrap();

        let _ = match serde_json::to_string(&tmp) {
            Ok(s) => writeln!(output_target, "{s}"),
            Err(e) => writeln!(output_target, "Error serializing to JSON: {e:?}"),
        };
    }

    fn error(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Json, &mut self.allowed_formats) {
            return;
        }

        let tmp = json!({
            "label": "error",
            "data": msg.as_serialize(),
        });

        let mut output_target = self.output_target.lock().unwrap();

        let _ = match serde_json::to_string(&tmp) {
            Ok(s) => writeln!(output_target, "{s}"),
            Err(e) => writeln!(output_target, "Error serializing to JSON: {e:?}"),
        };
    }

    fn success(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Json, &mut self.allowed_formats) {
            return;
        }

        let tmp = json!({
            "label": "success",
            "data": msg.as_serialize(),
        });

        let mut output_target = self.output_target.lock().unwrap();

        let _ = match serde_json::to_string(&tmp) {
            Ok(s) => writeln!(output_target, "{s}"),
            Err(e) => writeln!(output_target, "Error serializing to JSON: {e:?}"),
        };
    }

    fn warning(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Json, &mut self.allowed_formats) {
            return;
        }

        let tmp = json!({
            "label": "warning",
            "data": msg.as_serialize(),
        });

        let mut output_target = self.output_target.lock().unwrap();

        let _ = match serde_json::to_string(&tmp) {
            Ok(s) => writeln!(output_target, "{s}"),
            Err(e) => writeln!(output_target, "Error serializing to JSON: {e:?}"),
        };
    }

    fn debug(&mut self, msg: &dyn Displayable) {
        if !take_and_check_allowed(Format::Json, &mut self.allowed_formats) || !self.debug {
            return;
        }

        let tmp = json!({
            "label": "debug",
            "data": msg.as_serialize(),
        });

        let mut output_target = self.output_target.lock().unwrap();

        let _ = match serde_json::to_string(&tmp) {
            Ok(s) => writeln!(output_target, "{s}"),
            Err(e) => writeln!(output_target, "Error serializing to JSON: {e:?}"),
        };
    }

    fn indent(fmtter: &Arc<Mutex<Self>>) -> Box<dyn IndentGuard> {
        let cloned_tree = Arc::clone(fmtter);
        let guard = Guard::new(cloned_tree);
        Box::new(guard)
    }

    fn outdent(&mut self) {}

    fn spacer(&mut self) {}

    #[allow(dead_code)]
    fn pause(&mut self) {}

    #[allow(dead_code)]
    fn start(&mut self) {}

    fn question(&mut self, msg: &dyn Displayable) -> String {
        if !take_and_check_allowed(Format::Json, &mut self.allowed_formats) {
            return "".to_string();
        }

        let tmp = json!({
            "label": "question",
            "data": msg.as_serialize(),
        });

        let mut output_target = self.output_target.lock().unwrap();

        let _ = match serde_json::to_string(&tmp) {
            Ok(s) => writeln!(output_target, "{s}"),
            Err(e) => writeln!(output_target, "Error serializing to JSON: {e:?}"),
        };

        output_target.flush().unwrap();
        drop(output_target);

        let mut input = String::from("");

        let _ = std::io::stdin().read_line(&mut input);

        input.trim().to_string()
    }

    fn only(&mut self, types: Vec<Format>) -> &mut Self {
        self.allowed_formats = types.into_iter().collect();
        self
    }

    fn finish(&self) {
        if let Ok(mut out) = self.output_target.lock() {
            let _ = out.flush();
        }
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

    fn pause(&mut self) {}

    fn resume(&mut self) {}

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
