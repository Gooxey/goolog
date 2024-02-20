# goolog

This library provides the no_std-compatible goolog logger and some macros to simplify printing logs.

## Usage in embedded / no_std environments

Because this logger is implemented in a no_std environment, it will work with all embedded systems by default.

```rust
#![no_std]

use goolog::*;

fn main() {
    // First, we initialize the logger.
    init_logger(
        None,
        None,
        // println callback
        &|_args| {
            // something printing our args to the console
        },
    );

    // Now we define a callback, which gets called whenever anyone uses the fatal! macro.
    set_on_fatal(&|| {
        // some function to restart your embedded device
    });

    // Now you can start logging using either these library macros or the log crates ones.
}
```


## Features

### `std`

This feature will: \
    1. Implement the `println` callback using the standard library \
    2. Implement the on_fatal callback \
    3. Enable timestamps using [chrono](https://crates.io/crates/chrono/0.4.34)

## Example

> All examples from here on will assume you have the `std` feature active.

```rust
use goolog::*;

fn main() {
    // Initializing the logger
    init_logger(None, None);

    // See the macros module for all possible log types.
    info!("Main"; "Initialized the goolog logger.");
}
```

The code above will result in the following output:

```bash
# The timestamp (first two blocks) will only be shown when
# the `std` feature is active.

29.05.2023 | 14:34:33 | Main             | INFO  | Initialized the goolog logger.
```

But in reality, the log message will be formatted with a color like this:

```text
GREY | GREY | WHITE | * | WHITE

*:
    DEBUG -> Blue
    ERROR -> Red
    INFO  -> Green
    TRACE -> White
    WARN  -> Yellow
```

## Quality of life

When printing log lines to the console using these library macros, there are two ways to specify the target:

### Specification by macro

This is always possible. Even in combination with the second method described below.

```rust
use goolog::*;

fn main() {
    init_logger(None, None);

    info!("Main"; "Initialized the goolog logger.");
}
```

### Specification by constant

For this method, you will only need to specify the target once:

```rust
use goolog::*;

set_target!("Main");

fn main() {
    init_logger(None, None);

    info!("Initialized the goolog logger.");

    // You can still specify a different target
    // just for that log line.
    info!("Foo"; "Initialized the goolog logger.");
}
```

## Customization

Currently, there are two ways to customize your goolog logger:

### Changing the logging level

By default, the logger will log at the `info` level. To change this, just provide a new log level.

```rust
use goolog::*;
use goolog::log::Level;

fn main() {
    // Initializing the logger with the log level set to trace
    init_logger(Some(Level::Trace), None);

    // See the macros module for all possible log types.
    // This will only be logged if the log level is set to
    // trace.
    trace!("Main"; "Initialized the goolog logger.");
}
```

### Changing the length of caller names

The default target length is 16 characters. Any given name longer than that will simply be truncated. However, there are two ways to customize this behavior:

#### 1. Change the limit

```rust
use goolog::*;

fn main() {
    // Initialize the logger with a new target length of 32
    // characters.
    init_logger(None, Some(32));

    // ...

    // You can also change the target length at runtime.
    set_target_length(8)
}
```

#### 2. Remove the limit

To do this set the `target_length` to `0` using either of the two ways shown above.

But before you do this, you might consider the drawbacks:
If no limit is given, the logger has no way of knowing how much space to leave for the name. Therefore, each log line will 'bend' around the name, which will look something like this:

```text
29.05.2023 | 14:34:33 | Main | INFO  | Starting some very important things...
29.05.2023 | 14:34:33 | MySuperAwesomeClient | INFO  | Starting...
```
