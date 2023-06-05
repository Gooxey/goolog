# goolog

This library provides a function for initiating a fern [Logger](https://docs.rs/fern/0.6.2/fern/struct.Dispatch.html) with some custom formatting and [macros](./src/macros.rs) to simplify printing logs.

## Features

- `timestamp` -> This feature is activated by default. Deactivating this feature will cause the logger to skip printing timestamps, which can be useful when programming for
an embedded system that does not support timestamps.
- `esp` -> This feature is intended to be used in conjunction with embedded development. To ensure developers can compile this library for these systems, this feature can not be used with any other feature. Therefore, you will also need to disable the `default-features`.

## Example

To print log messages to the console and, if specified, to a file, this library internally uses the [log](https://crates.io/crates/log) and [fern](https://crates.io/crates/fern) crates. But to simplify printing a custom
sender name, one can also use these [library macro`](./src/macros.rs):

```rust
use goolog::*;

fn main() {
    // Initializing the logger
    // If one decided to pass a path to this function, the logger would also print the log
    // messages to the file specified.
    init_logger(None);

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
