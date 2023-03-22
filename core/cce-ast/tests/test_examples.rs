use cce_ast::*;
use cce_stream::InputStream;
use std::io::Cursor;


#[test]
fn test_parser_example_helloworld() {
  let contents: Cursor<&[u8]> = Cursor::new(include_str!("./examples/hello.cce").as_bytes());
  let mut parser = Parser::new(Lexer::new(InputStream::new(Box::new(contents))));

  let expected_output = vec![
    ParseNode::Command(Command {
      components: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Literal("Hello, world!".to_string()),
        CommandComponent::Keyword("to".to_string()),
        CommandComponent::Keyword("the".to_string()),
        CommandComponent::Keyword("console".to_string())
      ],
      modifiers: vec![]
    }),
    ParseNode::HowToStatement(HowToStatement {
      signature: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Keyword("a".to_string()),
        CommandComponent::Keyword("string".to_string()),
        CommandComponent::Keyword("to".to_string()),
        CommandComponent::Keyword("the".to_string()),
        CommandComponent::Keyword("console".to_string())
      ],
      body: vec![
        Command {
          components: vec![
            CommandComponent::Keyword("write".to_string()),
            CommandComponent::Keyword("the".to_string()),
            CommandComponent::Keyword("string".to_string()),
            CommandComponent::Keyword("to".to_string()),
            CommandComponent::Keyword("stdout".to_string())
          ],
          modifiers: vec![vec![
              CommandComponent::Keyword("add".to_string()),
              CommandComponent::Keyword("a".to_string()),
              CommandComponent::Keyword("newline".to_string())
          ]]
        }
      ]
    }),
    ParseNode::WhatIsStatement(WhatIsStatement {
      signature: vec![
        CommandComponent::Keyword("stdout".to_string())
      ],
      body: vec![
        Command {
          components: vec![
            CommandComponent::Keyword("a".to_string()),
            CommandComponent::Keyword("file".to_string()),
            CommandComponent::Keyword("stream".to_string())
          ],
          modifiers: vec![]
        }
      ]
    })
  ];

  let mut output: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => output.push(node),
      None => break
    }
  };

  assert_eq!(output, expected_output);
}