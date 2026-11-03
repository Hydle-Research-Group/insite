use crate::error::Error;
use std::path::PathBuf;
use toml::Value;

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
    pub fn from_toml(input: &str) -> Result<Self, Error> {
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
                        return Err(Error::General(format!(
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
