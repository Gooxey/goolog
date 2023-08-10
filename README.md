# goolog

This library provides a function for initiating a fern [Logger](https://docs.rs/fern/0.6.2/fern/struct.Dispatch.html) with some custom formatting and [macros](./src/macros.rs) to simplify printing logs.

## Usage in embedded development

To use this crate on embedded systems, you must disable the `timestamp` feature:

Run this command:

```text
$ cargo add goolog --no-default-features
```

Or add this to your `Cargo.toml`

```toml
goolog = { version = "x.y.z", default-features = false }
```

## Features

| Feature | Description |
|-|-|
| `timestamp` | This feature is activated by default. Deactivating this feature will cause the logger to skip printing timestamps, which can be useful when programming for an embedded system that does not support timestamps. |
| `wasm` | This logger will only work on `wasm targets` if this feature is enabled. |

## Quality of life

> For more details, refer to the documentation of any macro.

When printing log lines to the console using these library macros, there are two ways to specify the caller name:

### Specification by macro

This is always possible. Even in combination with the second method described below.

```rust
use goolog::*;

fn main() {
    // Initializing the logger
    init_logger(None, None, None);

    info!("Main"; "Initialized the goolog logger.");
}
```

### Specification by constant

For this method, you will only need to specify the caller name once:

```rust
use goolog::*;

set_caller!("Main");

fn main() {
    // Initializing the logger
    init_logger(None, None, None);

    info!("Initialized the goolog logger.");

    // You can still specify a different caller name
    // just for that log line.
    info!("OtherCaller"; "Initialized the goolog logger.");
}
```

## Customization

> When the `wasm feature` is enabled, you have only `two` method of customization.

Currently, there are three ways to customize your goolog logger:

### Changing the logging level

> The log level for logging to a file will always remain at `info`

By default the logger will log at the `info` level. To change this, just provide a new log level.

```rust
use goolog::*;
use goolog::log::LevelFilter;

fn main() {
    // Initializing the logger with the log level set to trace
    // When you enable the wasm feature, you have only two
    // parameters
    init_logger(Some(LevelFilter::Trace), None, None);

    // See the macros module for all possible log types.
    // this will only be logged if the log level is set to
    // trace
    trace!("Main"; "Initialized the goolog logger.");
}
```

### Changing the length of caller names

The default caller name length is 16 characters. Any given name longer than that will simply be truncated. However, there are two ways to customize this behavior:

#### 1. Raise the limit

This is as easy forward as you can imagine: Just specify a `new limit` using the `max_name_length` parameter.

#### 2. Remove the limit

To do this set the `max_name_length` parameter to `0`.

But before you do this, you might consider the drawbacks:
If no limit is given, the logger has no way of knowing how much space to leave for the name. Therefore, each log line will 'bend' around the name, which will look something like the this:

```text
29.05.2023 | 14:34:33 | Main | INFO  | Starting some very important things...
29.05.2023 | 14:34:33 | MySuperAwesomeMCManageClient | INFO  | Starting...
```

### Setting a log file

> This customization option is `not` available for the `wasm feature`.

By specifying a `path` to the `log_file` parameter, you can tell the logger to save an unformatted version of the log to that file. Meaning, you will have a colored log in your console and an uncolored log in the specified file.

## Example

To print log messages to the console and, if specified, to a file, this library internally uses the [log](https://crates.io/crates/log) and [fern](https://crates.io/crates/fern) crates. But to simplify printing a custom
sender name, one can also use these [library macro`](./src/macros.rs):

```rust
use goolog::*;

fn main() {
    // Initializing the logger
    // When you enable the wasm feature, you have only two
    // parameters
    init_logger(None, None, None);

    // See the macros module for all possible log types.
    info!("Main"; "Initialized the goolog logger.");
}
```

The code above will result in the following output:

```bash
# The timestamp (first two blocks) will only be shown when the
# `timestamp` feature is active, which is the default.

29.05.2023 | 14:34:33 | Main             | INFO  | Initialized the goolog logger.
```

But in reality, the log message will be formatted with color like this:

```text
GREY | GREY | WHITE | * | WHITE

*:
    DEBUG -> Blue
    ERROR -> Red
    INFO  -> Green
    TRACE -> White
    WARN  -> Yellow
```
