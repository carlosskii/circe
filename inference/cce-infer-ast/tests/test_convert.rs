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


use cce_infer_ast::*;
use cce_ast::{Parser, ParseNode};


#[test]
fn test_convert_basic() {
  let mut parser: Parser = Parser::from("print 'Hello, world!' to the console.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::Command(CommandNode {
      command: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Literal("Hello, world!".to_string()),
        CommandComponent::Keyword("to".to_string()),
        CommandComponent::Keyword("the".to_string()),
        CommandComponent::Keyword("console".to_string()),
      ],
      modifiers: vec![]
    })
  ];

  assert_eq!(ast_nodes, expected);
}

#[test]
fn test_convert_modifiers() {
  let mut parser: Parser = Parser::from("print 'Hello, world!' to the console | add a newline.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::Command(CommandNode {
      command: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Literal("Hello, world!".to_string()),
        CommandComponent::Keyword("to".to_string()),
        CommandComponent::Keyword("the".to_string()),
        CommandComponent::Keyword("console".to_string()),
      ],
      modifiers: vec![
        vec![
          CommandComponent::Keyword("add".to_string()),
          CommandComponent::Keyword("a".to_string()),
          CommandComponent::Keyword("newline".to_string())
        ]
      ]
    })
  ];

  assert_eq!(ast_nodes, expected);
}

#[test]
fn test_convert_multiple_commands() {
  let mut parser: Parser = Parser::from("print 'Hello, world!' to the console. print 'Goodbye, world!' to the console.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::Command(CommandNode {
      command: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Literal("Hello, world!".to_string()),
        CommandComponent::Keyword("to".to_string()),
        CommandComponent::Keyword("the".to_string()),
        CommandComponent::Keyword("console".to_string()),
      ],
      modifiers: vec![]
    }),
    ProgramNode::Command(CommandNode {
      command: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Literal("Goodbye, world!".to_string()),
        CommandComponent::Keyword("to".to_string()),
        CommandComponent::Keyword("the".to_string()),
        CommandComponent::Keyword("console".to_string()),
      ],
      modifiers: vec![]
    })
  ];

  assert_eq!(ast_nodes, expected);
}

#[test]
fn test_convert_howto() {
  let mut parser: Parser = Parser::from("howto print a string - write the string.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::HowTo(HowToNode {
      signature: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Keyword("a".to_string()),
        CommandComponent::Keyword("string".to_string()),
      ],
      body: vec![
        HowToCommand::HighLevel(CommandNode {
          command: vec![
            CommandComponent::Keyword("write".to_string()),
            CommandComponent::Keyword("the".to_string()),
            CommandComponent::Keyword("string".to_string()),
          ],
          modifiers: vec![]
        })
      ]
    })
  ];

  assert_eq!(ast_nodes, expected);
}

#[test]
fn test_convert_howto_modifiers() {
  let mut parser: Parser = Parser::from("howto print a string - write the string | add a newline.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::HowTo(HowToNode {
      signature: vec![
        CommandComponent::Keyword("print".to_string()),
        CommandComponent::Keyword("a".to_string()),
        CommandComponent::Keyword("string".to_string()),
      ],
      body: vec![
        HowToCommand::HighLevel(CommandNode {
          command: vec![
            CommandComponent::Keyword("write".to_string()),
            CommandComponent::Keyword("the".to_string()),
            CommandComponent::Keyword("string".to_string()),
          ],
          modifiers: vec![
            vec![
              CommandComponent::Keyword("add".to_string()),
              CommandComponent::Keyword("a".to_string()),
              CommandComponent::Keyword("newline".to_string())
            ]
          ]
        })
      ]
    })
  ];

  assert_eq!(ast_nodes, expected);
}

#[test]
fn test_convert_lowlevel() {
  let mut parser: Parser = Parser::from("howto write a string -* < call the system > *.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::HowTo(HowToNode {
      signature: vec![
        CommandComponent::Keyword("write".to_string()),
        CommandComponent::Keyword("a".to_string()),
        CommandComponent::Keyword("string".to_string()),
      ],
      body: vec![
        HowToCommand::LowLevel(vec![
          LowLevelCommand::CommandCall(CommandNode {
            command: vec![
              CommandComponent::Keyword("call".to_string()),
              CommandComponent::Keyword("the".to_string()),
              CommandComponent::Keyword("system".to_string()),
            ],
            modifiers: vec![]
          })
        ])
      ]
    })
  ];

  assert_eq!(ast_nodes, expected);
}

#[test]
fn test_convert_lowlevel_varassign() {
  let mut parser: Parser = Parser::from("howto convert that -* @VAR = < the result > *.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::HowTo(HowToNode {
      signature: vec![
        CommandComponent::Keyword("convert".to_string()),
        CommandComponent::Keyword("that".to_string()),
      ],
      body: vec![
        HowToCommand::LowLevel(vec![
          LowLevelCommand::VariableAssignment(LLVariableAssignment {
            name: "VAR".to_string(),
            value: LLVariableValue::CommandResult(CommandNode {
              command: vec![
                CommandComponent::Keyword("the".to_string()),
                CommandComponent::Keyword("result".to_string()),
              ],
              modifiers: vec![]
            })
          })
        ])
      ]
    })
  ];

  assert_eq!(ast_nodes, expected);
}

#[test]
fn test_convert_whatis() {
  let mut parser: Parser = Parser::from("whatis a string - a sequence of characters.");

  let mut parse_nodes: Vec<ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  }

  let ast_nodes: Vec<ProgramNode> = convert(parse_nodes);

  let expected: Vec<ProgramNode> = vec![
    ProgramNode::WhatIs(WhatIsNode {
      signature: vec![
        CommandComponent::Keyword("a".to_string()),
        CommandComponent::Keyword("string".to_string()),
      ],
      body: vec![
        CommandNode {
          command: vec![
            CommandComponent::Keyword("a".to_string()),
            CommandComponent::Keyword("sequence".to_string()),
            CommandComponent::Keyword("of".to_string()),
            CommandComponent::Keyword("characters".to_string()),
          ],
          modifiers: vec![]
        }
      ]
    })
  ];

  assert_eq!(ast_nodes, expected);
}