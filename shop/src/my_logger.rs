// src/my_logger.rs

use colored::*;
use log::{Level, Log, Metadata, Record};
use std::time::Instant;

pub fn test() {
    log::debug!("Test debug message");
    log::info!("Test info message");
    log::warn!("Test warning message");
    log::error!("Test error message");
}
pub struct MyLogger {
    start_time: Instant,
    newline_len: usize, //print new line if log longer than this
}

impl MyLogger {
    pub fn new() -> Self {
        MyLogger {
            start_time: Instant::now(),
            newline_len: 120,
        }
    }

    fn elapsed_time(&self) -> String {
        let elapsed = self.start_time.elapsed();
        let seconds = elapsed.as_secs();
        let milliseconds = elapsed.subsec_millis();

        format!("{:04}.{:03}", seconds, milliseconds)
    }
}

impl Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug // Allow Debug level and higher
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Get the elapsed time since program start.
            let elapsed_time = self.elapsed_time();

            // Format the log level with uniform padding to a length of 6 characters.
            let padded_log_level =
                format!("{:5}", format!("{}", record.level())).color(match record.level() {
                    Level::Error => Color::Red,
                    Level::Warn => Color::Yellow,
                    Level::Info => Color::Green,
                    Level::Debug => Color::Cyan,
                    Level::Trace => Color::White,
                });

            // Format the log message with the padded log level and color.
            let log_message = format!(
                "[{}][{}][{}] {}",
                elapsed_time,
                padded_log_level,
                record.module_path().unwrap_or("Unknown"),
                record.args()
            );

            // Check the length of the log message and add a new line if it's longer than newline_len characters.
            if log_message.len() > self.newline_len {
                println!("{}\n", log_message);
            } else {
                println!("{}", log_message);
            }
        }
    }

    fn flush(&self) {}
}
