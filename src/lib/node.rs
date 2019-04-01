#[derive(Debug, PartialEq)]
pub struct Node {
  pub id: i64,
  pub coordinates: [f64; 3],

  pub next_id: Option<i64>,

  pub is_departure: bool,
  pub is_destination: bool,
  pub time_window: [i64; 2],
}

impl Node {
  pub fn new(
    id: i64,
    coordinates: [f64; 3],
    next_id: Option<i64>,
    is_departure: bool,
    is_destination: bool,
    time_window: [i64; 2],
  ) -> Self {
    Self {
      id,
      coordinates,
      next_id,
      is_departure,
      is_destination,
      time_window,
    }
  }
}

#[cfg(test)]
mod node_tests {
  use super::*;

  #[test]
  fn node_new() {
    let node = Node::new(1, [0.0, 1.0, 2.0], Some(1), true, true, [0, 0]);

    assert_eq!(node.id, 1, "Node id must be set to 1 (!= {})", node.id);
    assert_eq!(
      node.coordinates,
      [0.0, 1.0, 2.0],
      "Node coordinates must be set correctly"
    );
    assert_eq!(node.next_id, Some(1), "Node next_id must be set");
  }

}
