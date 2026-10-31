use std::fmt::Display;

use crate::lexer::Location;

/// A token type.
///
/// In Insite, token types represent:
///
/// - Individual characters (`.`)
/// - Indentifiers (`some_text`)
/// - Keywords (`page`)
/// - Strings (`"Hello, world!"`)
/// - Integers (`1`) and float (`3.14`) values are also accounted for, but are not used in the language currently.
#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    Integer {
        value: isize,
    },
    Float {
        value: f64,
    },
    String {
        value: String,
    },
    Keyword {
        value: String,
    },
    Indentifier {
        value: String,
    },
    Dot,
    Colon,
    Comma,
    /// End of file.
    EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(match self {
            TokenKind::Integer { value } => "integer",
            TokenKind::Float { value } => "float",
            TokenKind::String { value } => "string",
            TokenKind::Keyword { value } => value,
            TokenKind::Indentifier { value } => "indentifier",
            TokenKind::Dot => ".",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::EOF => "EOF",
        });

        Ok(())
    }
}

/// A token.
///
/// In Insite, tokens represent a lexed section of characters that can be fed into an Insite parser.
#[derive(Debug, Clone)]
pub struct Token {
    location: Location,
    kind: TokenKind,
}

impl Token {
    /// Constructs a new `Token` object.
    ///
    /// - `location`: the location of the token
    /// - `kind`: the type of token
    pub fn new(location: Location, kind: TokenKind) -> Self {
        Self { location, kind }
    }

    /// Returns the location of the `Token`.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns the kind of the `Token`.
    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }
}
