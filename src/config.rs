use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    sections: BTreeMap<String, BTreeMap<String, String>>,
}

impl Config {
    pub fn parse(input: &str) -> Result<Self, ConfigError> {
        let mut sections: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        let mut current_section = String::new();

        for (index, raw_line) in input.lines().enumerate() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].trim().to_owned();
                sections.entry(current_section.clone()).or_default();
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                return Err(ConfigError::InvalidLine {
                    line: index + 1,
                    content: raw_line.to_owned(),
                });
            };

            sections
                .entry(current_section.clone())
                .or_default()
                .insert(key.trim().to_owned(), value.trim().to_owned());
        }

        Ok(Self { sections })
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let input = fs::read_to_string(path).map_err(ConfigError::Io)?;
        Self::parse(&input)
    }

    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.sections
            .get(section)
            .and_then(|values| values.get(key))
            .map(String::as_str)
    }

    pub fn required(&self, section: &str, key: &str) -> Result<&str, ConfigError> {
        self.get(section, key)
            .ok_or_else(|| ConfigError::MissingKey {
                section: section.to_owned(),
                key: key.to_owned(),
            })
    }

    pub fn parse_value<T>(&self, section: &str, key: &str) -> Result<T, ConfigError>
    where
        T: FromStr,
        T::Err: Display,
    {
        let value = self.required(section, key)?;
        value
            .parse::<T>()
            .map_err(|error| ConfigError::InvalidValue {
                section: section.to_owned(),
                key: key.to_owned(),
                value: value.to_owned(),
                error: error.to_string(),
            })
    }

    pub fn get_bool(&self, section: &str, key: &str) -> Result<bool, ConfigError> {
        match self.required(section, key)?.to_ascii_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            value => Err(ConfigError::InvalidValue {
                section: section.to_owned(),
                key: key.to_owned(),
                value: value.to_owned(),
                error: "expected true or false".to_owned(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    InvalidLine {
        line: usize,
        content: String,
    },
    MissingKey {
        section: String,
        key: String,
    },
    InvalidValue {
        section: String,
        key: String,
        value: String,
        error: String,
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::InvalidLine { line, content } => {
                write!(f, "invalid config line {line}: {content}")
            }
            Self::MissingKey { section, key } => {
                write!(f, "missing config key [{section}] {key}")
            }
            Self::InvalidValue {
                section,
                key,
                value,
                error,
            } => {
                write!(f, "invalid config value [{section}] {key}={value}: {error}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
