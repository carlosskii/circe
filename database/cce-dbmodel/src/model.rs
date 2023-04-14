
/****************************************
* Node
****************************************/
pub trait Node {
  fn get_id(&self) -> u64;
}

/****************************************
* Database
****************************************/
pub struct Database<N: Node> {
  pub nodes: Vec<N>
}

impl<N: Node> Database<N> {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new()
    }
  }

  pub fn add_node(&mut self, node: N) {
    self.nodes.push(node);
  }

  pub fn get_node(&self, id: u64) -> Option<&N> {
    self.nodes.iter().find(|node| node.get_id() == id)
  }
}