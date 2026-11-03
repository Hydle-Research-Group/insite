use std::{fmt::Display, path::PathBuf};
use toml::Value;

#[derive(Debug)]
pub enum ConfigError {
    Parse(toml::de::Error),
    InvalidField(String),
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        Self::Parse(err)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Parse(e) => f.write_str(&e.to_string()),
            ConfigError::InvalidField(e) => f.write_str(e),
        }
    }
}

/// A build configuration.
pub struct BuildConfig {
    pub output: PathBuf,
    pub style: Option<PathBuf>,
    pub files: Vec<PathBuf>,
}

impl BuildConfig {
    /// Construct a `BuildConfig` object from a TOML configuration file.
    ///
    /// - `input`: the TOML contents to parse
    pub fn from_toml(input: &str) -> Result<Self, ConfigError> {
        let value: toml::Value = toml::from_str(input)?;

        let Some(build) = value.get("build") else {
            return Ok(Self::default());
        };

        let mut config = Self::default();

        if let Some(output) = build.get("output").and_then(Value::as_str) {
            config.output = PathBuf::from(output);
        }

        if let Some(style) = build.get("style").and_then(Value::as_str) {
            config.style = Some(PathBuf::from(style));
        }

        let Some(resources) = value.get("resources") else {
            return Ok(Self::default());
        };

        if let Some(files) = resources.get("files").and_then(Value::as_array) {
            let mut f = Vec::new();

            for (i, file) in files.iter().enumerate() {
                match file {
                    Value::String(s) => {
                        f.push(PathBuf::from(s));
                    }
                    _ => {
                        return Err(ConfigError::InvalidField(format!(
                            "resources.files[{i}] must be a string"
                        )));
                    }
                }
            }

            config.files = f;
        }

        Ok(config)
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output: PathBuf::from("output"),
            style: None,
            files: Vec::new(),
        }
    }
}
