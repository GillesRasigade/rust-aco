extern crate ants;
extern crate serde;

use serde::{Deserialize, Serialize};

use ants::edge::Edge;
use ants::node::Node;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeImport {
  pub coordinates: [f64; 3],

  pub next_id: Option<i64>,

  pub is_departure: bool,
  pub is_destination: bool,
  pub time_window: [i64; 2],
}

#[test]
fn parser_nodes() {
  let serialized = r#"[{
    "coordinates": [0, 0, 0],
    "is_departure": true,
    "is_destination": false,
    "time_window": [0, 1000000]
  }, {
    "coordinates": [1, 0, 0],
    "is_departure": false,
    "is_destination": true,
    "time_window": [0, 1000000]
  }]"#;

  let deserialized: Vec<NodeImport> = serde_json::from_str(&serialized).unwrap();

  println!("Deserialized: {:?}", deserialized);
  let mut vec_nodes: Vec<Node> = Vec::with_capacity(deserialized.len());

  for (i, node) in deserialized.iter().enumerate() {
    vec_nodes.push(Node::new(
      i as i64,
      node.coordinates,
      node.next_id,
      node.is_departure,
      node.is_destination,
      node.time_window,
    ));
  }

  let mut nodes: Vec<&Node> = Vec::new();
  for n in vec_nodes.iter() {
    nodes.push(&n);
  }

  println!("Nodes: {:?}", nodes);

  let edges = Edge::build_from_nodes(&nodes);

  println!("Edges: {:?}", edges);
}
