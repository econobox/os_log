# oslog

A Rust [`log`](log) implementation that logs messages using Apple's new [unified logging
system][uls].

**Note**: Rather annoyingly, the log levels used by the unified logging system are different than
the ones used by `log` (e.g. `log::Level::Error`, `log::Level::Debug`). Even though several of them
share the same names, they are not the closest equivalents to their namesakes in `log`. What follows
is my best effort at matching up the closest equivalents between the two log level schemes. I'm open
to pull requests modifying this mapping (by changing `impl From<Level> for OsLogType`) if anyone
thinks they can make improvements.

| `log` Crate    | `oslog` Crate        |
| -------------- | -------------------- |
| `Level::Error` | `OsLogType::Fault`   |
| `Level::Warn`  | `OsLogType::Error`   |
| `Level::Info`  | `OsLogType::Default` |
| `Level::Debug` | `OsLogType::Info`    |
| `Level::Trace` | `OsLogType::Debug`   |

> My reasoning behind this mapping is based on the descriptions of the unified log levels
> [here][uls].

## Usage

Add both `oslog` and `log` to your `Cargo.toml`:

```toml
[dependencies]
log = "0.4"
oslog = { git = "https://git.sr.ht/~nerosnm/oslog" }
```

and to your crate root:

```rust
#[macro_use]
extern crate log;
extern crate oslog;
```

Then, to initialize logging, call `init()` or `init_custom()` in your crate:

```rust
use oslog::init_custom;

fn main() {
	// Initialize logging
	init_custom("com.example.crate", "category")
		.expect("Could not initialize oslog");
	
	// Send a log message
	trace!("Initialized logging for com.example.crate category");
}
```

## License

`oslog` is licensed under the [MIT
License](https://opensource.org/licenses/MIT).

[log]: https://github.com/rust-lang-nursery/log
[uls]: https://developer.apple.com/documentation/os/logging
