use super::node::Node;

#[derive(Debug, PartialEq)]
pub struct Edge {
  pub id: [i64; 2],

  pub from: i64, // From node id
  pub to: i64,   // To node id

  pub distance: f64, // Edge distance
  pub eta: f64,
  pub tau: f64,
  pub velocity: f64,

  pub num: Option<f64>,
}

impl Edge {
  pub fn new(from: &Node, to: &Node) -> Self {
    let id = Self::build_id(from, to);
    let distance = Self::compute_distance(from, to);

    Self {
      id,
      from: from.id,
      to: to.id,

      distance,
      eta: 1.0 / distance,
      tau: 1.0,
      num: None,
      velocity: std::f64::INFINITY,
    }
  }

  fn build_id(from: &Node, to: &Node) -> [i64; 2] {
    [from.id, to.id]
  }

  fn compute_distance(from: &Node, to: &Node) -> f64 {
    from
      .coordinates
      .iter()
      .enumerate()
      .fold(0f64, |acc, (i, value)| {
        acc + (value - to.coordinates[i]).powi(2)
      })
      .sqrt()
  }

  pub fn set_num(&mut self, num: Option<f64>) {
    self.num = num;
  }

  pub fn add_pheromone(&mut self, quantity: f64) {
    self.tau += quantity;
  }

  pub fn evaporate_pheromone(&mut self, quantity: f64) {
    self.tau = 1f64.max(self.tau - quantity);
  }
}

// TESTS

#[cfg(test)]
mod edge_tests {
  use super::*;

  #[test]
  fn edge_new() {
    let from = Node::new(1, [0.0, 1.0, 0.0], None, true, true, [0, 0]);
    let to = Node::new(2, [1.0, 1.0, 0.0], None, true, true, [0, 0]);

    let edge = Edge::new(&from, &to);

    assert_eq!(
      edge,
      Edge {
        id: [1, 2],
        from: 1,
        to: 2,
        distance: 1.0,
        eta: 1.0,
        tau: 1.0,
        num: None,
        velocity: std::f64::INFINITY
      },
      "Edge initialization must be correct"
    );
  }

  #[test]
  fn set_num() {
    let from = Node::new(1, [0.0, 1.0, 0.0], None, true, true, [0, 0]);
    let to = Node::new(2, [1.0, 1.0, 0.0], None, true, true, [0, 0]);

    let mut edge = Edge::new(&from, &to);

    assert_eq!(edge.num, None);

    edge.set_num(Some(1.0));

    assert_eq!(edge.num, Some(1.0));
  }

  #[test]
  fn add_pheromone() {
    let from = Node::new(1, [0.0, 1.0, 0.0], None, true, true, [0, 0]);
    let to = Node::new(2, [1.0, 1.0, 0.0], None, true, true, [0, 0]);

    let mut edge = Edge::new(&from, &to);

    assert_eq!(edge.tau, 1.0);

    edge.add_pheromone(1.0);
    edge.add_pheromone(1.0);

    assert_eq!(edge.tau, 3.0);
  }

  #[test]
  fn evaporate_pheromone() {
    let from = Node::new(1, [0.0, 1.0, 0.0], None, true, true, [0, 0]);
    let to = Node::new(2, [1.0, 1.0, 0.0], None, true, true, [0, 0]);

    let mut edge = Edge::new(&from, &to);

    assert_eq!(edge.tau, 1.0);

    edge.evaporate_pheromone(1.0);

    assert_eq!(edge.tau, 1.0);

    edge.add_pheromone(2.0);
    edge.evaporate_pheromone(1.0);

    assert_eq!(edge.tau, 2.0);
  }
}
