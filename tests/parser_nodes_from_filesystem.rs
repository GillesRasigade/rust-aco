extern crate ants;
extern crate serde;

use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

use serde::{Deserialize, Serialize};

use ants::colony::Colony;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeExport {
  pub id: [i64; 2],

  pub tau: f64,
}

#[test]
fn parser_nodes_from_filesystem() {
  let mut file = File::open("./tests/circle.json").unwrap();
  let mut serialized = String::new();
  file.read_to_string(&mut serialized).unwrap();

  let deserialized: Vec<NodeImport> = serde_json::from_str(&serialized).unwrap();

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

  let mut edges = Edge::build_from_nodes(&nodes);

  let mut colony = Colony::new(1.0, 1.0, 0.1, 0.05, 1.0);

  let mut ants = Colony::get_ants(0, 100);

  for _ in 0..1000 {
    colony.explore(&mut ants, &mut edges);
  }

  edges.sort_by(|a, b| b.tau.partial_cmp(&a.tau).unwrap());

  let path: Vec<[i64; 2]> = Vec::new();

  let mut edgeToSerialize: Vec<EdgeExport> = Vec::new();

  dbg!(&ants[0].explored_nodes);
  let mut last_tau: f64 = 0.0;
  for edge in edges.iter() {
    println!(
      "{:?} - d={:?} - from={:?} -> to={:?}: {:?}",
      edge.id, edge.distance, edge.from.coordinates, edge.to.coordinates, edge.tau
    );

    edgeToSerialize.push(EdgeExport {
      id: edge.id,
      tau: edge.tau,
    });
  }

  match edges.iter().position(|edge| edge.id[0] == 0) {
    Some(index) => {}
    _ => {}
  }

  let serialized = serde_json::to_string(&edgeToSerialize).unwrap();
  println!("{:?}", serialized);

  let mut file = File::create("./tests/result.json").unwrap();
  file.write(serialized.as_bytes()).unwrap();
  // file.write_all(&serialized).unwrap();
  // fs::write("./tests/result.json", serialized).unwrap();
}
