//! This module provides various macros used simplify printing log messages with a sender name set.
//!
//! # Macros
//!
//! - [`info!`](crate::info)
//! - [`warn!`](crate::warn)
//! - [`error!`](crate::error)
//! - [`fatal!`](crate::fatal)
//! - [`trace!`](crate::trace)
//! - [`debug!`](crate::debug)

/// This macro logs a message at the info level. \
/// Infos indicate important information that should be logged under normal conditions such as services starting.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let secs = Duration::new(1, 0);
/// info!("Main", "Started in {:?} secs", secs);
///
/// // This is what this macro will expand to:
/// goolog::log::info!(target: &"Main", "Started in {:?} secs", secs);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($sender: expr, $( $argument: tt ) *) => {
        $crate::log::info!(target: &$sender, $( $argument ) *);
    }
}
/// This macro logs a message at the warn level. \
/// Warnings indicate a potential problem that may or may not require investigation. They should be used sparingly to avoid becoming meaningless.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let erro = "The given file is invalid!";
/// warn!("Main", "Accept the EULA to use this MCServer. Error: {}", erro);
///
/// // This is what this macro will expand to:
/// goolog::log::warn!(target: &"Main", "Accept the EULA to use this MCServer. Error: {}", erro);
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($sender: expr, $( $argument: tt ) *) => {
        $crate::log::warn!(target: &$sender, $( $argument ) *);
    }
}
/// This macro logs a message at the error level. \
/// Errors indicate a problem that needs to be investigated, but doesn't require immediate attention.
///
/// # Limitations
///
/// When used with the goolog logger, any message starting with `$goolog:fatal=` will be converted to a log line looking like it was printed by the [`fatal!`] macro.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let erro = "The given file is invalid!";
/// error!("Main", "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
///
/// // This is what this macro will expand to:
/// goolog::log::error!(target: &"Main", "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($sender: expr, $( $argument: tt ) *) => {
        $crate::log::error!(target: &$sender, $( $argument ) *);
    }
}
/// This macro logs a message at the error level and exits the application with the error code 1. \
/// Fatal errors indicate a problem that is not recoverable.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let erro = "The given file is invalid!";
/// fatal!("Main", "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
///
/// // This is what this macro will expand to:
///     // don't even think about touching this static
/// if goolog::INTERNAL__LOGGER_ACTIVE.get().is_some() {
///     goolog::log::error!(
///         target: &"Main",
///         "$goolog:fatal={}", format!("An error occurred while waiting on the Minecraft server to finish. Error: {}", erro)
///     );
/// } else {
///     goolog::log::error!(
///         target: &"Main",
///         "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro
///     );
/// }
/// std::process::exit(1)
/// # }
/// ```
#[macro_export]
macro_rules! fatal {
    ($sender: expr, $( $argument: tt ) *) => {
        {
            // we assume the user followed our warning and that the goolog logger is active
            if $crate::INTERNAL__LOGGER_ACTIVE.get().is_some() {
                // tell our logger to send an fatal message instead of an error
                $crate::log::error!(
                    target: &$sender,
                    "$goolog:fatal={}", format!($( $argument ) *)
                );
            } else {
                $crate::log::error!(
                    target: &$sender,
                    $( $argument ) *
                );
            }
            std::process::exit(1)
        }
    }
}
/// This macro logs a message at the trace level. \
/// Trace messages indicate the steps leading up to errors and warnings, and should provide context to understand them.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None, None, None);
///
/// trace!("Main", "Initiated goolog logger without a log file set.");
///
/// // This is what this macro will expand to:
/// goolog::log::trace!(target: &"Main", "Initiated goolog logger without a log file set.");
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    ($sender: expr, $( $argument: tt ) *) => {
        $crate::log::trace!(target: &$sender, $( $argument ) *);
    }
}
/// This macro logs a message at the debug level. \
/// Debug messages indicate debugging information that is compiled out of Release builds and is discouraged due to its tendency to create log noise. \
/// \
/// Note: Messages passed to this macro will only be printed during debug mode.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None, None, None);
///
/// debug!("Main", "Initiated logger.");
///
/// // This is what this macro will expand to:
/// #[cfg(debug_assertions)]
/// goolog::log::debug!(target: &"Main", "Initiated logger.");
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    ($sender: expr, $( $argument: tt ) *) => {
        #[cfg(debug_assertions)]
        $crate::log::debug!(target: &$sender, $( $argument ) *);
    }
}
