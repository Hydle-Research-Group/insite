use std::{collections::HashMap, path::PathBuf};

use crate::{
    error::Error,
    lexer::{Location, Token, TokenKind},
};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LETTERS_DIGITS_SYMBOLS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const KEYWORDS: &[&str] = &[
    "page",
    "section",
    "ld",
    "str",
    "jmp",
    "exp",
    "md",
    "horizontal",
    "vertical",
    "right",
    "center",
    "left",
];

/// An Insite Lexer.
pub struct Lexer {
    current_char: Option<char>,
    index: usize,
    filename: PathBuf,
    contents: String,
}

impl Lexer {
    /// Constructs a new `Lexer` object.
    ///
    /// - `filename`: the name of the lexed file
    /// - `contents`: the contents of the lexed file
    pub fn new(filename: &PathBuf, contents: &str) -> Self {
        let mut lexer = Self {
            current_char: None,
            index: 0,
            filename: filename.to_owned(),
            contents: contents.trim_end().to_owned(),
        };

        lexer.advance();
        lexer
    }

    /// Run the lexer.
    ///
    /// This function will return a `Vec` of `Token` objects if the file was lexed correctly.
    ///
    /// # Errors
    ///
    /// This function will return an `Err` if the file wasn't lexed properly, due to grammar errors
    /// or language inconsistencies.
    pub fn run(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while let Some(character) = self.current_char {
            let start = self.index;

            match character {
                c if c.is_whitespace() => {
                    self.advance();
                }
                c if LETTERS.contains(c) => {
                    tokens.push(self.make_identifier_or_keyword());
                }
                '"' => {
                    tokens.push(self.make_string()?);
                }
                '.' => {
                    tokens.push(Token::new(
                        Location::new(start, start + 1, &self.filename),
                        TokenKind::Dot,
                    ));
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::new(
                        Location::new(start, start + 1, &self.filename),
                        TokenKind::Comma,
                    ));
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::new(
                        Location::new(start, start + 1, &self.filename),
                        TokenKind::Colon,
                    ));
                    self.advance();
                }
                ';' => {
                    self.skip_comment();
                }
                _ => {
                    return Err(Error::new(
                        Location::new(start, start + 1, &self.filename),
                        "unknown character",
                    ));
                }
            }
        }

        // end of file
        tokens.push(Token::new(
            Location::new(self.index, self.index, &self.filename),
            TokenKind::EOF,
        ));

        Ok(tokens)
    }

    /// Advances one character forward, incrementing the next index.
    fn advance(&mut self) {
        let chars = self.contents.chars().collect::<Vec<char>>();

        if (self.index as usize) < chars.len() {
            self.current_char = Some(chars[self.index]);
        } else {
            self.current_char = None;
        }

        self.index += 1;
    }

    /// Skips any characters until the next `\n` character.
    fn skip_comment(&mut self) {
        self.advance();

        while let Some(character) = self.current_char {
            if character != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Constructs a `Token` with an `Identifier` or `Keyword` kind depending on the parsed characters.
    fn make_identifier_or_keyword(&mut self) -> Token {
        let mut id_string = String::new();
        let pos_start = self.index;

        while let Some(character) = self.current_char {
            if LETTERS_DIGITS_SYMBOLS.contains(character) {
                id_string.push(character);

                self.advance();
            } else {
                break;
            }
        }

        let pos_end = self.index;

        let token_kind = if KEYWORDS.contains(&id_string.as_str()) {
            TokenKind::Keyword { value: id_string }
        } else {
            TokenKind::Indentifier { value: id_string }
        };

        Token::new(
            Location::new(pos_start, pos_end, &self.filename),
            token_kind,
        )
    }

    /// Constructs a `Token` with a `String` type.
    ///
    /// # Errors
    ///
    /// This function will `Err` if there is an invalid escape character, or if the string is not terminated.
    fn make_string(&mut self) -> Result<Token, Error> {
        let mut string = String::new();
        let pos_start = self.index;
        let mut escape_char = false;

        self.advance();

        let mut escape_chars = HashMap::new();
        escape_chars.insert('r', '\r');
        escape_chars.insert('n', '\n');
        escape_chars.insert('t', '\t');
        escape_chars.insert('\\', '\\');
        escape_chars.insert('"', '\"');

        while let Some(character) = self.current_char {
            if character == '"' && !escape_char {
                break;
            }

            if escape_char {
                if let Some(replacement) = escape_chars.get(&character) {
                    string.push(*replacement);
                    self.advance();
                } else {
                    return Err(Error::new(
                        Location::new(pos_start, self.index, &self.filename),
                        "invalid escape character",
                    ));
                }

                escape_char = false;

                continue;
            }

            if character == '\\' {
                escape_char = true;
            } else {
                string.push(character);
            }

            self.advance();
        }

        if self.current_char != Some('"') {
            return Err(Error::new(
                Location::new(pos_start, self.index, &self.filename),
                "string not terminated",
            ));
        }

        self.advance();

        let pos_end = self.index;

        Ok(Token::new(
            Location::new(pos_start, pos_end, &self.filename),
            TokenKind::String { value: string },
        ))
    }
}
