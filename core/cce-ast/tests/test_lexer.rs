use cce_ast::{InputStream, Lexer, Token};
use std::io::Cursor;

#[test]
fn test_lexer_basic() {
  let contents: Cursor<&[u8]> = Cursor::new(b"howto hello world");
  let mut lexer = Lexer::new(InputStream::new(Box::new(contents)));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Keyword("howto".to_string()));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Identifier("hello".to_string()));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Identifier("world".to_string()));
}

#[test]
fn test_lexer_string() {
  let contents: Cursor<&[u8]> = Cursor::new(b"howto 'hello world'");
  let mut lexer = Lexer::new(InputStream::new(Box::new(contents)));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Keyword("howto".to_string()));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Literal("hello world".to_string()));
}

#[test]
fn test_lexer_punct() {
  let contents: Cursor<&[u8]> = Cursor::new(b"howto hello world\n- do it");
  let mut lexer = Lexer::new(InputStream::new(Box::new(contents)));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Keyword("howto".to_string()));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Identifier("hello".to_string()));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Identifier("world".to_string()));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Punctuation('-'));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Identifier("do".to_string()));

  let next_token = lexer.next().unwrap();
  assert_eq!(next_token, Token::Identifier("it".to_string()));
}