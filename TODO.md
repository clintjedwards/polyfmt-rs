- Autodetect TTY
- No color support
- Figure out a way to not have polyfmt always need to be mutable.
- In Go we can attempt to Marshal and most of the time we get what we want free of charge. In Rust there doesn't seem
  to be an easy way to do this. Trait objects have a lot of limitations on what we can force the input to be in
  the functions.
- Drop doesn't seem to work.
- Note that anything printed must implement display and serilize.
- We could possibly gate json behind a flag so if you don't need it you don't have to pull in serde.
- I wasn't able to get drop to properly handle finish_and_clear. FOr the global it would run immediately.
