use std::{fmt::Display, fs};

use crate::lexer::Location;

/// A general error object.
///
/// This struct contains a location and message, where the location is where the error begins,
/// and the message is the specific "problem"
pub struct Error {
    location: Location,
    message: String,
}

impl Error {
    /// Constructs a new `Error` object.
    ///
    /// - `location`: the location of the error
    /// - `message`: the error's conveyed message
    pub fn new(location: Location, message: &str) -> Self {
        Self {
            location,
            message: message.to_owned(),
        }
    }

    /// Calculates the line and column of the `Error` location.
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

impl Display for Error {
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
