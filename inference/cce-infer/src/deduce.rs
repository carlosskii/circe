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


use cce_ast::ParseNode;
use crate::infer::infer_pass;

pub struct Deducer {
  pub(crate) nodes: Vec<ParseNode>
}


impl Deducer {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new()
    }
  }

  pub fn add_node(&mut self, node: ParseNode) {
    self.nodes.push(node);
  }

  fn full_infer(&self) -> Vec<ParseNode> {
    let mut result: Vec<ParseNode> = self.nodes.clone();

    loop {
      let (new_nodes, changed) = infer_pass(&result);
      if !changed {
        break;
      }
      result = new_nodes;
    };

    result
  }

  fn full_compile<'a>(&self, nodes: Vec<ParseNode>) -> Box<[u8]> {
    let generated: Vec<u8> = Vec::new();
    generated.into_boxed_slice()
  }

  pub fn deduce<'a>(&self) -> Box<[u8]> {
    let infer_nodes: Vec<ParseNode> = self.full_infer();
    self.full_compile(infer_nodes)
  }
}