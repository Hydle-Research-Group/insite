use std::path::PathBuf;

/// A build configuration.
pub struct BuildConfig {
    pub output: PathBuf,
    pub style: Option<PathBuf>,
    pub favicon: Option<PathBuf>,
}

impl BuildConfig {
    /// Construct a `BuildConfig` object from a TOML configuration file.
    ///
    /// - `input`: the TOML contents to parse
    pub fn from_toml(input: &str) -> Result<Self, toml::de::Error> {
        let value: toml::Value = toml::from_str(input)?;

        let Some(build) = value.get("build") else {
            return Ok(Self::default());
        };

        let mut config = Self::default();

        if let Some(output) = build.get("output").and_then(toml::Value::as_str) {
            config.output = PathBuf::from(output);
        }

        if let Some(style) = build.get("style").and_then(toml::Value::as_str) {
            config.style = Some(PathBuf::from(style));
        }

        if let Some(favicon) = build.get("favicon").and_then(toml::Value::as_str) {
            config.favicon = Some(PathBuf::from(favicon))
        }

        Ok(config)
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output: PathBuf::from("output"),
            style: None,
            favicon: None,
        }
    }
}
