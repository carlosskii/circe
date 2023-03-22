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