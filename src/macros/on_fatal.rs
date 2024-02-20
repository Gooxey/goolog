use spin::Mutex;

type OnFatal = &'static (dyn Fn() + Sync);

#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub static ON_FATAL: Mutex<Option<OnFatal>> = Mutex::new(None);
#[doc(hidden)]
#[cfg(feature = "std")]
pub static ON_FATAL: Mutex<Option<OnFatal>> = Mutex::new(Some(&|| std::process::exit(1)));

/// Set the callback to be called whenever the [`fatal`](crate::fatal) macro
/// gets called.
pub fn set_on_fatal(on_fatal: OnFatal) {
    *ON_FATAL.lock() = on_fatal.into()
}
