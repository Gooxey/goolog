//! This module provides various macros used simplify printing log messages with a caller name set.
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
///
/// In case you are tired of always specifying the name of the caller, you can also just set a constant:
///
/// ```
/// use goolog::*;
///
/// const GOOLOG_CALLER: &str = "Main";
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let secs = Duration::new(1, 0);
/// info!("Started in {:?} secs", secs);
///
/// // This is what this macro will expand to:
/// goolog::info!(GOOLOG_CALLER, "Started in {:?} secs", secs);;
/// // this will then further expand to:
/// goolog::log::info!(target: &GOOLOG_CALLER, "Started in {:?} secs", secs);
///
/// // but you can still specify a caller name which will result in the standard behavior
/// info!("OtherCaller", "Started in {:?} secs", secs);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($caller: expr, $( $argument: tt ) *) => {
        $crate::log::info!(target: &$caller, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        info!(GOOLOG_CALLER, $( $argument ) *)
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
///
/// In case you are tired of always specifying the name of the caller, you can also just set a constant:
///
/// ```
/// use goolog::*;
///
/// const GOOLOG_CALLER: &str = "Main";
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let erro = "The given file is invalid!";
/// warn!("Accept the EULA to use this MCServer. Error: {}", erro);
///
/// // This is what this macro will expand to:
/// goolog::warn!(GOOLOG_CALLER, "Accept the EULA to use this MCServer. Error: {}", erro);
/// // this will then further expand to:
/// goolog::log::warn!(target: &GOOLOG_CALLER, "Accept the EULA to use this MCServer. Error: {}", erro);
///
/// // but you can still specify a caller name which will result in the standard behavior
/// warn!("OtherCaller", "Accept the EULA to use this MCServer. Error: {}", erro);
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($caller: expr, $( $argument: tt ) *) => {
        $crate::log::warn!(target: &$caller, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        warn!(GOOLOG_CALLER, $( $argument ) *)
    }
}
/// This macro logs a message at the error level. \
/// Errors indicate a problem that needs to be investigated, but doesn't require immediate attention.
///
/// # Limitations
///
/// When used with the goolog logger, any message starting with `$goolog:fatal=` will be converted to a log line looking like it was printed by the [`fatal!`](crate::fatal)
/// macro.
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
///
/// In case you are tired of always specifying the name of the caller, you can also just set a constant:
///
/// ```
/// use goolog::*;
///
/// const GOOLOG_CALLER: &str = "Main";
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let erro = "The given file is invalid!";
/// error!("An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
///
/// // This is what this macro will expand to:
/// goolog::error!(GOOLOG_CALLER, "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
/// // this will then further expand to:
/// goolog::log::error!(target: &GOOLOG_CALLER, "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
///
/// // but you can still specify a caller name which will result in the standard behavior
/// error!("OtherCaller", "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($caller: expr, $( $argument: tt ) *) => {
        $crate::log::error!(target: &$caller, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        error!(GOOLOG_CALLER, $( $argument ) *)
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
///
/// In case you are tired of always specifying the name of the caller, you can also just set a constant:
///
/// ```
/// use goolog::*;
///
/// const GOOLOG_CALLER: &str = "Main";
/// # fn main() {
/// # init_logger(None, None, None);
///
/// let erro = "The given file is invalid!";
/// fatal!("An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
///
/// // This is what this macro will expand to:
/// goolog::fatal!(GOOLOG_CALLER, "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
/// // this will then further expand to:
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
///
/// // but you can still specify a caller name which will result in the standard behavior
/// fatal!("OtherCaller", "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
/// # }
/// ```
#[macro_export]
macro_rules! fatal {
    ($caller: expr, $( $argument: tt ) *) => {
        {
            // we assume the user followed our warning and that the goolog logger is active
            if $crate::INTERNAL__LOGGER_ACTIVE.get().is_some() {
                // tell our logger to send an fatal message instead of an error
                $crate::log::error!(
                    target: &$caller,
                    "$goolog:fatal={}", format!($( $argument ) *)
                );
            } else {
                $crate::log::error!(
                    target: &$caller,
                    $( $argument ) *
                );
            }
            std::process::exit(1)
        }
    };
    ($( $argument: tt ) *) => {
        fatal!(GOOLOG_CALLER, $( $argument ) *)
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
/// # fn main() {
/// # init_logger(None, None, None);
///
/// trace!("Main", "Initiated goolog logger without a log file set.");
///
/// // This is what this macro will expand to:
/// goolog::log::trace!(target: &"Main", "Initiated goolog logger without a log file set.");
/// # }
/// ```
///
/// In case you are tired of always specifying the name of the caller, you can also just set a constant:
///
/// ```
/// use goolog::*;
///
/// const GOOLOG_CALLER: &str = "Main";
/// # fn main() {
/// # init_logger(None, None, None);
///
/// trace!("Initiated goolog logger without a log file set.");
///
/// // This is what the code above will expand to:
/// goolog::trace!(GOOLOG_CALLER, "Initiated goolog logger without a log file set.");
/// // this will then further expand to:
/// goolog::log::trace!(target: &GOOLOG_CALLER, "Initiated goolog logger without a log file set.");
///
/// // but you can still specify a caller name which will result in the standard behavior
/// trace!("OtherCaller", "A different trace message.");
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    ($caller: expr, $( $argument: tt ) *) => {
        $crate::log::trace!(target: &$caller, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        trace!(GOOLOG_CALLER, $( $argument ) *)
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
///
/// In case you are tired of always specifying the name of the caller, you can also just set a constant:
///
/// ```
/// use goolog::*;
///
/// const GOOLOG_CALLER: &str = "Main";
/// # fn main() {
/// # init_logger(None, None, None);
///
/// debug!("Initiated logger.");
///
/// // This is what this macro will expand to:
/// goolog::debug!(GOOLOG_CALLER, "Initiated logger.");
/// // this will then further expand to:
/// goolog::log::debug!(target: &GOOLOG_CALLER, "Initiated logger.");
///
/// // but you can still specify a caller name which will result in the standard behavior
/// debug!("OtherCaller", "Initiated logger.");
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    ($caller: expr, $( $argument: tt ) *) => {
        #[cfg(debug_assertions)]
        $crate::log::debug!(target: &$caller, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        debug!(GOOLOG_CALLER, $( $argument ) *)
    }
}
