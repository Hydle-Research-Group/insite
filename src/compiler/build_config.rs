use std::path::PathBuf;

/// A build configuration.
pub struct BuildConfig {
    pub output: PathBuf,
    pub pretty: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output: PathBuf::new(),
            pretty: false,
        }
    }
}
