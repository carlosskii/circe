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
      }
    };

    Ok(Token::Literal(literal))
  }

  fn create_low_level_sequence(&mut self) -> Result<Token, LexerError> {
    let mut sequence: String = String::new();
    let mut c: Option<char> = self.stream.peek()?;
    let mut dash_found: bool = false;

    loop {
      if let Some(ch) = c {
        if ch == '-' {
          if dash_found {
            sequence.push(ch);
            self.stream.next()?;
            c = self.stream.peek()?;
          } else {
            dash_found = true;
            self.stream.next()?;
            c = self.stream.peek()?;
          }
        } else if ch == '|' {
          if dash_found {
            sequence.push(ch);
            self.stream.next()?;
            c = self.stream.peek()?;
          } else {
            return Err(LexerError::UnexpectedCharacter(ch));
          }
        } else if ch == '.' {
          if dash_found {
            sequence.push(ch);
            self.stream.next()?;
            c = self.stream.peek()?;
          } else {
            return Err(LexerError::UnexpectedCharacter(ch));
          }
        } else if ch == '*' {
          if dash_found {
            self.stream.next()?;
            break;
          } else {
            sequence.push(ch);
          }
        } else {
          if dash_found {
            sequence.push('-');
            dash_found = false;
          };

          sequence.push(ch);

          self.stream.next()?;
          c = self.stream.peek()?;
        }
      } else {
        return Err(LexerError::UnexpectedEndOfStream);
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
      c = self.stream.peek()?.ok_or(LexerError::UnexpectedEndOfStream)?;
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

        if let Some(ch) = self.stream.peek()? {
          if ch == '*' {
            self.stream.next()?;
            return Ok(Some(self.create_low_level_sequence()?));
          }
        };

        Ok(Some(Token::Punctuation(c)))
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