use cce_ast::ParseNode;


pub fn infer_pass(nodes: &Vec<ParseNode>) -> (Vec<ParseNode>, bool) {
  let mut changed: bool = false;
  let mut result: Vec<ParseNode> = Vec::new();

  return (result, changed);
}