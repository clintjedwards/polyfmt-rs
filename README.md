# Polyfmt

`polyfmt` is a convenience package that provides multiple forms of formatted output.
Useful for CLI applications where you might want to provide JSON output for machine users,
but pretty output for interactive users.

<img src="./demo.gif" />

## Why

In a command line application you usually want to provide some well-formatted output to users. This
may include progress bars, timers, spinners, or tables so that interactive users can better parse
your programs output. For non-interactive users or automation this might make your CLI application
difficult to parse, build automation around, or just unnecessarily verbose. To this end, you might want to
provide a common serialization format to users who use your CLI app from within a non-interactive environment.

Polyfmt aims to simplify the API around multiple formatting options and make it easy to switch between them.

## Usage

Polyfmt provides a very simple API, full of print functions.

Initiate a new formatter instance, passing in what type of formatter you want back. This is usually passed in
by your user at runtime via flags or config.

```rust
use polyfmt::{new, Format, Options};

let mut fmt = polyfmt::new(Format::Plain, Options::default()).unwrap();

// Use the returned formatter to print a simple string.

fmt.print(&"something");
```

Output: `something`

Sometimes you'll want to output something only for specific formatters.
You can use the [only](Formatter::only) function to list formatters for which
the following print command will only print for those formatters.

```rust
...
fmt.only(vec![Format::Plain]).print(&"test");
```

Polyfmt is meant to be used as a formatter that is easy to be changed by the user.
So most likely you'll want to automatically figure out which formatter you want from
a flag of env_var the user passes in.

```rust
let some_flag = "plain".to_string(); // case-insensitive
let format = Format::from_str(&some_flag).unwrap();

let mut fmt = new(format, options).unwrap();
```

### Additional Details

You can turn off color by using the popular `NO_COLOR` environment variable.
