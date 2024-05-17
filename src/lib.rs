#![allow(clippy::style)]

//! # speak-easy
//!
//! Logging functionalities with different levels and rotation options built on top of tokio-rs tracing.
//!
//! ## Features
//!| Feature                          | Status |
//!|----------------------------------|--------|
//!| Console logging                  | ✅      |
//!| File logging                     | ✅      |
//!| Rotation options                 | ✅      |
//!| Dispatch logs to multiple destinations | ❌  |
//!
//! ## Modules
//!
//! - `formatter`: This module contains the functionality for formatting the logs.
//! - `log_levels`: This module defines the different levels of logging.
//! - `speak_easy`: This module contains the functionality for the SpeakEasy logging system.
//!
//! ## Enums
//!
//! - `Rotation`: Defines the rotation options for the logs. Options are `Minutely`, `Hourly`, `Daily`, and `Never`.
//!
//! ## Structs
//!
//! - `SpeakConfig`: Defines the configuration for the SpeakEasy logging system. It includes the rotation interval, the directory path for the logs, and the prefix for the log files.
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! speak-easy = "0.1.0"
//! ```
//!
//! Then add this to your crate root:
//!
//! ```rust
//! use speak_easy::{debug, error, info, trace, warn};
//! use speak_easy::{speak_easy::SpeakEasy, Level, Rotation, SpeakConfig};
//! ```
//!
//! You can then use the logging functionality as follows:
//!
//! ```rust
//! # use speak_easy::{debug, error, info, trace, warn};
//! # use speak_easy::{speak_easy::SpeakEasy, Level, Rotation, SpeakConfig};
//! let speak_config = SpeakConfig::new(Rotation::Minutely, "./logs".to_string(), "my_log".to_string());
//! SpeakEasy::init(
//!     Level::INFO,
//!     Some(speak_config),
//! );
//!
//! info!("This is an info log");
//! debug!("This is a debug log");
//! warn!("This is a warning log");
//! error!("This is an error log");
//! trace!("This is a trace log");
//! ```
//!
//! This will create logs with different levels and rotate them hourly.
//!
//!
//! ### With Cleanup
//!
//! You can also set up log cleanup with the `with_cleanup` method:
//!
//! ```rust
//! # use speak_easy::{debug, error, info, trace, warn};
//! # use speak_easy::{speak_easy::SpeakEasy, Level, Rotation, SpeakConfig};
//! let speak_config = SpeakConfig::new(Rotation::Minutely, "./logs".to_string(), "my_log".to_string())
//!    .with_cleanup(24 * 60 * 60, 5);
//! SpeakEasy::init(
//!    Level::INFO,
//!   Some(speak_config),
//! );
//! ```
//!
//! This will create logs with different levels, rotate them minutely, and clean up the logs every 24 hours, keeping the last 5 logs.
//!
//! ## License
//!
//! This project is licensed under the MIT license.

mod formatter;
pub use tracing::{debug, error, info, trace, warn, Level};
pub mod speak_easy;

#[derive(Clone, Debug)]
pub enum Rotation {
    Minutely,
    Hourly,
    Daily,
    Never,
}

#[derive(Clone, Debug)]
pub struct SpeakConfig {
    pub interval: Rotation,
    pub directory_path: String,
    pub prefix: String,
    pub cleanup: bool,
    pub cleanup_interval: u64,
    pub keep_last: usize,
}

impl Default for SpeakConfig {
    fn default() -> Self {
        Self {
            interval: Rotation::Never,
            directory_path: "./logs".to_string(),
            prefix: "log".to_string(),
            cleanup: false,
            cleanup_interval: 24 * 60 * 60, // 24 hours
            keep_last: 5,
        }
    }
}

impl SpeakConfig {
    /// Creates a new `SpeakConfig` with the specified rotation interval, directory path, and log file prefix.
    ///
    /// # Arguments
    ///
    /// * `interval` - The rotation interval for the logs. Options are `Minutely`, `Hourly`, `Daily`, and `Never`.
    /// * `directory_path` - The directory path where the log files will be stored.
    /// * `prefix` - The prefix for the log files.
    ///
    /// # Example
    ///
    /// ```rust
    /// use speak_easy::{Rotation, SpeakConfig};
    ///
    /// let speak_config = SpeakConfig::new(Rotation::Minutely, "./logs".to_string(), "my_log".to_string());
    /// ```
    pub fn new(interval: Rotation, directory_path: String, prefix: String) -> Self {
        Self {
            interval,
            directory_path,
            prefix,
            cleanup: SpeakConfig::default().cleanup,
            cleanup_interval: SpeakConfig::default().cleanup_interval,
            keep_last: SpeakConfig::default().keep_last,
        }
    }

    /// Sets the cleanup options for the log files.
    ///
    /// # Arguments
    ///
    /// * `cleanup_interval` - The interval (in seconds) at which the log files should be cleaned up.
    /// * `keep_last` - The number of log files to keep.
    ///
    /// # Example
    ///
    /// ```rust
    /// use speak_easy::{Rotation, SpeakConfig};
    ///
    /// let speak_config = SpeakConfig::new(Rotation::Minutely, "./logs".to_string(), "my_log".to_string())
    ///     .with_cleanup(24 * 60 * 60, 5);
    /// ```
    pub fn with_cleanup(mut self, cleanup_interval: u64, keep_last: usize) -> Self {
        self.cleanup = true;
        self.cleanup_interval = cleanup_interval;
        self.keep_last = keep_last;
        self
    }
}

impl SpeakConfig {}
