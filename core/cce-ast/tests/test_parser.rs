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
use cce_stream::InputStream;
use std::io::Cursor;


#[test]
fn test_parser_basic() {
  let contents: Cursor<&[u8]> = Cursor::new(b"say hello world");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::Command(Command {
    components: vec![
      CommandComponent::Keyword("say".to_string()),
      CommandComponent::Keyword("hello".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    modifiers: vec![]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_literal() {
  let contents: Cursor<&[u8]> = Cursor::new(b"say 'hello world'");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::Command(Command {
    components: vec![
      CommandComponent::Keyword("say".to_string()),
      CommandComponent::Literal("hello world".to_string())
    ],
    modifiers: vec![]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_modifier() {
  let contents: Cursor<&[u8]> = Cursor::new(b"say hello world | say hello world");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::Command(Command {
    components: vec![
      CommandComponent::Keyword("say".to_string()),
      CommandComponent::Keyword("hello".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    modifiers: vec![
      vec![
        CommandComponent::Keyword("say".to_string()),
        CommandComponent::Keyword("hello".to_string()),
        CommandComponent::Keyword("world".to_string())
      ]
    ]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_modifier_multiple() {
  let contents: Cursor<&[u8]> = Cursor::new(b"say hello world | say hello world | say hello world");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::Command(Command {
    components: vec![
      CommandComponent::Keyword("say".to_string()),
      CommandComponent::Keyword("hello".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    modifiers: vec![
      vec![
        CommandComponent::Keyword("say".to_string()),
        CommandComponent::Keyword("hello".to_string()),
        CommandComponent::Keyword("world".to_string())
      ],
      vec![
        CommandComponent::Keyword("say".to_string()),
        CommandComponent::Keyword("hello".to_string()),
        CommandComponent::Keyword("world".to_string())
      ]
    ]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_howto() {
  let contents: Cursor<&[u8]> = Cursor::new(b"howto say hello world - say hello world | do not say goodbye");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::HowToStatement(HowToStatement {
    signature: vec![
      CommandComponent::Keyword("say".to_string()),
      CommandComponent::Keyword("hello".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    body: vec![
      Command {
        components: vec![
          CommandComponent::Keyword("say".to_string()),
          CommandComponent::Keyword("hello".to_string()),
          CommandComponent::Keyword("world".to_string())
        ],
        modifiers: vec![
          vec![
            CommandComponent::Keyword("do".to_string()),
            CommandComponent::Keyword("not".to_string()),
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("goodbye".to_string())
          ]
        ]
      }
    ]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_howto_multiple() {
  let contents: Cursor<&[u8]> = Cursor::new(b"howto say hello world - say hello world | do not say goodbye - say hello world again");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::HowToStatement(HowToStatement {
    signature: vec![
      CommandComponent::Keyword("say".to_string()),
      CommandComponent::Keyword("hello".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    body: vec![
      Command {
        components: vec![
          CommandComponent::Keyword("say".to_string()),
          CommandComponent::Keyword("hello".to_string()),
          CommandComponent::Keyword("world".to_string())
        ],
        modifiers: vec![
          vec![
            CommandComponent::Keyword("do".to_string()),
            CommandComponent::Keyword("not".to_string()),
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("goodbye".to_string())
          ]
        ]
      },
      Command {
        components: vec![
          CommandComponent::Keyword("say".to_string()),
          CommandComponent::Keyword("hello".to_string()),
          CommandComponent::Keyword("world".to_string()),
          CommandComponent::Keyword("again".to_string())
        ],
        modifiers: vec![]
      }
    ]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_whatis() {
  let contents: Cursor<&[u8]> = Cursor::new(b"whatis the world - a planet | in the universe");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::WhatIsStatement(WhatIsStatement {
    signature: vec![
      CommandComponent::Keyword("the".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    body: vec![
      Command {
        components: vec![
          CommandComponent::Keyword("a".to_string()),
          CommandComponent::Keyword("planet".to_string())
        ],
        modifiers: vec![
          vec![
            CommandComponent::Keyword("in".to_string()),
            CommandComponent::Keyword("the".to_string()),
            CommandComponent::Keyword("universe".to_string())
          ]
        ]
      }
    ]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_whatis_multiple() {
  let contents: Cursor<&[u8]> = Cursor::new(b"whatis the world - a planet | in the universe - a planet in the solar system");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::WhatIsStatement(WhatIsStatement {
    signature: vec![
      CommandComponent::Keyword("the".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    body: vec![
      Command {
        components: vec![
          CommandComponent::Keyword("a".to_string()),
          CommandComponent::Keyword("planet".to_string())
        ],
        modifiers: vec![
          vec![
            CommandComponent::Keyword("in".to_string()),
            CommandComponent::Keyword("the".to_string()),
            CommandComponent::Keyword("universe".to_string())
          ]
        ]
      },
      Command {
        components: vec![
          CommandComponent::Keyword("a".to_string()),
          CommandComponent::Keyword("planet".to_string()),
          CommandComponent::Keyword("in".to_string()),
          CommandComponent::Keyword("the".to_string()),
          CommandComponent::Keyword("solar".to_string()),
          CommandComponent::Keyword("system".to_string())
        ],
        modifiers: vec![]
      }
    ]
  });

  assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_howto_multiple_nomod() {
  let contents: Cursor<&[u8]> = Cursor::new(b"howto say hello world - say hello world - say hello world again");
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let next_node: ParseNode = parser.next().unwrap().unwrap();
  let expected_node: ParseNode = ParseNode::HowToStatement(HowToStatement {
    signature: vec![
      CommandComponent::Keyword("say".to_string()),
      CommandComponent::Keyword("hello".to_string()),
      CommandComponent::Keyword("world".to_string())
    ],
    body: vec![
      Command {
        components: vec![
          CommandComponent::Keyword("say".to_string()),
          CommandComponent::Keyword("hello".to_string()),
          CommandComponent::Keyword("world".to_string())
        ],
        modifiers: vec![]
      },
      Command {
        components: vec![
          CommandComponent::Keyword("say".to_string()),
          CommandComponent::Keyword("hello".to_string()),
          CommandComponent::Keyword("world".to_string()),
          CommandComponent::Keyword("again".to_string())
        ],
        modifiers: vec![]
      }
    ]
  });

  assert_eq!(next_node, expected_node);
}