//!# obs-log
//! This is a very simple bridge library between the `log` crate, which is a
//! logging facade, and the OBS plugin logging API.
//! The api is very simple and just maps log calls to `blog` with appropriate
//! logging levels, applying formatting on the Rust side with a `format!` call.
//!
//! Both `debug` and `trace` levels are mapped into `LOG_DEBUG` OBS level since
//! OBS has no trace level.
//!
//! It links to the `obs` dynamic library, so make sure you have that installed.
//!
//! As with any `log` facade, simply put a call to `obs_log::install()` or
//! `obs::log::install_with_level(...)` before using log macros, e.g. in your
//! `obs_module_load` implementation.
//!
//! Install methods can be called multiple times - next invokations will do nothing.

#![deny(missing_docs)]

use std::{
    ffi::CString,
    sync::Once,
};

use log::{Level, Log, Metadata, Record};

use libobs_sys::{
    LOG_ERROR,
    LOG_WARNING,
    LOG_INFO,
    LOG_DEBUG,
    blog,
};

struct ObsLogger {
    level: Level
}

impl Log for ObsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let obs_lvl = match record.level() {
            Level::Error => LOG_ERROR,
            Level::Warn => LOG_WARNING,
            Level::Info => LOG_INFO,
            Level::Debug => LOG_DEBUG,
            Level::Trace => LOG_DEBUG, // obs has no trace level
        };
        let target = if record.target().len() > 0 {
            record.target()
        } else {
            record.module_path().unwrap_or_default()
        };
        let display = format!("[{}] {}", target, record.args());
        unsafe {
            let c_display = CString::new(display).expect("Message contained NUL bytes");
            blog(obs_lvl as i32, c_display.as_ptr());
        }
    }

    fn flush(&self) {}
}

/// Install an OBS logger and additionally configures it
/// to suppress any logging for messages whose logging level
/// is below the one given as a parameter
pub fn install_with_level(level: Level) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        log::set_boxed_logger(Box::new(ObsLogger { level }))
            .map(|()| log::set_max_level(level.to_level_filter()))
            .unwrap();
    });
}

/// Installs an OBS logger with no level restrictions.
///
///
/// # Examples
///
/// ```
/// use log::*;
///
/// #[no_mangle]
/// pub unsafe extern "C" fn obs_module_load() -> bool {
///     obs_log::install();
///     // ... other init code
///     true
/// }
///
/// #[no_mangle]
/// pub unsafe extern "C" fn obs_module_post_load() {
///     debug!("All modules were loaded, idk")
/// }
/// ```
pub fn install() {
    install_with_level(Level::Trace);
}
