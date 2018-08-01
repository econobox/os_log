//
//  lib.rs
//  oslog
//
//  Created by Søren Mortensen on 29/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

//! `oslog` is a Rust [`log`][log] implementation that logs messages using Apple's new [unified
//! logging system][uls].
//!
//! ## Log Levels
//!
//! Rather annoyingly, the log levels used by the unified logging system are different than the ones
//! used by `log` (i.e. [`log::Level`][loglevel]). Even though several of them share the same names,
//! they are not the closest equivalents to their namesakes in `log`. I've done my best to match the
//! `log` levels to the unified logging levels - it's imperfect, but seems to match Apple's
//! intentions.
//!
//! ## Examples
//!
//! Initialize logging with default settings:
//!
//! ```rust
//! # extern crate oslog;
//! oslog::init()
//!     .expect("Could not initialize logging");
//! ```
//!
//! Initialize logging with subsystem `"com.example.crate"` and category `"category"`:
//!
//! ```rust
//! # extern crate oslog;
//! oslog::init_custom("com.example.crate", "category")
//!     .expect("Could not initialize logging");
//! ```
//!
//! [log]: ../log/index.html
//! [uls]: https://developer.apple.com/documentation/os/logging
//! [loglevel]: ../log/enum.Level.html

extern crate libc;
extern crate log;
extern crate oslog_sys;

use log::{Level, Log, Metadata, Record, SetLoggerError};

use oslog_sys::os_log_t;
use oslog_sys::{
    _os_log, _os_log_debug, _os_log_error, _os_log_fault, _os_log_info, os_log_create,
};
use oslog_sys::{
    OS_LOG_TYPE_DEBUG, OS_LOG_TYPE_DEFAULT, OS_LOG_TYPE_ERROR, OS_LOG_TYPE_FAULT, OS_LOG_TYPE_INFO,
};

use std::ffi::CString;

#[repr(u32)]
enum OsLogType {
    Default = OS_LOG_TYPE_DEFAULT,
    Info = OS_LOG_TYPE_INFO,
    Debug = OS_LOG_TYPE_DEBUG,
    Error = OS_LOG_TYPE_ERROR,
    Fault = OS_LOG_TYPE_FAULT,
}

impl From<Level> for OsLogType {
    fn from(other: Level) -> OsLogType {
        match other {
            Level::Error => OsLogType::Fault,
            Level::Warn => OsLogType::Error,
            Level::Info => OsLogType::Default,
            Level::Debug => OsLogType::Info,
            Level::Trace => OsLogType::Debug,
        }
    }
}

struct OsLog {
    log_object: os_log_t,
    level: Level,
}

unsafe impl Send for OsLog {}
unsafe impl Sync for OsLog {}

impl Log for OsLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let string = format!("{}", record.args());
            let c_string = CString::new(string).unwrap();

            unsafe {
                match record.level().into() {
                    OsLogType::Fault => _os_log_fault(self.log_object, c_string.as_ptr()),
                    OsLogType::Error => _os_log_error(self.log_object, c_string.as_ptr()),
                    OsLogType::Default => _os_log(self.log_object, c_string.as_ptr()),
                    OsLogType::Info => _os_log_info(self.log_object, c_string.as_ptr()),
                    OsLogType::Debug => _os_log_debug(self.log_object, c_string.as_ptr()),
                }
            };
        }
    }

    fn flush(&self) {}
}

impl Drop for OsLog {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.log_object) };
    }
}

/// Sets up logging with the default settings.
///
/// Generally, use the default `init()` function to perform logging using the system’s behavior. Use
/// [`init_custom()`][custom] only when you want to tag messages with a specific subsystem and
/// category for the purpose of filtering, or to customize the logging behavior of your subsystem
/// with a profile for debugging purposes.
///
/// [custom]: fn.init_custom.html
pub fn init() -> Result<(), SetLoggerError> {
    let log_object = unsafe { ::oslog_sys::_os_log_default_ptr() };
    let level = Level::Trace;
    let oslog = OsLog { log_object, level };

    log::set_boxed_logger(Box::new(oslog)).map(|()| log::set_max_level(level.to_level_filter()))
}

/// Sets up logging with custom settings.
///
/// `subsystem` is an identifier string, in reverse DNS notation, representing the subsystem that’s
/// performing logging. For example, `com.your_company.your_subsystem_name`. The subsystem is used
/// for categorization and filtering of related log messages, as well as for grouping related
/// logging settings.
///
/// `category` is a category within the specified subsystem. The category is used for categorization
/// and filtering of related log messages, as well as for grouping related logging settings within
/// the subsystem’s settings. A category’s logging settings override those of the parent subsystem.
///
/// Generally, use the default [`init()`][init] function to perform logging using the system’s
/// behavior. Use `init_custom()` only when you want to tag messages with a specific subsystem and
/// category for the purpose of filtering, or to customize the logging behavior of your subsystem
/// with a profile for debugging purposes.
///
/// [init]: fn.init.html
pub fn init_custom(subsystem: &str, category: &str) -> Result<(), SetLoggerError> {
    let c_subsystem = CString::new(subsystem).unwrap();
    let c_category = CString::new(category).unwrap();

    let log_object = unsafe { os_log_create(c_subsystem.as_ptr(), c_category.as_ptr()) };
    let level = Level::Trace;

    let oslog = OsLog { log_object, level };

    log::set_boxed_logger(Box::new(oslog)).map(|()| log::set_max_level(level.to_level_filter()))
}
