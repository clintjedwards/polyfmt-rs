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

    #[cfg(feature = "tui")]
    {
        run_tui_examples();
    }

    #[cfg(not(feature = "tui"))]
    {
        println!("--- TUI helpers (feature disabled) ---");
        println!(
            "Enable the `tui` feature to see choose_one/choose_many in action (e.g. `cargo run --features tui --bin demo`)."
        );
    }

    println!("--- Demo complete ---");
}

fn sleep() {
    thread::sleep(Duration::from_millis(1000));
}

#[cfg(feature = "tui")]
fn run_tui_examples() {
    use polyfmt::tui::{choose_many, choose_one};
    use std::collections::HashMap;

    println!("--- TUI helpers (stdout/tty only) ---");

    let mut choices = HashMap::new();
    choices.insert("Apples".to_string(), "apples".to_string());
    choices.insert("Bananas".to_string(), "bananas".to_string());
    choices.insert("Cherries".to_string(), "cherries".to_string());

    println!("Pick one fruit:");
    if let Ok((label, value)) = choose_one(choices) {
        println!("You picked {label} ({value})");
    }

    let mut toggles = [
        ("Laser Sharks", false),
        ("Hoverboards", true),
        ("Time Travel", false),
        ("Jetpacks", true),
        ("Teleporters", false),
        ("Gravity Boots", false),
        ("Moon Base", true),
        ("Robot Sidekick", false),
        ("Invisibility Cloak", false),
        ("Unlimited Snacks", true),
    ];
    println!("Toggle any options (space to toggle, enter to finish):");
    if let Ok(()) = choose_many(&mut toggles, 4) {
        println!("Final selections: {:?}", toggles);
    }
}
