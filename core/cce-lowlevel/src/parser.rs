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


use crate::{Lexer, Token, LexerError};
use thiserror::Error;


pub struct Parser {
  pub(crate) lexer: Lexer,
  pub(crate) peeked: Option<ParseNode>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNode {
  CommandCall(CommandCall),
  VariableAssignment(VariableAssignment)
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssignment {
  pub name: String,
  pub value: VariableValue
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
  Number(f64),
  HighLevelSequence(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandCall {
  HighLevelSequence(String)
}

#[derive(Debug, Error)]
pub enum ParserError {
  #[error("{0}")]
  LexerError(#[from] LexerError),
  #[error("Syntax Error: {0}")]
  SyntaxError(String)
}


impl Parser {
  pub fn new(lexer: Lexer) -> Self {
    Self {
      lexer,
      peeked: None
    }
  }

  fn parse_variable_assignment(&mut self) -> Result<VariableAssignment, ParserError> {
    let name = match self.lexer.next()? {
      Some(Token::Variable(name)) => name,
      _ => return Err(ParserError::SyntaxError("Expected identifier".to_string()))
    };

    match self.lexer.next()? {
      Some(Token::Punctuation('=')) => {},
      _ => return Err(ParserError::SyntaxError("Expected equals".to_string()))
    }

    let value = match self.lexer.next()? {
      Some(Token::Literal(n)) => VariableValue::Number(n.parse().or(Err(ParserError::SyntaxError("Expected number".to_string())))?),
      Some(Token::HighLevelSequence(hls)) => VariableValue::HighLevelSequence(hls),
      _ => return Err(ParserError::SyntaxError("Expected number".to_string()))
    };

    Ok(VariableAssignment {
      name,
      value
    })
  }

  pub fn next(&mut self) -> Result<Option<ParseNode>, ParserError> {
    if self.peeked.is_some() {
      let peeked = self.peeked.clone();
      self.peeked = None;
      return Ok(peeked);
    }

    let tok: Token = match self.lexer.peek()? {
      Some(t) => t,
      None => return Ok(None)
    };

    match tok {
      Token::HighLevelSequence(hls) => {
        self.lexer.next()?;
        Ok(Some(ParseNode::CommandCall(CommandCall::HighLevelSequence(hls))))
      },
      Token::Variable(_) => {
        Ok(Some(ParseNode::VariableAssignment(self.parse_variable_assignment()?)))
      },
      _ => {
        Err(ParserError::SyntaxError("Only high level sequences are supported at the moment".to_string()))
      }
    }
  }

  pub fn peek(&mut self) -> Result<Option<ParseNode>, ParserError> {
    if self.peeked.is_none() {
      self.peeked = self.next()?;
    }

    Ok(self.peeked.clone())
  }
}