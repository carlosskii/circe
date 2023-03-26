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


use cce_infer::Deducer;
use cce_infer_ast::{convert, ProgramNode};
use cce_ast as ast;


#[test]
fn test_infer_basic() {
  let mut parser: ast::Parser = ast::Parser::from("print 'Hello, world!' to the console.");

  let mut parse_nodes: Vec<ast::ParseNode> = Vec::new();
  loop {
    match parser.next().unwrap() {
      Some(node) => parse_nodes.push(node),
      None => break
    }
  };

  let infer_ast: Vec<ProgramNode> = convert(parse_nodes);

  let mut deducer: Deducer = Deducer::new();
  for node in &infer_ast {
    deducer.add_node(node.clone());
  }

  let result: Vec<ProgramNode> = deducer.deduce();

  assert_eq!(result, infer_ast);
}