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
fn test_input_stream() {
  let mut stream = InputStream::new("Hello!");

  let next_char: char = stream.next().unwrap();
  assert_eq!(next_char, 'H');

  let next_char: char = stream.next().unwrap();
  assert_eq!(next_char, 'e');

  let next_char: char = stream.peek().unwrap();
  assert_eq!(next_char, 'l');

  let next_char: char = stream.next().unwrap();
  assert_eq!(next_char, 'l');

  let next_char: char = stream.next().unwrap();
  assert_eq!(next_char, 'l');

  let next_char: char = stream.peek().unwrap();
  assert_eq!(next_char, 'o');
}

#[test]
fn test_input_stream_none() {
  let mut stream = InputStream::new("");

  let next_char: Option<char> = stream.next();
  assert_eq!(next_char, None);
}