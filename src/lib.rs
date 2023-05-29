//! This library provides a function for initiating a fern [`Logger`](fern::Dispatch) with some custom formatting and [`macros`] to simplify printing logs.
//!
//! # Features
//!
//! - `timestamp` -> This feature is activated by default. Deactivating this feature will cause the logger to skip printing timestamps, which can be useful when programming for
//! an embedded system that does not support timestamps.
//!
//! # Example
//!
//! To print log messages to the console and, if specified, to a file, this library internally uses the [`log`] and [`fern`] crates. But to simplify printing a custom
//! sender name, one can also use these [`library macros`](macros):
//! ```
//! use goolog::*;
//!
//! # fn main() {
//!     // Initializing the logger
//!     // If one decided to pass a path to this function, the logger would also print the log messages to the file specified.
//!     init_logger(None);
//!
//!     // See the macros module for all possible log types.
//!     info!("Main", "Initialized the goolog logger.");
//! # }
//! ```
//! The code above will result in the following output:
//! ```bash
//! 29.05.2023 | 14:34:33 | Main             | INFO  | Initialized the goolog logger.
//! ```
//!
//! But in reality, the log message will be formatted with color like this:
//! ```text
//! GREY | GREY | WHITE | * | WHITE
//!
//! *:
//!     DEBUG -> Blue
//!     ERROR -> Red
//!     INFO  -> Green
//!     TRACE -> White
//!     WARN  -> Yellow
//! ```

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]

use std::path::PathBuf;

use fern::colors::{
    Color,
    ColoredLevelConfig,
};

// A required export needed by this libraries macros.
pub use log;

pub mod macros;
mod tests;

/// Initiate the custom [`Logger`](fern::Dispatch). \
/// If the path to the log file is specified, the logger will also save the logs to that file. (It is recommended to call the log file `*.log`.) \
/// Note: The formatting of colors will not be saved.
/// \
/// See the libraries documentation for more information on the custom logger.
///
/// # Panics
///
/// This function will panic if:
/// - A global logger has already been set to a previous logger.
/// - The given log file could not be opened.
pub fn init_logger(log_file: Option<PathBuf>) {
    if let Some(mut logs_dir) = log_file.clone() {
        // we need to pop here because logs_dir is the path to the log file and not the path to the log directory
        logs_dir.pop();
        std::fs::create_dir_all(&logs_dir).unwrap_or_else(|erro| {
            panic!(
                "An error occurred while creating the directory '{}'. Error: {erro}",
                logs_dir.display()
            )
        });
    }

    let colors = ColoredLevelConfig::new()
        .debug(Color::Blue)
        .error(Color::Red)
        .info(Color::Green)
        .trace(Color::White)
        .warn(Color::Yellow);

    let logger = fern::Dispatch::new().chain(
        fern::Dispatch::new()
            .format(move |out, message, record| {
                if cfg!(feature = "timestamp") {
                    out.finish(format_args!(
                        "{} | {:16.16} | {:5} | {}",
                        chrono::Local::now()
                            .format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"),
                        record.target(),
                        colors.color(record.level()),
                        message
                    ))
                } else {
                    out.finish(format_args!(
                        "{:16.16} | {:5} | {}",
                        record.target(),
                        colors.color(record.level()),
                        message
                    ))
                }
            })
            .level(log::LevelFilter::Info)
            .chain(std::io::stdout()),
    );

    if let Some(log_file) = log_file {
        logger
            .chain(
                fern::Dispatch::new()
                    .format(move |out, message, record| {
                        if cfg!(feature = "timestamp") {
                            out.finish(format_args!(
                                "{} | {:16.16} | {:5} | {}",
                                chrono::Local::now().format("%d.%m.%Y | %H:%M:%S"),
                                record.target(),
                                record.level(),
                                message
                            ))
                        } else {
                            out.finish(format_args!(
                                "{:16.16} | {:5} | {}",
                                record.target(),
                                record.level(),
                                message
                            ))
                        }
                    })
                    .level(log::LevelFilter::Info)
                    .chain(fern::log_file(&log_file).unwrap_or_else(|erro| {
                        panic!("Failed to open the log file `{log_file:#?}`. Error: {erro}")
                    })),
            )
            .apply()
            .unwrap_or_else(|erro| panic!("Failed to initiate the goolog logger. Error: {erro}"));
    } else {
        logger
            .apply()
            .unwrap_or_else(|erro| panic!("Failed to initiate the goolog logger. Error: {erro}"));
    }
}
