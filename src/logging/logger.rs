use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::logging::DateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logger {
    output_enabled: bool,
    error_enabled: bool,
    log_dir: PathBuf,
}

impl Logger {
    pub fn new(output_enabled: bool, error_enabled: bool, log_dir: impl Into<PathBuf>) -> Self {
        Self {
            output_enabled,
            error_enabled,
            log_dir: log_dir.into(),
        }
    }

    pub fn from_config(config: &Config, log_dir: impl Into<PathBuf>) -> Self {
        Self::new(
            config.get_bool("Logging", "log.output").unwrap_or(false),
            config.get_bool("Logging", "log.errors").unwrap_or(false),
            log_dir,
        )
    }

    pub fn startup_lines(timestamp_millis: i64) -> Vec<String> {
        vec![
            String::new(),
            "-----------------------------------------".to_owned(),
            format!(
                "-- SERVER BOOT TIME: {}",
                DateTime::format_millis(timestamp_millis)
            ),
            "-----------------------------------------".to_owned(),
            String::new(),
            Self::line("Roseau - Rust Server", timestamp_millis),
            Self::line("Loading server...", timestamp_millis),
            Self::empty_line(timestamp_millis),
        ]
    }

    pub fn empty_line(timestamp_millis: i64) -> String {
        format!("{} [ROSEAU] ", Self::date_prefix(timestamp_millis))
    }

    pub fn line(message: impl Display, timestamp_millis: i64) -> String {
        format!(
            "{} [ROSEAU] >> {}",
            Self::date_prefix(timestamp_millis),
            message
        )
    }

    pub fn exception_lines(error_text: &str, timestamp_millis: i64) -> Vec<String> {
        vec![
            Self::line(
                "---------------------------------------------",
                timestamp_millis,
            ),
            Self::line("Error has occured!", timestamp_millis),
            error_text.to_owned(),
            Self::line(
                "---------------------------------------------",
                timestamp_millis,
            ),
        ]
    }

    pub fn write_output_line(&self, line: &str) -> io::Result<()> {
        if self.output_enabled {
            self.append_file("output.log", line)?;
        }
        Ok(())
    }

    pub fn write_error(&self, error_text: &str, timestamp_millis: i64) -> io::Result<()> {
        if !self.error_enabled {
            return Ok(());
        }

        self.append_file("error.log", "---------------------------------------------")?;
        self.append_file(
            "error.log",
            &format!(
                " {} - Error has occured!",
                DateTime::format_millis(timestamp_millis)
            ),
        )?;
        self.append_file("error.log", error_text)
    }

    pub fn write_exception(&self, error_text: &str, timestamp_millis: i64) -> io::Result<()> {
        for line in Self::exception_lines(error_text, timestamp_millis) {
            self.write_output_line(&line)?;
        }

        self.write_error(error_text, timestamp_millis)
    }

    pub fn output_enabled(&self) -> bool {
        self.output_enabled
    }

    pub fn error_enabled(&self) -> bool {
        self.error_enabled
    }

    pub fn log_dir(&self) -> &Path {
        &self.log_dir
    }

    fn date_prefix(timestamp_millis: i64) -> String {
        format!("[{}]", DateTime::format_millis(timestamp_millis))
    }

    fn append_file(&self, file_name: &str, line: &str) -> io::Result<()> {
        fs::create_dir_all(&self.log_dir)?;
        let path = self.log_dir.join(file_name);
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{line}")
    }
}

#[cfg(test)]
#[path = "logger_tests.rs"]
mod tests;
