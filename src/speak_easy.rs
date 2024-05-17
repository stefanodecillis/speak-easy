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
    formatter::LogsFileFormatter,
    {Rotation, SpeakConfig},
};
use tracing::Level;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{writer::MakeWriterExt, Layer},
    layer::SubscriberExt,
    reload,
};

use std::fs;
use std::io;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

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
                    let directory_path = Arc::new(speak_easy_config.directory_path.clone());
                    let prefix = Arc::new(speak_easy_config.prefix.clone());
                    let cleanup_interval = Arc::new(speak_easy_config.cleanup_interval);
                    let holds_num = Arc::new(speak_easy_config.keep_last);

                    tokio::spawn(async move {
                        loop {
                            let _ = SpeakEasy::keep_last_logs(&directory_path, &prefix, &holds_num);
                            sleep(Duration::from_secs(*cleanup_interval)).await;
                        }
                    });
                }
            }
            None => {
                if tracing::subscriber::set_global_default(base_subscriber).is_err() {
                    tracing::warn!("A global default tracing subscriber has already been set.");
                }
            }
        };
    }

    fn keep_last_logs(directory_path: &str, prefix: &str, holds_num: &usize) -> io::Result<()> {
        // Read the directory
        let mut entries = fs::read_dir(directory_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        // Filter the entries to only include files that start with the prefix
        entries.retain(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with(prefix))
                .unwrap_or(false)
        });

        // Sort the entries by modified date in descending order
        entries.sort_by_key(|path| fs::metadata(path).and_then(|meta| meta.modified()).unwrap());
        entries.reverse();

        // Remove all but the last five entries
        for path in entries.into_iter().skip(*holds_num) {
            fs::remove_file(path)?;
        }

        Ok(())
    }
}
