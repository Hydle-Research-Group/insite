use crate::error::Error;
use std::path::PathBuf;
use toml::Value;

/// A build configuration.
#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub output: PathBuf,
    pub purge: bool,
    pub favicon: Option<PathBuf>,
    pub stylesheet: Option<PathBuf>,
    pub assets: Vec<PathBuf>,
    pub allow_markdown_html: bool,
    pub allow_gfm: bool,
}

impl BuildConfig {
    /// Construct a `BuildConfig` object from a TOML configuration file.
    ///
    /// - `input`: the TOML contents to parse
    ///
    /// # Errors
    ///
    /// This function will `Err` if `input` couldn't be parsed properly.
    pub fn from_toml(input: &str) -> Result<Self, Error> {
        let value: toml::Value = toml::from_str(input)?;

        let mut config = Self::default();

        if let Some(build) = value.get("build") {
            if let Some(output) = build.get("output").and_then(Value::as_str) {
                config.output = PathBuf::from(output);
            }
            if let Some(purge) = build.get("purge").and_then(Value::as_bool) {
                config.purge = purge;
            }
        };

        if let Some(resources) = value.get("resources") {
            if let Some(favicon) = resources.get("favicon").and_then(Value::as_str) {
                config.favicon = Some(PathBuf::from(favicon));
            }

            if let Some(stylesheet) = resources.get("stylesheet").and_then(Value::as_str) {
                config.stylesheet = Some(PathBuf::from(stylesheet));
            }

            if let Some(assets) = resources.get("assets").and_then(Value::as_array) {
                let mut f = Vec::new();

                for (i, file) in assets.iter().enumerate() {
                    match file {
                        Value::String(s) => {
                            f.push(PathBuf::from(s));
                        }
                        _ => {
                            return Err(Error::General(format!(
                                "resources.assets[{i}] must be a string"
                            )));
                        }
                    }
                }

                config.assets = f;
            }
        }

        if let Some(compiler) = value.get("compiler") {
            if let Some(allow_markdown_html) =
                compiler.get("allow-markdown-html").and_then(Value::as_bool)
            {
                config.allow_markdown_html = allow_markdown_html
            }
            if let Some(allow_gfm) = compiler.get("allow-gfm").and_then(Value::as_bool) {
                config.allow_gfm = allow_gfm
            }
        }

        Ok(config)
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output: PathBuf::from("output"),
            purge: true,
            favicon: None,
            stylesheet: None,
            assets: Vec::new(),
            allow_markdown_html: false,
            allow_gfm: true,
        }
    }
}
