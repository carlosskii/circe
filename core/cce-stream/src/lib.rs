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


use thiserror::Error;

pub struct InputStream<'s> {
  pub(crate) data: &'s str,
  pub line: usize,
  pub column: usize,
  pub pos: usize
}

#[derive(Error, Debug)]
pub enum InputStreamError {
  #[error("Failed to read from stream")]
  ReadError(#[from] std::io::Error)
}

impl<'s> InputStream<'s> {
  pub fn new(data: &'s str) -> Self {
    InputStream {
      data,
      line: 1,
      column: 1,
      pos: 0
    }
  }

  pub fn peek(&self) -> Option<char> {
    self.data.chars().next()
  }

  pub fn peek_n(&self, n: usize) -> Option<char> {
    self.data.chars().nth(n)
  }
}

impl<'s> Iterator for InputStream<'s> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let c = self.peek()?;

    self.pos += 1;
    self.column += 1;

    if c == '\n' {
      self.line += 1;
      self.column = 1;
    }

    self.data = &self.data[c.len_utf8()..];

    Some(c)
  }
}
