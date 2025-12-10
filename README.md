# Polyfmt

`polyfmt` is a convenience package that provides multiple forms of formatted output.
Useful for CLI applications where you might want to provide JSON output for machine users,
but pretty output for interactive users.

> An “output facade” for Rust CLI tools that unifies pretty UX (spinners / indentation / trees) and
> machine-friendly JSON, under a single, easy-swap Format abstraction.

<img src="./demo.gif" />

## Why

In a command line application you usually want to provide some well-formatted output to users. This
may include progress bars, timers, spinners, or tables so that interactive users can better parse
your programs output. For non-interactive users or automation this might make your CLI application
difficult to parse, build automation around, or just unnecessarily verbose. To this end, you might want to
provide a common serialization format to users who use your CLI app from within a non-interactive environment.

Polyfmt aims to simplify the API around multiple formatting options and make it easy to switch between them.

## Usage

Polyfmt provides a very simple API full of print functions.

### Using the global formatter

The easiest way to use polyfmt is by using the global formatter:

```rust
use polyfmt::println;
println!("Hello from polyfmt");
```

This is good for simple implementations but obviously the whole point of this library is being able to switch
formatters. To do this you can still use a global formatter. (Which is available wherever polyfmt is imported)

### Altering the global formatter

Initiate a new formatter instance, passing in what type of formatter you want back. This is usually passed in
by your user at runtime via flags or config.

```rust
use polyfmt::{new, Format, Options, println};
let fmt = polyfmt::new(Format::Plain, Options::default());
polyfmt::set_global_formatter(fmt);
// Use the returned formatter to print a simple string.
println!("something");
// Output: `something`
```

### Using a scoped formatter

Lastly you might want to just use a scoped formatter for specific instances. To do this you can just directly
use the formatter you get back from the new function:

```rust
use polyfmt::{new, Format, Options};
let mut fmt = polyfmt::new(Format::Plain, Options::default());
fmt.print(&"test");
```

### Feature flags

- `tui` *(off by default)*: opt into interactive helpers such as `polyfmt::tui::choose_one` and
  `polyfmt::tui::choose_many`. These operate directly on stdout/tty and aren’t suitable for custom
  output targets. Enable them with:

```toml
[dependencies]
polyfmt = { version = "0.0.13", features = ["tui"] }
```

### Tuning `Options`

`Options::default()` gets you sensible defaults (no debug output, auto max line length based on terminal width, zero
padding, line-buffered stdout). You can tune individual fields with builder-style helpers:

```rust
use polyfmt::{Format, Options};

let opts = Options::default()
    .with_debug(true)
    .with_max_line_length(60)
    .with_padding(2)
    .with_custom_output_target(std::fs::File::create("out.log")?);
```

Builder notes:
- `with_debug(bool)`: turn debug lines on/off (default: off).
- `with_max_line_length(usize)`: override line wrapping length (default: terminal width minus a small margin).
- `with_padding(u16)`: add leading spaces before output (default: 0).
- `with_custom_output_target`: send output to any writer (files, buffers, etc.). Spinner
  falls back to plain when using a custom target because spinners only make sense on a TTY.

### Filtering output

Sometimes you'll want to output something only for specific formatters.
You can use the [only](Formatter::only) function to list formatters for which
the following print command will only print for those formatters.

```rust
use polyfmt::{new, Format, Options};
let mut fmt = polyfmt::new(Format::Plain, Options::default());

fmt.only(vec![Format::Plain]).print(&"test");
// This will only print the string "test" if the formatter Format is "Plain".
```

The global macros also allow you to list formats to whitelist on the fly:

```rust
use polyfmt::{print, Format};
print!("test"; vec![Format::Plain, Format::Tree])
```

### Dynamically choosing a format

Polyfmt is meant to be used as a formatter that is easy to be changed by the user.
So most likely you'll want to automatically figure out which formatter you want from
a flag of env_var the user passes in.

```rust
use polyfmt::{new, Format, Options};
use std::str::FromStr;

let some_flag = "plain".to_string(); // case-insensitive
let format = Format::from_str(&some_flag).unwrap();
let mut fmt = new(format, Options::default());
```

### Redirecting output (stdout, files, buffers)

Polyfmt now lets you pick where output goes. By default everything is written to stdout with line-buffering. You can
point output at any `Write + Send + Sync` target (e.g., file, buffer, socket) via `Options::with_custom_output_target`:

```rust
use polyfmt::{new, Format, Options};
use std::fs::File;

let file = File::create("polyfmt.log")?;
let opts = Options::default().with_custom_output_target(file);
let mut fmt = new(Format::Plain, opts);
fmt.println(&"Hello to a file");
fmt.finish();
```

Spinner output only makes sense on a TTY; if you choose `Format::Spinner` with a custom output target, polyfmt will
fall back to the plain formatter.

### Indentation

Polyfmt supports indentation also with a similar implementation to spans in the tracing crate
You initialize the indent, tie it to a guard, and then once that guard drops out of scope the
indentation level will decrement.

```rust

# use polyfmt::{indent, println};

println!("This line is base level of indentation.");
let _guard = indent!();
println!("This line has a greater indentation than the previous line.");
drop(_guard);
println!("This line has the same indentation level as the first.");
```

### Additional Details

- You can turn off color by using the popular `NO_COLOR` environment variable.
- Output defaults to line-buffered stdout; use `Options::with_custom_output_target(...)` to redirect to files or other
  sinks.
- Anything to be printed must implement Display and Serialize due to the need to possibly print it into both plaintext
  and json.
