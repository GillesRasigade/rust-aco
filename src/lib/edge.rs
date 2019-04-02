use super::node::Node;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge<'a> {
  pub id: [i64; 2],

  pub from: &'a Node,
  pub to: &'a Node,

  pub distance: f64, // Edge distance
  pub eta: f64,
  pub tau: f64,
  pub velocity: f64,

  pub num: Option<f64>,
}

impl<'a> Edge<'a> {
  pub fn new(from: &'a Node, to: &'a Node) -> Self {
    let id = Self::build_id(from, to);
    let distance = Self::compute_distance(from, to);

    Self {
      id,
      from,
      to,

      distance,
      eta: 1.0 / distance,
      tau: 1.0,
      num: None,
      velocity: std::f64::INFINITY,
    }
  }

  /**
   * Build edges from the nodes definitions.
   */
  pub fn build_from_nodes(nodes: &Vec<&'a Node>) -> Vec<Edge<'a>> {
    let mut edges: Vec<Edge<'a>> = Vec::new();
    dbg!(&edges);

    for from in nodes {
      for to in nodes {
        if dbg!(from.id == to.id) {
          dbg!("Skipping: same node");
          continue;
        }

        match dbg!(to.is_destination) {
          true => {}
          false => {
            dbg!("Skipping: `to` node is not a destination");
            continue;
          }
        }

        match dbg!(from.next_id) {
          Some(next_id) => {
            if next_id != to.id {
              dbg!("Skipping: from has next and to does not match");
              continue;
            }
          }
          None => {}
        }

        dbg!(from.id);
        dbg!(to.id);

        let edge: Edge<'a> = Edge::new(from, to);

        edges.push(edge);
      }
    }

    dbg!(&edges);

    edges
  }

  fn build_id(from: &'a Node, to: &'a Node) -> [i64; 2] {
    [from.id, to.id]
  }

  fn compute_distance(from: &'a Node, to: &'a Node) -> f64 {
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
        from: &from,
        to: &to,
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
