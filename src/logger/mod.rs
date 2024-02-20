use core::fmt::Arguments;

use log::{
    set_logger_racy,
    set_max_level_racy,
    Level,
    Log,
};
use spin::Mutex;

use self::band_aid_fix::DisplayChars;

mod band_aid_fix;

static SETTING_LOGGER: Mutex<()> = Mutex::new(());
static mut LOGGER: Option<Logger> = None;

static TARGET_LENGTH: Mutex<u8> = Mutex::new(16);

/// A [logger](log::Log) has already been set.
#[derive(Debug)]
pub struct LoggerAlreadySet;

type Println = &'static (dyn Fn(Arguments<'_>) + Sync + Send);

/// Set the space for the target to a fixed size.
pub fn set_target_length(target_length: u8) {
    *TARGET_LENGTH.lock() = target_length
}

/// Initialize the goolog logger. \
/// \
/// See the library documentation for more information on the usage and
/// customization possibilities of the goolog logger.
pub fn init_logger(
    log_level: Option<Level>,
    target_length: Option<u8>,
    #[cfg(not(feature = "std"))] println: Println,
) -> Result<(), LoggerAlreadySet> {
    // ensure only one thread is setting the logger at once
    let _setting_logger = SETTING_LOGGER.lock();

    let log_level = log_level.unwrap_or(Level::Info);
    let logger;

    #[cfg(not(feature = "std"))]
    {
        logger = Logger { log_level, println }
    }
    #[cfg(feature = "std")]
    {
        logger = Logger {
            log_level,
            println: &|args| println!("{args}"),
        }
    }

    unsafe {
        if LOGGER.is_some() {
            return Err(LoggerAlreadySet);
        }

        LOGGER = logger.into();

        if set_logger_racy(
            LOGGER
                .as_ref()
                .expect("The `LOGGER` static should contain a value at this point."),
        )
        .is_err()
        {
            LOGGER = None;
            return Err(LoggerAlreadySet);
        }

        set_max_level_racy(log_level.to_level_filter())
    }

    if let Some(target_length) = target_length {
        set_target_length(target_length);
    }

    Ok(())
}

struct Logger {
    log_level: Level,
    println: Println,
}
impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            // timestamp in std
            #[cfg(feature = "std")]
            let time_stamp = chrono::Local::now()
                .format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m | ");
            #[cfg(not(feature = "std"))]
            let time_stamp = "";

            // target to fixed size
            let target;
            let max_name_length = TARGET_LENGTH.lock();
            if *max_name_length == 0 {
                target = record.target().into()
            } else {
                let name = record.target();
                let name_len = name.len();
                if name_len >= *max_name_length as usize {
                    target = name.chars().take(*max_name_length as usize).into();
                } else {
                    target = DisplayChars::new(
                        name.chars().take(usize::MAX).into(),
                        *max_name_length as usize - name_len,
                    );
                }
            }
            drop(max_name_length);

            (self.println)(format_args!(
                "{}{} | {:14.14} | {}",
                time_stamp,
                target,
                record.metadata().level(),
                record.args()
            ))
        }
    }

    fn flush(&self) {}
}
