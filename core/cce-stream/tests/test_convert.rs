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

#[test]
fn test_convert_string() {
  let mut istream: InputStream = InputStream::from("Hello!".to_string());

  assert_eq!(istream.next().unwrap().unwrap(), 'H');
  assert_eq!(istream.next().unwrap().unwrap(), 'e');
  assert_eq!(istream.next().unwrap().unwrap(), 'l');
  assert_eq!(istream.next().unwrap().unwrap(), 'l');
  assert_eq!(istream.next().unwrap().unwrap(), 'o');
  assert_eq!(istream.next().unwrap().unwrap(), '!');
}

#[test]
fn test_convert_string_slice() {
  let mut istream: InputStream = InputStream::from("Hello!");

  assert_eq!(istream.next().unwrap().unwrap(), 'H');
  assert_eq!(istream.next().unwrap().unwrap(), 'e');
  assert_eq!(istream.next().unwrap().unwrap(), 'l');
  assert_eq!(istream.next().unwrap().unwrap(), 'l');
  assert_eq!(istream.next().unwrap().unwrap(), 'o');
  assert_eq!(istream.next().unwrap().unwrap(), '!');
}

#[test]
fn test_convert_byte_slice() {
  let contents: &[u8] = b"Hello!";
  let mut istream: InputStream = InputStream::from(contents);

  assert_eq!(istream.next().unwrap().unwrap(), 'H');
  assert_eq!(istream.next().unwrap().unwrap(), 'e');
  assert_eq!(istream.next().unwrap().unwrap(), 'l');
  assert_eq!(istream.next().unwrap().unwrap(), 'l');
  assert_eq!(istream.next().unwrap().unwrap(), 'o');
  assert_eq!(istream.next().unwrap().unwrap(), '!');
}