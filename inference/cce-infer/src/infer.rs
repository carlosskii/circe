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


use cce_infer_ast::ProgramNode;

pub fn infer_pass(nodes: &Vec<ProgramNode>) -> (Vec<ProgramNode>, bool) {
  let changed: bool = false;
  let mut result: Vec<ProgramNode> = Vec::new();

  for node in nodes.iter() {
    result.push(node.clone());
  }

  return (result, changed);
}