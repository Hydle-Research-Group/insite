use std::path::PathBuf;

use crate::{
    error::Error,
    lexer::{Location, Token, TokenKind},
    parser::{Alignment, Command, Layout},
};

pub struct Parser {
    filename: PathBuf,
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(filename: &PathBuf, tokens: &[Token]) -> Self {
        Self {
            filename: filename.to_owned(),
            tokens: tokens.to_vec(),
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Command>, Error> {
        let mut commands = Vec::new();

        while !self.is_at_end() {
            if self.check(TokenKind::Dot) {
                self.advance();

                if self.check(TokenKind::Keyword {
                    value: "page".to_string(),
                }) {
                    self.advance();

                    commands.push(self.parse_page()?);
                } else if self.check(TokenKind::Keyword {
                    value: "section".to_string(),
                }) {
                    self.advance();

                    commands.push(self.parse_section()?);
                }
            }
        }

        Ok(commands)
    }

    /// Inspect the current token without consuming it.
    fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    /// Consume the current token.
    fn advance(&mut self) {
        if self.index < self.tokens.len() - 1 {
            self.index += 1;
        }
    }

    /// Check the current token's type.
    ///
    /// Returns `true` if `kind` matches the current token's kind.
    fn check(&self, kind: TokenKind) -> bool {
        self.tokens[self.index].kind() == kind
    }

    /// Consume a required token or return an error.
    ///
    /// - `kind`: the required token type
    ///
    /// # Errors
    ///
    /// This function will error if the current token doesn't match `kind`.
    fn expect(&mut self, kind: TokenKind) -> Result<(), Error> {
        let current = self.peek();

        if !self.check(kind.clone()) {
            Err(Error::new(
                current.location().to_owned(),
                &format!("expected '{}' got '{}'", kind, current.kind()),
            ))
        } else {
            self.advance(); // consume the token

            Ok(())
        }
    }

    /// Consume a string token, returning the token's value if consumed, otherwise return an error.
    ///
    /// # Errors
    ///
    /// This function will `Err` if the token isn't of `TokenType::String`.
    fn expect_string(&mut self) -> Result<String, Error> {
        let current = self.peek();

        match current.kind() {
            TokenKind::String { value } => {
                self.advance();

                Ok(value)
            }
            _ => Err(Error::new(
                current.location().to_owned(),
                &format!("expected 'string' got '{}'", current.kind()),
            )),
        }
    }

    /// Consume a keyword token, returning the token's value if consumed, otherwise return an error.
    ///
    /// # Errors
    ///
    /// This function will `Err` if the token isn't of `TokenType::Keyword`.
    fn expect_keyword(&mut self) -> Result<String, Error> {
        let current = self.peek();

        match current.kind() {
            TokenKind::Keyword { value } => {
                self.advance();

                Ok(value)
            }
            _ => Err(Error::new(
                current.location().to_owned(),
                &format!("expected 'keyword' got '{}'", current.kind()),
            )),
        }
    }

    /// Consume an indentifier token, returning the token's value if consumed, otherwise return an error.
    ///
    /// # Errors
    ///
    /// This function will `Err` if the token isn't of `TokenType::Identifier`.
    fn expect_indentifier(&mut self) -> Result<String, Error> {
        let current = self.peek();

        match current.kind() {
            TokenKind::Indentifier { value } => {
                self.advance();

                Ok(value)
            }
            _ => Err(Error::new(
                current.location().to_owned(),
                &format!("expected 'identifier' got '{}'", current.kind()),
            )),
        }
    }

    /// If the token stream has ended.
    fn is_at_end(&self) -> bool {
        self.check(TokenKind::EOF)
    }

    /// Parse a `.page` definition.
    fn parse_page(&mut self) -> Result<Command, Error> {
        let mut children = Vec::new();
        let location = self.peek().location().to_owned();

        let page_name = self.expect_string()?;

        self.expect(TokenKind::Comma)?;

        let page_path = self.expect_string()?;

        self.expect(TokenKind::Colon)?;

        children.append(&mut self.parse_commands()?);

        Ok(Command::Page {
            name: page_name,
            path: page_path,
            children,
            location,
        })
    }

    /// Parse a `.section` definition.
    fn parse_section(&mut self) -> Result<Command, Error> {
        let mut children = Vec::new();
        let location = self.peek().location().to_owned();

        let section_name = self.expect_indentifier()?;

        self.expect(TokenKind::Colon)?;

        children.append(&mut self.parse_commands()?);

        Ok(Command::Section {
            name: section_name,
            children,
            location,
        })
    }
    /// Parse a group of commands, stopping when the token is an EOF or '.' (indicating a
    /// section or page definition)
    fn parse_commands(&mut self) -> Result<Vec<Command>, Error> {
        let mut commands = Vec::new();

        while !self.check(TokenKind::EOF) && !self.check(TokenKind::Dot) {
            let value = self.expect_keyword()?;

            commands.push(self.parse_command(&value)?);
        }

        Ok(commands)
    }

    /// Parse a single command.
    ///
    /// - `value`: the command keyword
    ///
    /// # Errors
    ///
    /// This function will `Err` if the `value` is an unknown keyword.
    fn parse_command(&mut self, value: &str) -> Result<Command, Error> {
        let start = self.peek().location().to_owned();

        match value {
            "ld" => {
                let layout = match self.expect_keyword()?.as_str() {
                    "horizontal" => Layout::Horizontal,
                    "vertical" => Layout::Vertical,
                    _ => {
                        return Err(Error::new(
                            self.peek().location().to_owned(),
                            "expected 'horizontal' or 'vertical' layout",
                        ));
                    }
                };

                self.expect(TokenKind::Comma)?;

                let alignment = match self.expect_keyword()?.as_str() {
                    "right" => Alignment::Right,
                    "center" => Alignment::Center,
                    "left" => Alignment::Left,
                    _ => {
                        return Err(Error::new(
                            self.peek().location().to_owned(),
                            "expected 'right', 'center', or 'left' alignment",
                        ));
                    }
                };

                Ok(Command::Load {
                    layout: layout,
                    alignment: alignment,
                    location: start,
                })
            }
            "str" => Ok(Command::Store { location: start }),
            "jmp" => Ok(Command::Jump {
                section: self.expect_indentifier()?,
                location: start,
            }),
            "exp" => Ok(Command::Expand { location: start }),
            "md" => Ok(Command::Markdown {
                text: self.expect_string()?,
                location: start,
            }),
            _ => Err(Error::new(
                start,
                &format!("expected command, not '{}'", self.peek().kind()),
            )),
        }
    }
}
