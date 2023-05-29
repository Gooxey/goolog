//! This module provides various macros used simplify printing log messages with a sender name set.
//!
//! # Macros
//!
//! - [`info!`](crate::info)
//! - [`warn!`](crate::warn)
//! - [`error!`](crate::error)
//! - [`trace!`](crate::trace)
//! - [`debug!`](crate::debug)

/// This macro logs a message at the info level. \
/// Infos indicate important information that should be logged under normal conditions such as services starting.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None);
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
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None);
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
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None);
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
/// This macro logs a message at the trace level. \
/// Trace messages indicate the steps leading up to errors and warnings, and should provide context to understand them.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None);
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
/// 1. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 2. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format!`] macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// # fn main() {
/// # init_logger(None);
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
