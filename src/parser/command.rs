use std::fmt::Display;

use crate::lexer::Location;

/// A layout value, either horizontal or vertical.
#[derive(Debug, Clone)]
pub enum Layout {
    Vertical,
    Horizontal,
}

/// An alignment value, either left, center, or right
#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(match self {
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
        });

        Ok(())
    }
}

/// An Insite command.
///
/// In the Insite language, commands are single lines that contain keywords, strings, or other definitions.
/// These lines are parsed into a collection of `Command` enums that can be read by the Insite compiler to build
/// the necessary HTML.
#[derive(Debug, Clone)]
pub enum Command {
    Page {
        name: String,
        path: String,
        children: Vec<Command>,
        location: Location,
    },
    Section {
        name: String,
        children: Vec<Command>,
        location: Location,
    },
    Load {
        layout: Layout,
        alignment: Alignment,
        location: Location,
    },
    Store {
        location: Location,
    },
    Jump {
        section: String,
        location: Location,
    },
    Expand {
        location: Location,
    },
    Markdown {
        text: String,
        location: Location,
    },
}

impl Command {
    /// Returns the location of the command.
    pub fn location(&self) -> &Location {
        match self {
            Command::Page { location, .. }
            | Command::Section { location, .. }
            | Command::Load { location, .. }
            | Command::Store { location }
            | Command::Jump { location, .. }
            | Command::Expand { location }
            | Command::Markdown { location, .. } => location,
        }
    }
}
