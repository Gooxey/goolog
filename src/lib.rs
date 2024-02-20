#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(unreachable_pub)]

pub extern crate log;

mod logger;
mod macros;

pub use logger::*;
pub use macros::on_fatal::*;
