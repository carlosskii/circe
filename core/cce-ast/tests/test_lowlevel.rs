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


use cce_ast::*;
use cce_lowlevel as low;
use cce_stream::InputStream;
use std::io::Cursor;


#[test]
fn test_lowlevel_basic() {
  let content: Cursor<&[u8]> = Cursor::new(b"howto run -* < learn to walk > *");
  let mut parser: Parser = Parser::new(Lexer::new(InputStream::new(Box::new(content))));

  let expected: ParseNode = ParseNode::HowToStatement(HowToStatement {
    signature: vec![
      CommandComponent::Keyword("run".to_string())
    ],
    body: vec![
      HowToCommand::LowLevel(vec![
        low::ParseNode::CommandCall(
          low::CommandCall::HighLevelSequence(" learn to walk ".to_string())
        )
      ])
    ]
  });

  assert_eq!(parser.next().unwrap().unwrap(), expected);
  assert_eq!(parser.next().unwrap(), None);
}