use std::{fs, path::PathBuf, thread::panicking};

use markdown::{CompileOptions, Options};

use crate::{
    config::BuildConfig,
    error::{Error, LanguageError},
    lexer::{Location, Token, TokenKind},
    parser::{Alignment, Command, Layout},
};

/// An Insite parser.
pub struct Parser {
    filename: PathBuf,
    config: BuildConfig,
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    /// Constructs a new `Parser` object.
    ///
    /// - `filename`: the name of the parsed file
    /// - `config`: the build configuration
    /// - `tokens`: the token stream to parse
    pub fn new(filename: &PathBuf, config: &BuildConfig, tokens: &[Token]) -> Self {
        Self {
            filename: filename.to_owned(),
            config: config.to_owned(),
            tokens: tokens.to_vec(),
            index: 0,
        }
    }

    /// Parse the parser's tokens into a `Vec` of `Command` objects.
    ///
    /// # Errors
    ///
    /// This function will `Err` if the file couldn't be parsed properly (due to syntax
    /// errors or related issues.)
    pub fn parse(&mut self) -> Result<Vec<Command>, Error> {
        let mut commands = Vec::new();

        while !self.is_at_end() {
            self.expect(TokenKind::Dot)?;

            let start = self.peek().location().to_owned();
            let keyword = self.expect_keyword()?;

            match &*keyword {
                "page" => commands.push(self.parse_page()?),
                "section" => commands.push(self.parse_section()?),
                _ => {
                    return Err(Error::Language(LanguageError::new(
                        start,
                        "expected a 'page' or 'section' definition",
                    )));
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
            Err(Error::Language(LanguageError::new(
                current.location().to_owned(),
                &format!("expected '{}' got '{}'", kind, current.kind()),
            )))
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
            _ => Err(Error::Language(LanguageError::new(
                current.location().to_owned(),
                &format!("expected 'string' got '{}'", current.kind()),
            ))),
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
            _ => Err(Error::Language(LanguageError::new(
                current.location().to_owned(),
                &format!("expected 'keyword' got '{}'", current.kind()),
            ))),
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
            _ => Err(Error::Language(LanguageError::new(
                current.location().to_owned(),
                &format!("expected 'identifier' got '{}'", current.kind()),
            ))),
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

    /// Parse a `md` or `mdf` (markdown) command, converting it into HTML.
    ///
    /// - `is_file`: if the preceding string is a file path (in this case, `true` for the `mdf` command)
    ///
    /// If `is_file` is `true`, the following string is parsed into a `PathBuf`, then read as a file.
    ///
    /// # Errors
    ///
    /// This function will `Err` if the markdown cannot be parsed, which is unlikely. Additionally,
    /// the function will `Err` if the markdown file can't be read, or is not valid markdown.
    fn parse_markdown(&mut self, is_file: bool) -> Result<Command, Error> {
        let start = self.peek().location().to_owned();

        let md_string = self.expect_string()?;
        let contents = if is_file {
            let path = PathBuf::from(&md_string);

            if !path.exists()
                || !path
                    .extension()
                    .is_some_and(|ext| ext.to_ascii_lowercase() == "md")
            {
                return Err(Error::Language(LanguageError::new(
                    start,
                    "invalid path (file doesn't exist or isn't a valid markdown file)",
                )));
            }

            fs::read_to_string(&path).map_err(|e| {
                Error::Language(LanguageError::new(
                    start.clone(),
                    &format!("unable to read file '{e}'"),
                ))
            })?
        } else {
            md_string
        };

        let mut compile_options = if self.config.allow_gfm {
            CompileOptions::gfm()
        } else {
            CompileOptions::default()
        };
        compile_options.allow_dangerous_html = self.config.allow_markdown_html; // enable HTML injection

        Ok(Command::Markdown {
            html: markdown::to_html_with_options(
                &contents,
                &Options {
                    compile: compile_options,
                    ..Default::default()
                },
            )
            .map_err(|_| LanguageError::new(start.clone(), "error parsing markdown string"))?,
            location: start,
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
                        return Err(Error::Language(LanguageError::new(
                            self.peek().location().to_owned(),
                            "expected 'horizontal' or 'vertical' layout",
                        )));
                    }
                };

                self.expect(TokenKind::Comma)?;

                let alignment = match self.expect_keyword()?.as_str() {
                    "right" => Alignment::Right,
                    "center" => Alignment::Center,
                    "left" => Alignment::Left,
                    _ => {
                        return Err(Error::Language(LanguageError::new(
                            self.peek().location().to_owned(),
                            "expected 'right', 'center', or 'left' alignment",
                        )));
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
            "md" => Ok(self.parse_markdown(false)?),
            "mdf" => Ok(self.parse_markdown(true)?),
            _ => Err(Error::Language(LanguageError::new(
                start,
                &format!("expected command, not '{}'", self.peek().kind()),
            ))),
        }
    }
}
