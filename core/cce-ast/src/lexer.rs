/*

Copyright (C) 2023 Carlos Kieliszewski

This file is part of the Circe Project.

Circe is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option)
any later version.

Circe is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
Circe. If not, see <https://www.gnu.org/licenses/>. 

*/

use cce_stream::InputStream;

use thiserror::Error;

pub struct Lexer {
  pub(crate) stream: InputStream,
  pub(crate) peeked: Option<Token>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Identifier(String),
  Keyword(String),
  Literal(String),
  Punctuation(char),
  LowLevelSequence(String)
}

#[derive(Error, Debug)]
pub enum LexerError {
  #[error("{0}")]
  InputStreamError(#[from] cce_stream::InputStreamError),
  #[error("Unexpected end of stream")]
  UnexpectedEndOfStream,
  #[error("Unexpected character: {0}")]
  UnexpectedCharacter(char)
}


impl Lexer {
  pub fn new(stream: InputStream) -> Lexer {
    Lexer {
      stream,
      peeked: None
    }
  }

  fn create_ident_or_keyword(&mut self) -> Result<Token, LexerError> {
    let mut ident = String::new();
    let mut c: Option<char> = self.stream.peek()?;

    loop {
      if let Some(ch) = c {
        if ch.is_alphanumeric() || ch == '_' {
          ident.push(ch);
          self.stream.next()?;
          c = self.stream.peek()?;
        } else {
          break;
        }
      } else {
        break;
      }
    }

    match ident.as_str() {
      "howto" | "whatis" => {
        Ok(Token::Keyword(ident))
      },
      _ => {
        Ok(Token::Identifier(ident))
      }
    }
  }

  fn create_string_literal(&mut self) -> Result<Token, LexerError> {
    let mut literal: String = String::new();
    let mut c: Option<char> = self.stream.peek()?;

    loop {
      if let Some(ch) = c {
        if ch == '\'' {
          self.stream.next()?;
          break;
        } else {
          literal.push(ch);
          self.stream.next()?;
          c = self.stream.peek()?;
        }
      } else {
        return Err(LexerError::UnexpectedEndOfStream);
      }
    };

    Ok(Token::Literal(literal))
  }

  fn create_low_level_sequence(&mut self) -> Result<Token, LexerError> {
    let mut sequence: String = String::new();
    let mut c: Option<char> = self.stream.peek()?;

    loop {
      if let Some(ch) = c {
        if ch == '*' {
          self.stream.next()?;
          break;
        } else {
          sequence.push(ch);
          self.stream.next()?;
          c = self.stream.peek()?;
        }
      }
    };

    Ok(Token::LowLevelSequence(sequence))
  }

  pub fn next(&mut self) -> Result<Option<Token>, LexerError> {
    if self.peeked.is_some() {
      let tok = self.peeked.clone();
      self.peeked = None;
      return Ok(tok);
    };

    let mut c: char = match self.stream.peek()? {
      Some(c) => c,
      None => {
        return Ok(None)
      }
    };

    while c.is_whitespace() {
      self.stream.next()?;
      c = match self.stream.peek()? {
        Some(c) => c,
        None => {
          return Ok(None)
        }
      };
    };

    match c {
      'a'..='z' | 'A'..='Z' | '_' => {
        Ok(Some(self.create_ident_or_keyword()?))
      },
      '\'' => {
        self.stream.next()?;
        Ok(Some(self.create_string_literal()?))
      },
      '-' | '|' | '.' => {
        self.stream.next()?;
        Ok(Some(Token::Punctuation(c)))
      },
      '*' => {
        self.stream.next()?;
        Ok(Some(self.create_low_level_sequence()?))
      },
      _ => {
        Err(LexerError::UnexpectedCharacter(c))
      }
    }
  }

  pub fn peek(&mut self) -> Result<Option<Token>, LexerError> {
    if self.peeked.is_none() {
      self.peeked = self.next()?;
    };

    Ok(self.peeked.clone())
  }
}

impl From<String> for Lexer {
  fn from(s: String) -> Lexer {
    Lexer::new(InputStream::from(s))
  }
}

impl From<&str> for Lexer {
  fn from(s: &str) -> Lexer {
    Lexer::new(InputStream::from(s))
  }
}

impl From<&[u8]> for Lexer {
  fn from(s: &[u8]) -> Lexer {
    Lexer::new(InputStream::from(s))
  }
}