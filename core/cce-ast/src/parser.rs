use crate::lexer::{Lexer, Token, LexerError};

use thiserror::Error;

pub struct Parser {
  pub(crate) lexer: Lexer,
  pub(crate) peeked: Option<ParseNode>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNode {
  Command(Command),
  HowToStatement(HowToStatement),
  WhatIsStatement(WhatIsStatement)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
  pub components: Vec<CommandComponent>,
  pub modifiers: Vec<Vec<CommandComponent>>
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandComponent {
  Literal(String),
  Keyword(String)
}

#[derive(Debug, Clone, PartialEq)]
pub struct HowToStatement {
  pub signature: Vec<CommandComponent>,
  pub body: Vec<Command>
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhatIsStatement {
  pub signature: Vec<CommandComponent>,
  pub body: Vec<Command>
}


#[derive(Error, Debug)]
pub enum ParserError {
  #[error("{0}")]
  LexerError(#[from] LexerError),
  #[error("Syntax error: {0}")]
  SyntaxError(String),
  #[error("Internal error: {0}")]
  InternalError(String)
}


impl Parser {
  pub fn new(lexer: Lexer) -> Parser {
    Parser {
      lexer,
      peeked: None
    }
  }

  fn parse_vec_command_component(&mut self) -> Result<Vec<CommandComponent>, ParserError> {
    let mut components: Vec<CommandComponent> = Vec::new();

    let mut tok: Option<Token> = self.lexer.peek()?;

    loop {
      if let Some(token) = tok.clone() {
        match token {
          Token::Identifier(ident) => {
            components.push(CommandComponent::Keyword(ident));
          },
          Token::Keyword(kw) => {
            components.push(CommandComponent::Keyword(kw));
          },
          Token::Literal(lit) => {
            components.push(CommandComponent::Literal(lit));
          },
          Token::Punctuation(_) => {
            break;
          },
          Token::LowLevelSequence(_) => {
            return Err(ParserError::SyntaxError("Low level sequences are not allowed here".to_string()));
          },
        }

        self.lexer.next()?;
        tok = self.lexer.peek()?;
      } else {
        break;
      }
    };

    Ok(components)
  }

  fn parse_command(&mut self) -> Result<Command, ParserError> {
    let components: Vec<CommandComponent> = self.parse_vec_command_component()?;
    let mut modifiers: Vec<Vec<CommandComponent>> = Vec::new();

    let mut tok: Option<Token> = self.lexer.peek()?;

    loop {
      if let Some(token) = tok {
        match token {
          Token::Punctuation(punc) => {
            match punc {
              '|' => {
                self.lexer.next()?;
                modifiers.push(self.parse_vec_command_component()?);
                tok = self.lexer.peek()?;
              },
              '-' => {
                break;
              },
              '.' => {
                self.lexer.next()?;
                break;
              },
              _ => {
                return Err(ParserError::SyntaxError("Expected '|'".to_string()));
              }
            }
          },
          _ => {
            break;
          }
        }
      } else {
        break;
      }
    };

    Ok(Command {
      components,
      modifiers
    })
  }

  fn parse_howto_statement(&mut self) -> Result<HowToStatement, ParserError> {
    let signature: Vec<CommandComponent> = self.parse_vec_command_component()?;
    let mut body: Vec<Command> = Vec::new();

    let mut tok: Option<Token> = self.lexer.peek()?;

    match tok {
      Some(Token::Punctuation(punc)) => {
        if punc == '-' {
          self.lexer.next()?;
        } else {
          return Err(ParserError::SyntaxError("Expected '-'".to_string()));
        }
      },
      _ => {
        return Err(ParserError::SyntaxError("Expected '-'".to_string()));
      }
    };

    loop {
      let cmd: Command = self.parse_command()?;
      body.push(cmd);

      tok = self.lexer.peek()?;
      match tok {
        Some(Token::Punctuation('-')) => {
          self.lexer.next()?;
        },
        Some(Token::Punctuation(_)) => {
          return Err(ParserError::SyntaxError("Expected '-'".to_string()));
        },
        _ => { break }
      }
    };

    Ok(HowToStatement {
      signature,
      body
    })
  }

  fn parse_whatis_statement(&mut self) -> Result<WhatIsStatement, ParserError> {
    let signature: Vec<CommandComponent> = self.parse_vec_command_component()?;
    let mut body: Vec<Command> = Vec::new();

    let mut tok: Option<Token> = self.lexer.peek()?;

    match tok {
      Some(Token::Punctuation(punc)) => {
        if punc == '-' {
          self.lexer.next()?;
        } else {
          return Err(ParserError::SyntaxError("Expected '-'".to_string()));
        }
      },
      _ => {
        return Err(ParserError::SyntaxError("Expected '-'".to_string()));
      }
    };

    loop {
      let cmd: Command = self.parse_command()?;
      body.push(cmd);

      tok = self.lexer.peek()?;
      match tok {
        Some(Token::Punctuation('-')) => {
          self.lexer.next()?;
        },
        Some(Token::Punctuation(_)) => {
          return Err(ParserError::SyntaxError("Expected '-'".to_string()));
        },
        _ => { break }
      }
    };

    Ok(WhatIsStatement {
      signature,
      body
    })
  }

  pub fn next(&mut self) -> Result<Option<ParseNode>, ParserError> {
    if self.peeked.is_some() {
      let peeked: Option<ParseNode> = self.peeked.clone();
      self.peeked = None;
      return Ok(peeked);
    }

    let token: Token = match self.lexer.peek()? {
      Some(tok) => tok,
      None => {
        return Ok(None);
      }
    };

    match token {
      Token::Keyword(kw) => {
        match kw.as_str() {
          "howto" => {
            self.lexer.next()?;
            let howto: HowToStatement = self.parse_howto_statement()?;
            Ok(Some(ParseNode::HowToStatement(howto)))
          },
          "whatis" => {
            self.lexer.next()?;
            let whatis: WhatIsStatement = self.parse_whatis_statement()?;
            Ok(Some(ParseNode::WhatIsStatement(whatis)))
          },
          _ => {
            Err(ParserError::InternalError("Unexpected keyword".to_string()))
          }
        }
      },
      Token::Identifier(_) => {
        Ok(Some(ParseNode::Command(self.parse_command()?)))
      },
      _ => {
        Err(ParserError::InternalError("Unexpected token".to_string()))
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