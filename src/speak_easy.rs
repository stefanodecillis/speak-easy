//! # SpeakEasy
//!
//! This module contains the `SpeakEasy` struct and its associated methods.
//!
//! ## Structs
//!
//! - `SpeakEasy`: This struct represents the SpeakEasy logging system.
//!
//! ## Methods
//!
//! - `init`: This method initializes the SpeakEasy logging system. It takes a log level and an optional `SpeakConfig` struct. The `SpeakConfig` struct defines the rotation interval, the directory path for the logs, and the prefix for the log files.
//! - `keep_last_logs`: This method keeps the last `n` logs in the log directory and deletes the rest. It is used for log rotation.
//!

use crate::{
    formatter::LogsFileFormatter, processor::spawn_processor, Rotation, SpeakConfig
};
use tracing::Level;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{writer::MakeWriterExt, Layer},
    layer::SubscriberExt,
    reload,
};

pub struct SpeakEasy {}

impl SpeakEasy {
    pub fn init(log_level: Level, speak_easy_config: Option<SpeakConfig>) {
        let base_subscriber = tracing_subscriber::registry::Registry::default()
            .with(tracing_subscriber::fmt::layer().map_writer(|w| w.with_max_level(log_level)));

        match speak_easy_config {
            Some(speak_easy_config) => {
                let logfile = match &speak_easy_config.interval {
                    Rotation::Minutely => tracing_appender::rolling::minutely(
                        &speak_easy_config.directory_path,
                        &speak_easy_config.prefix,
                    ),
                    Rotation::Hourly => tracing_appender::rolling::hourly(
                        &speak_easy_config.directory_path,
                        &speak_easy_config.prefix,
                    ),
                    Rotation::Daily => tracing_appender::rolling::daily(
                        &speak_easy_config.directory_path,
                        &speak_easy_config.prefix,
                    ),
                    Rotation::Never => tracing_appender::rolling::never(
                        &speak_easy_config.directory_path,
                        &speak_easy_config.prefix,
                    ),
                };

                let (filter, _reload_handle) =
                    reload::Layer::new(LevelFilter::from_level(log_level));

                let with_file_subscriber = base_subscriber.with(filter).with(
                    Layer::new()
                        .pretty()
                        .with_level(false)
                        .with_ansi(false)
                        .event_format(LogsFileFormatter)
                        .with_writer(logfile),
                );
                if tracing::subscriber::set_global_default(with_file_subscriber).is_err() {
                    tracing::warn!("A global default tracing subscriber has already been set.");
                }

                if speak_easy_config.clone().cleanup {
                    spawn_processor(speak_easy_config.directory_path.to_string(), speak_easy_config.prefix.to_string(), speak_easy_config.cleanup_interval, speak_easy_config.keep_last)
                }
            }
            None => {
                if tracing::subscriber::set_global_default(base_subscriber).is_err() {
                    tracing::warn!("A global default tracing subscriber has already been set.");
                }
            }
        };
    }
}
