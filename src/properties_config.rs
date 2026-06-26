use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertiesConfig {
    file: Option<PathBuf>,
    values: BTreeMap<String, String>,
}

impl PropertiesConfig {
    pub fn parse(input: &str) -> Self {
        let mut values = BTreeMap::new();

        for raw_line in input.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                values.insert(key.trim().to_owned(), value.trim().to_owned());
            } else if let Some((key, value)) = line.split_once(':') {
                values.insert(key.trim().to_owned(), value.trim().to_owned());
            }
        }

        Self { file: None, values }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, PropertiesConfigError> {
        let path = path.as_ref();
        let input = fs::read_to_string(path).map_err(PropertiesConfigError::Io)?;
        let mut config = Self::parse(&input);
        config.file = Some(path.to_path_buf());
        Ok(config)
    }

    pub fn file(&self) -> Option<&Path> {
        self.file.as_deref()
    }

    pub fn set_file(&mut self, file: impl Into<PathBuf>) {
        self.file = Some(file.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn get_integer(&self, key: &str) -> Result<i32, PropertiesConfigError> {
        let value = self.required(key)?;
        value
            .parse::<i32>()
            .map_err(|error| PropertiesConfigError::InvalidInteger {
                key: key.to_owned(),
                value: value.to_owned(),
                error: error.to_string(),
            })
    }

    pub fn get_boolean(&self, key: &str) -> bool {
        self.get(key)
            .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    fn required(&self, key: &str) -> Result<&str, PropertiesConfigError> {
        self.get(key)
            .ok_or_else(|| PropertiesConfigError::MissingKey(key.to_owned()))
    }
}

#[derive(Debug)]
pub enum PropertiesConfigError {
    Io(io::Error),
    MissingKey(String),
    InvalidInteger {
        key: String,
        value: String,
        error: String,
    },
}

impl Display for PropertiesConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::MissingKey(key) => write!(f, "missing property {key}"),
            Self::InvalidInteger { key, value, error } => {
                write!(f, "invalid integer property {key}={value}: {error}")
            }
        }
    }
}

impl std::error::Error for PropertiesConfigError {}

#[cfg(test)]
#[path = "properties_config_tests.rs"]
mod tests;
