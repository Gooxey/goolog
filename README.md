# goolog

This library provides a function for initiating a fern [Logger](https://docs.rs/fern/0.6.2/fern/struct.Dispatch.html) with some custom formatting and [macros](./src/macros.rs) to simplify printing logs.

## Features

- `timestamp` -> This feature is activated by default. Deactivating this feature will cause the logger to skip printing timestamps, which can be useful when programming for
an embedded system that does not support timestamps.
- `esp` -> This feature is intended to be used in conjunction with embedded development. To ensure developers can compile this library for these systems, this feature can not be used with any other feature. Therefore, you will also need to disable the `default-features`.

## Customization

Currently, there are two ways to customize your goolog logger:

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

By specifying a `path` to the `log_file` parameter, you can tell the logger to save an unformatted version of the log to that file. Meaning, you will have a colored log in your console and an uncolored log in the specified file.

## Example

To print log messages to the console and, if specified, to a file, this library internally uses the [log](https://crates.io/crates/log) and [fern](https://crates.io/crates/fern) crates. But to simplify printing a custom
sender name, one can also use these [library macro`](./src/macros.rs):

```rust
use goolog::*;

fn main() {
    // Initializing the logger
    init_logger(None, None);

    // See the macros module for all possible log types.
    info!("Main", "Initialized the goolog logger.");
}
```

The code above will result in the following output:

```bash
# The timestamp (first two blocks) will only be shown when the `timestamp` feature is active,
# which is the default.

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
