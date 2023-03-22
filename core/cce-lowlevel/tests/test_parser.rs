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


use cce_lowlevel::*;
use cce_stream::InputStream;
use std::io::Cursor;


#[test]
fn test_parser_basic() {
  let contents: Cursor<&[u8]> = Cursor::new(b"< call the system >");
  let mut parser: Parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let expected: ParseNode = ParseNode::CommandCall(
    CommandCall::HighLevelSequence(" call the system ".to_string())
  );

  assert_eq!(parser.next().unwrap().unwrap(), expected);
  assert_eq!(parser.next().unwrap(), None);
}

#[test]
fn test_parser_peek() {
  let contents: Cursor<&[u8]> = Cursor::new(b"< call the system >");
  let mut parser: Parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));
  let expected: ParseNode = ParseNode::CommandCall(
    CommandCall::HighLevelSequence(" call the system ".to_string())
  );

  assert_eq!(parser.peek().unwrap().unwrap(), expected);
  assert_eq!(parser.next().unwrap().unwrap(), expected);

  assert_eq!(parser.peek().unwrap(), None);
  assert_eq!(parser.next().unwrap(), None);
}

#[test]
fn test_parser_varassign() {
  let contents: Cursor<&[u8]> = Cursor::new(b"@SYS0 = 1");
  let mut parser: Parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));
  let expected: ParseNode = ParseNode::VariableAssignment(
    VariableAssignment {
      name: "SYS0".to_string(),
      value: VariableValue::Number(1 as f64)
    }
  );

  assert_eq!(parser.next().unwrap().unwrap(), expected);
}

#[test]
fn test_parser_varassign_highlevel() {
  let contents: Cursor<&[u8]> = Cursor::new(b"@SYS0 = < the value to insert >");
  let mut parser: Parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let expected: ParseNode = ParseNode::VariableAssignment(
    VariableAssignment {
      name: "SYS0".to_string(),
      value: VariableValue::HighLevelSequence(" the value to insert ".to_string())
    }
  );

  assert_eq!(parser.next().unwrap().unwrap(), expected);
}

#[test]
fn test_parser_multiple() {
  let contents: Cursor<&[u8]> = Cursor::new(b"@SYS0 = 1\n@SYS1 = 2");
  let mut parser: Parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let expected1: ParseNode = ParseNode::VariableAssignment(
    VariableAssignment {
      name: "SYS0".to_string(),
      value: VariableValue::Number(1 as f64)
    }
  );

  let expected2: ParseNode = ParseNode::VariableAssignment(
    VariableAssignment {
      name: "SYS1".to_string(),
      value: VariableValue::Number(2 as f64)
    }
  );

  assert_eq!(parser.next().unwrap().unwrap(), expected1);
  assert_eq!(parser.next().unwrap().unwrap(), expected2);
  assert_eq!(parser.next().unwrap(), None);
}