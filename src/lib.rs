#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]
#![warn(unreachable_pub)]

#[cfg(feature = "esp")]
#[cfg(any(feature = "default", feature = "timestamp"))]
compile_error!("The `esp` feature may not be used with other features!");

#[cfg(not(feature = "wasm"))]
use std::path::PathBuf;

use fern::colors::{
    Color,
    ColoredLevelConfig,
};
// A required export needed by this libraries macros.
pub use log;

pub mod macros;
mod tests;

/// Generate the log line
#[cfg(feature = "timestamp")]
macro_rules! generate_log {
    (
        $final_max_name_length: ident,
        $record: ident,
        $colors: ident,
        $message: ident
    ) => {
        format!(
            "{} | {} | {:5} | {}",
            chrono::Local::now()
                .format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"),
            to_fixed_size($final_max_name_length, $record.target()),
            $colors.color($record.level()),
            $message
        )
    };
}
/// Generate the log line
#[cfg(not(feature = "timestamp"))]
macro_rules! generate_log {
    (
        $final_max_name_length: ident,
        $record: ident,
        $colors: ident,
        $message: ident
    ) => {
        format!(
            "{} | {:5} | {}",
            to_fixed_size($final_max_name_length, $record.target()),
            $colors.color($record.level()),
            $message
        )
    };
}

/// Initiate the custom [`Logger`](fern::Dispatch). \
/// \
/// See the library documentation for more information on the usage and customization possibilities of the goolog logger.
///
/// # Panics
///
/// This function will panic if:
/// - A global logger has already been set to a previous logger.
/// - The given log file could not be opened.
pub fn init_logger(
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

    let mut final_max_name_length = 16;
    if let Some(max_name_length) = max_name_length {
        final_max_name_length = max_name_length;
    }

    #[cfg(not(feature = "wasm"))]
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

    #[allow(unused_mut)] // when we use the wasm feature this does not need to be mut
    let mut logger = fern::Dispatch::new().chain(
        fern::Dispatch::new()
            .format(move |_out, message, record| {
                let log = generate_log!(final_max_name_length, record, colors, message);

                #[cfg(feature = "wasm")]
                web_sys::console::log_1(&log.into());

                #[cfg(not(feature = "wasm"))]
                _out.finish(format_args!("{log}"));
            })
            .level(log::LevelFilter::Info)
            .chain(std::io::stdout()),
    );

    #[cfg(not(feature = "wasm"))]
    if let Some(log_file) = log_file {
        logger = logger.chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    let log = generate_log!(final_max_name_length, record, colors, message);

                    out.finish(format_args!("{log}"))
                })
                .level(log::LevelFilter::Info)
                .chain(fern::log_file(&log_file).unwrap_or_else(|erro| {
                    panic!("Failed to open the log file `{log_file:#?}`. Error: {erro}")
                })),
        );
    }

    logger
        .apply()
        .unwrap_or_else(|erro| panic!("Failed to initiate the goolog logger. Error: {erro}"));
}
