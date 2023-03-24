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


use std::io::{Read, Seek};

use thiserror::Error;

pub trait Streamable: Read + Seek {}

impl<T: Read + Seek> Streamable for T {
  // Empty
}

pub struct InputStream {
  pub(crate) stream: Box<dyn Streamable>,
  pub line: usize,
  pub column: usize,
  pub pos: usize,
  pub(crate) buf: [u8; 4],
  pub(crate) buf_len: usize,
  pub(crate) peeked: Option<char>
}

#[derive(Error, Debug)]
pub enum InputStreamError {
  #[error("Failed to read from stream")]
  ReadError(#[from] std::io::Error)
}

impl InputStream {
  pub fn new(stream: Box<dyn Streamable>) -> InputStream {
    InputStream {
      stream,
      line: 1,
      column: 1,
      pos: 0,
      buf: [0; 4],
      buf_len: 0,
      peeked: None
    }
  }

  fn get_next_char(&mut self) -> Result<Option<char>, InputStreamError> {
    loop {
      if self.buf_len == 4 { break };
      let mut buf: [u8; 1] = [0];
      match self.stream.read(&mut buf) {
        Ok(0) => break,
        Ok(_) => {
          self.buf[self.buf_len] = buf[0];
        }
        Err(e) => return Err(e.into())
      };
      self.buf_len += 1;
    };

    if self.buf_len == 0 { return Ok(None) };

    // Get char boundaries
    let mut char_len: usize = 1;
    if self.buf[0] & 0b1111_0000 == 0b1111_0000 { char_len = 4 }
    else if self.buf[0] & 0b1110_0000 == 0b1110_0000 { char_len = 3 }
    else if self.buf[0] & 0b1100_0000 == 0b1100_0000 { char_len = 2 };

    let mut char_buf: [u8; 4] = [0; 4];
    char_buf[0..4].copy_from_slice(&self.buf[0..4]);
    let c: char = std::str::from_utf8(&char_buf[..char_len])
      .or(Err(InputStreamError::ReadError(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))))?
      .chars().next()
      .ok_or(InputStreamError::ReadError(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")))?;

    // Shift buffer
    self.buf_len -= char_len;
    self.buf = [0; 4];
    self.buf[..4-char_len].copy_from_slice(&char_buf[char_len..]);

    Ok(Some(c))
  }

  pub fn next(&mut self) -> Result<Option<char>, InputStreamError> {
    if self.peeked.is_some() {
      let c = self.peeked;
      self.peeked = None;
      return Ok(c);
    };

    let c = self.get_next_char()?;
    match c {
      Some(c) => {
        self.pos += 1;
        if c == '\n' {
          self.line += 1;
          self.column = 1;
        } else {
          self.column += 1;
        }
        Ok(Some(c))
      },
      None => Ok(None)
    }
  }

  pub fn peek(&mut self) -> Result<Option<char>, InputStreamError> {
    if self.peeked.is_some() { return Ok(self.peeked) };

    let c = self.get_next_char()?;
    match c {
      Some(c) => {
        self.peeked = Some(c);
        Ok(Some(c))
      },
      None => Ok(None)
    }
  }
}