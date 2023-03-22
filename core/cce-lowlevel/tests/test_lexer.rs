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


use cce_lowlevel::{Lexer, Token};
use cce_stream::InputStream;
use std::io::Cursor;


#[test]
fn test_lexer_basic() {
  let contents: Cursor<&[u8]> = Cursor::new(b"@SYSREG0 = 1");
  let mut lexer = Lexer::new(InputStream::new(Box::new(contents)));

  assert_eq!(lexer.next().unwrap().unwrap(), Token::Variable("SYSREG0".to_string()));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Punctuation('='));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Literal("1".to_string()));
}

#[test]
fn test_lexer_variable() {
  let contents: Cursor<&[u8]> = Cursor::new(b"@SYSREG0 = @SYSREG1");
  let mut lexer = Lexer::new(InputStream::new(Box::new(contents)));

  assert_eq!(lexer.next().unwrap().unwrap(), Token::Variable("SYSREG0".to_string()));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Punctuation('='));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Variable("SYSREG1".to_string()));
}

#[test]
fn test_lexer_numbers() {
  let contents: Cursor<&[u8]> = Cursor::new(b"12 34 567 = @SYSREG1");
  let mut lexer = Lexer::new(InputStream::new(Box::new(contents)));

  assert_eq!(lexer.next().unwrap().unwrap(), Token::Literal("12".to_string()));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Literal("34".to_string()));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Literal("567".to_string()));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Punctuation('='));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Variable("SYSREG1".to_string()));
}

#[test]
fn test_lexer_highlevel_sequence() {
  let contents: Cursor<&[u8]> = Cursor::new(b"@SYSREG0 = < the value to insert >");
  let mut lexer = Lexer::new(InputStream::new(Box::new(contents)));

  assert_eq!(lexer.next().unwrap().unwrap(), Token::Variable("SYSREG0".to_string()));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::Punctuation('='));
  assert_eq!(lexer.next().unwrap().unwrap(), Token::HighLevelSequence(" the value to insert ".to_string()));
}