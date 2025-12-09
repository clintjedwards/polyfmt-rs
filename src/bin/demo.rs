use polyfmt::{new, Format, Options};
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Plain formatter (debug off) ---");
    let mut plain = new(Format::Plain, Options::default());
    plain.println(&"Hello from polyfmt!");
    {
        let _guard = plain.indent();
        plain.println(&"This line is indented.");
    }
    plain.debug(&"You should NOT see this because debug is off.");
    plain.finish();
    sleep();

    println!("--- Plain formatter (debug on) ---");
    let mut plain_debug = new(Format::Plain, Options::default().with_debug(true));
    plain_debug.println(&"Hello with debug enabled.");
    plain_debug.debug(&"You should see this debug line.");
    plain_debug.finish();
    sleep();

    println!("--- Tree formatter ---");
    let mut tree = new(Format::Tree, Options::default());
    tree.println(&"root node");
    {
        let _guard = tree.indent();
        tree.println(&"child node");
        tree.success(&"child succeeded");
        tree.warning(&"child warning");
    }
    tree.finish();
    sleep();

    println!("--- JSON formatter ---");
    let mut json = new(Format::Json, Options::default());
    json.println(&"plain string");
    json.success(&"ok");
    json.warning(&"heads up");
    json.finish();
    sleep();

    println!("--- Spinner formatter ---");
    let mut spinner = new(
        Format::Spinner,
        Options::default()
            .with_debug(true)
            .with_padding(1)
            .with_max_line_length(60),
    );
    spinner.print(&"Working...");
    thread::sleep(Duration::from_millis(1400));
    spinner.print(&"Still going");
    spinner.println(&"Working hard");
    thread::sleep(Duration::from_millis(1400));
    spinner.success(&"Done");
    spinner.finish();

    println!("--- Demo complete ---");
}

fn sleep() {
    thread::sleep(Duration::from_millis(1000));
}
