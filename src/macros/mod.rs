//! This module provides various macros used to simplify printing log messages
//! with a target name set.
//!
//! # Macros
//!
//! - [`set_target!`](crate::set_target)
//! - [`info!`](crate::info)
//! - [`warn!`](crate::warn)
//! - [`error!`](crate::error)
//! - [`fatal!`](crate::fatal)
//! - [`trace!`](crate::trace)
//! - [`debug!`](crate::debug)

pub(super) mod on_fatal;

/// Set the target for all log calls in the current module where no target is
/// specified. \
/// \
/// Note: Calling this macro creates the constant `GOOLOG_TARGET`.
///
/// # Example
///
/// ```
/// use goolog::*;
/// set_target!("Main");
///
/// info!("NotMain"; "The target of this message is `NotMain`.");
///
/// info!("The target of this message is `Main`.");
/// ```
#[macro_export]
macro_rules! set_target {
    ($target:literal) => {
        /// A constant used by the goolog crate. \
        /// See the [`set_target macros documentation`](goolog::set_target) for more
        /// details.
        const GOOLOG_TARGET: &str = $target;
    };
}

/// This macro logs a message at the info level. \
/// Infos indicate important information that should be logged under normal
/// conditions, such as services starting.
/// \
/// In case you are tired of always specifying the target, see the
/// [`set_target!`](crate::set_target) macro.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be
///    used in the same way as the format! macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// info!("Main"; "Hello, world!");
/// ```
#[macro_export]
macro_rules! info {
    ($target: expr; $( $argument: tt ) *) => {
        $crate::log::info!(target: &$target, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        $crate::info!(GOOLOG_TARGET; $( $argument ) *)
    }
}
/// This macro logs a message at the warn level. \
/// Warnings indicate a potential problem that may or may not require
/// investigation. They should be used sparingly to avoid becoming meaningless.
/// \
/// In case you are tired of always specifying the target, see the
/// [`set_target!`](crate::set_target) macro.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be
///    used in the same way as the format! macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// let error = "The given file is invalid!";
/// warn!("Main"; "Accept the EULA to use this MCServer. Error: {}", error);
/// ```
#[macro_export]
macro_rules! warn {
    ($target: expr; $( $argument: tt ) *) => {
        $crate::log::warn!(target: &$target, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        $crate::warn!(GOOLOG_TARGET; $( $argument ) *)
    }
}
/// This macro logs a message at the error level. \
/// Errors indicate a problem that needs to be investigated, but doesn't require
/// immediate attention.
/// \
/// In case you are tired of always specifying the target, see the
/// [`set_target!`](crate::set_target) macro.
///
/// # Limitations
///
/// When used with the goolog logger, any message starting with `$goolog:fatal=`
/// will be converted to a log line looking like it was printed by the
/// [`fatal!`](crate::fatal) macro.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be
///    used in the same way as the format! macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// let erro = "The given file is invalid!";
/// error!("Main"; "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
/// ```
#[macro_export]
macro_rules! error {
    ($target: expr; $( $argument: tt ) *) => {
        $crate::log::error!(target: &$target, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        $crate::error!(GOOLOG_TARGET; $( $argument ) *)
    }
}
/// This macro logs a message at the error level and executes the callback
/// defined by the [`set_on_fatal`](crate::set_on_fatal) function. \
/// When using the `std` feature, this callback will, by default, call
/// `std::process::exit(1)`. \
/// Fatal errors indicate a problem that is not recoverable. \
/// \
/// In case you are tired of always specifying the target, see the
/// [`set_target!`](crate::set_target) macro.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be
///    used in the same way as the format! macro.
///
/// # Example
///
/// ```should_panic
/// use goolog::*;
///
/// let erro = "The given file is invalid!";
/// fatal!("Main"; "An error occurred while waiting on the Minecraft server to finish. Error: {}", erro);
/// # panic!()
/// ```
#[macro_export]
macro_rules! fatal {
    ($target: expr; $( $argument: tt ) *) => {
        $crate::log::error!(
            target: &$target,
            $( $argument ) *
        );
        if let Some(on_fatal) = *$crate::ON_FATAL.lock() {
            on_fatal()
        }
    };
    ($( $argument: tt ) *) => {
        $crate::fatal!(GOOLOG_TARGET; $( $argument ) *)
    }
}
/// This macro logs a message at the trace level. \
/// Trace messages indicate the steps leading up to errors and warnings, and
/// should provide context for understanding them.
/// \
/// In case you are tired of always specifying the target, see the
/// [`set_target!`](crate::set_target) macro.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be
///    used in the same way as the format! macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// trace!("Main"; "Initiated goolog logger without a log file set.");
/// ```
#[macro_export]
macro_rules! trace {
    ($target: expr; $( $argument: tt ) *) => {
        $crate::log::trace!(target: &$target, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        $crate::trace!(GOOLOG_TARGET; $( $argument ) *)
    }
}
/// This macro logs a message at the debug level. \
/// Debug messages indicate debugging information that is compiled out of
/// release builds and are discouraged due to their tendency to create log
/// noise. \ \
/// Note: Messages passed to this macro will only be printed during debug mode.
/// \
/// In case you are tired of always specifying the target, see the
/// [`set_target!`](crate::set_target) macro.
///
/// # Parameters
///
/// 1. This is the `name` under which this log should be sent.
/// 2. The following arguments represent the `message` to be sent. It can be
///    used in the same way as the format! macro.
///
/// # Example
///
/// ```
/// use goolog::*;
///
/// debug!("Main"; "Initiated logger.");
/// ```
#[macro_export]
macro_rules! debug {
    ($target: expr; $( $argument: tt ) *) => {
        #[cfg(debug_assertions)]
        $crate::log::debug!(target: &$target, $( $argument ) *);
    };
    ($( $argument: tt ) *) => {
        $crate::debug!(GOOLOG_TARGET; $( $argument ) *)
    }
}
