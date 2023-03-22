use cce_stream::InputStream;
use std::io::Cursor;

#[test]
fn test_input_stream() {
  let contents: Cursor<&[u8]> = Cursor::new(b"Hello, world!");
  let mut stream = InputStream::new(Box::new(contents));

  let next_char: char = stream.next().unwrap().unwrap();
  assert_eq!(next_char, 'H');

  let next_char: char = stream.next().unwrap().unwrap();
  assert_eq!(next_char, 'e');

  let next_char: char = stream.peek().unwrap().unwrap();
  assert_eq!(next_char, 'l');

  let next_char: char = stream.next().unwrap().unwrap();
  assert_eq!(next_char, 'l');

  let next_char: char = stream.next().unwrap().unwrap();
  assert_eq!(next_char, 'l');

  let next_char: char = stream.peek().unwrap().unwrap();
  assert_eq!(next_char, 'o');
}

#[test]
fn test_input_stream_none() {
  let contents: Cursor<&[u8]> = Cursor::new(b"");
  let mut stream = InputStream::new(Box::new(contents));

  let next_char: Option<char> = stream.next().unwrap();
  assert_eq!(next_char, None);
}