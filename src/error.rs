use std::{fmt::Display, fs};

use crate::lexer::Location;

/// An error type.
#[derive(Debug)]
pub enum Error {
    /// A general error.
    General(String),
    /// A file system error.
    File(std::io::Error),
    /// A TOML parsing error.
    TomlParse(toml::de::Error),
    /// A language error.
    Language(LanguageError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::File(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlParse(err)
    }
}

impl From<LanguageError> for Error {
    fn from(value: LanguageError) -> Self {
        Self::Language(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::General(e) => f.write_str(&format!("insite: {}", e)),
            Error::File(e) => f.write_str(&format!("insite: {}", e)),
            Error::TomlParse(e) => f.write_str(&format!("insite: {}", e)),
            Error::Language(e) => e.fmt(f),
        }
    }
}

/// A language error.
///
/// This struct contains a location and message, where the location is where the error begins,
/// and the message is the specific "problem".
#[derive(Debug)]
pub struct LanguageError {
    location: Location,
    message: String,
}

impl LanguageError {
    /// Constructs a new `LanguageError` object.
    ///
    /// - `location`: the location of the error
    /// - `message`: the error's conveyed message
    pub fn new(location: Location, message: &str) -> Self {
        Self {
            location,
            message: message.to_owned(),
        }
    }

    /// Calculates the line and column of the internal `Location`.
    fn calculate_line_and_column(&self) -> (usize, usize) {
        if let Ok(contents) = fs::read_to_string(self.location.filename()) {
            let mut line = 1usize;
            let mut column = 0usize;

            for (i, character) in contents.chars().enumerate() {
                column += 1;

                if character == '\n' {
                    line += 1;
                    column = 0;
                }

                if i == self.location.start() {
                    return (line, column);
                }
            }
        }

        (0, 0)
    }
}

impl Display for LanguageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, column) = self.calculate_line_and_column();

        let _ = f.write_str(&format!(
            "insite: {} @@ {}:{}:{}",
            self.message,
            self.location.filename().to_owned().into_string().unwrap(),
            line,
            column
        ));

        Ok(())
    }
}
