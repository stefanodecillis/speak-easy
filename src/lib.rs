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
//!
//! You can then use the logging functionality as follows:
//!
//! ```rust
//! use speak-easy::{debug, error, info, trace, warn};
//! use speak_easy::{speak_easy::SpeakEasy, Level, Rotation, SpeakConfig};
//!
//! let config = SpeakConfig {
//!     interval: Rotation::Hourly,
//!     directory_path: "./logs".to_string(),
//!     prefix: "my_log".to_string(),
//! };
//! SpeakEasy::init(
//!     Level::INFO,
//!     Some(SpeakConfig {
//!        interval: ::speak_easy::log::Rotation::Minutely,
//!        directory_path: "./logs".to_string(),
//!        prefix: "indid".to_string(),
//!    }),
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

mod formatter;
pub use tracing::{debug, error, info, trace, warn, Level};
pub mod speak_easy;
pub enum Rotation {
    Minutely,
    Hourly,
    Daily,
    Never,
}

pub struct SpeakConfig {
    pub interval: Rotation,
    pub directory_path: String,
    pub prefix: String,
}

impl Default for SpeakConfig {
    fn default() -> Self {
        Self {
            interval: Rotation::Never,
            directory_path: "./logs".to_string(),
            prefix: "log".to_string(),
        }
    }
}
