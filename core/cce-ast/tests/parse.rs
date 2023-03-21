use cce_ast::InputStream;
use std::io::Cursor;

#[test]
fn test_stream() {
  let contents: Cursor<&[u8]> = Cursor::new(b"Hello, world!");
  let mut stream = InputStream::new(Box::new(contents));

  let next_char: char = stream.next().unwrap().unwrap();
  assert_eq!(next_char, 'H');

  let next_char: char = stream.next().unwrap().unwrap();
  assert_eq!(next_char, 'e');
}