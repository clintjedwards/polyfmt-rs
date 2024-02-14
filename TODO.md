- Autodetect TTY
- No color support
- Figure out a way to not have polyfmt always need to be mutable.
- Drop doesn't seem to work.
- We could possibly gate json behind a flag so if you don't need it you don't have to pull in serde.
- I wasn't able to get drop to properly handle finish_and_clear. FOr the global it would run immediately.
- Implement all the drops
- Finish the tree concrete implementation

Actions supported:

- Have pretty command line output
- Have command line output that can be changed via an env var to other types of output
- Be able to define which of those outputs are printed on every invocation
