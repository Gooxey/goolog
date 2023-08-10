#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]
#![warn(unreachable_pub)]

#[cfg(not(feature = "wasm"))]
use std::path::PathBuf;
use std::sync::OnceLock;

use fern::colors::{
    Color,
    ColoredLevelConfig,
};
// A required export needed by this libraries macros.
pub use log;
use log::{
    Level,
    LevelFilter,
};

pub mod macros;
mod tests;

/// The caller name for fatal logs send by this logger.
const GOOLOG_CALLER: &str = "Logger";

/// # DO NOT TOUCH THIS STATIC
#[allow(non_upper_case_globals)]
pub static INTERNAL__LOGGER_ACTIVE: OnceLock<()> = OnceLock::new();

/// Generate the log line
#[cfg(feature = "timestamp")]
macro_rules! generate_log {
    ($max_name_length:ident, $record:ident, $colors:ident, $message:ident) => {{
        let mut message = $message.to_string();
        let mut log_level = $colors.color($record.level()).to_string();
        #[cfg(not(feature = "std_fatal"))]
        if let Level::Error = $record.level() {
            if let Some(val) = message.strip_prefix("$goolog:fatal=") {
                log_level = log_level.replace("ERROR", "FATAL");
                message = val.into();
            }
        }

        format!(
            "{} | {} | {:14.14} | {}",
            chrono::Local::now()
                .format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"),
            to_fixed_size($max_name_length, $record.target()),
            log_level,
            message
        )
    }};
}
/// Generate the log line
#[cfg(not(feature = "timestamp"))]
macro_rules! generate_log {
    ($max_name_length:ident, $record:ident, $colors:ident, $message:ident) => {{
        let mut message = $message.to_string();
        let mut log_level = $colors.color($record.level()).to_string();
        #[cfg(not(feature = "std_fatal"))]
        if let Level::Error = $record.level() {
            if let Some(val) = message.strip_prefix("$goolog:fatal=") {
                log_level = log_level.replace("ERROR", "FATAL");
                message = val.into();
            }
        }

        format!(
            "{} | {:14.14} | {}",
            to_fixed_size($max_name_length, $record.target()),
            log_level,
            message
        )
    }};
}

/// Initiate the custom [`Logger`](fern::Dispatch). \
/// \
/// See the library documentation for more information on the usage and
/// customization possibilities of the goolog logger.
///
/// # Panics
///
/// This function will panic if:
/// - A global logger has already been set to a previous logger.
/// - The given log file could not be opened.
pub fn init_logger(
    log_level: Option<LevelFilter>,
    max_name_length: Option<u32>,
    #[cfg(not(feature = "wasm"))] log_file: Option<PathBuf>,
) {
    /// Edit the name to be `max_name_length` long.
    fn to_fixed_size(max_name_length: u32, name: &str) -> String {
        if max_name_length == 0 {
            return name.to_string();
        }

        let mut new_name = vec![];
        let mut name_iter = name.chars();
        for _ in 0..max_name_length {
            if let Some(name_char) = name_iter.next() {
                new_name.push(name_char.to_string());
            } else {
                new_name.push(' '.to_string());
            }
        }

        new_name.concat()
    }

    let max_name_length = max_name_length.unwrap_or(16);
    let log_level = log_level.unwrap_or(LevelFilter::Info);

    #[cfg(not(feature = "wasm"))]
    if let Some(mut logs_dir) = log_file.clone() {
        // we need to pop here because logs_dir is the path to the log file and not the
        // path to the log directory
        logs_dir.pop();
        std::fs::create_dir_all(&logs_dir).unwrap_or_else(|error| {
            fatal!(
                "An error occurred while creating the directory '{}'. Error: {error}",
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

    #[allow(unused_mut)] // when we use the wasm feature this does not need to be mut
    let mut logger = fern::Dispatch::new().chain(
        fern::Dispatch::new()
            .format(move |_out, message, record| {
                let log = generate_log!(max_name_length, record, colors, message);

                #[cfg(feature = "wasm")]
                web_sys::console::log_1(&log.into());

                #[cfg(not(feature = "wasm"))]
                _out.finish(format_args!("{log}"));
            })
            .level(log_level)
            .chain(std::io::stdout()),
    );

    #[cfg(not(feature = "wasm"))]
    if let Some(log_file) = log_file {
        logger = logger.chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    let log = generate_log!(max_name_length, record, colors, message);

                    out.finish(format_args!("{log}"))
                })
                .level(log::LevelFilter::Info)
                .chain(fern::log_file(&log_file).unwrap_or_else(|error| {
                    fatal!("Failed to open the log file `{log_file:#?}`. Error: {error}")
                })),
        );
    }

    logger
        .apply()
        .unwrap_or_else(|error| fatal!("Failed to initiate the goolog logger. Error: {error}"));

    if INTERNAL__LOGGER_ACTIVE.set(()).is_err() {
        fatal!(
            "The `INTERNAL__LOGGER_ACTIVE` static should only be used by the goolog logger or its macros."
        )
    }
}
