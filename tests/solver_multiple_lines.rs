extern crate ants;

use ants::colony::Colony;
use ants::edge::Edge;
use ants::node::Node;

#[test]
fn solve_multiple_lines() {
  let a = Node::new(
    0,
    [0.0, 0.0, 0.0],
    None,
    true,
    false,
    [-std::i64::MAX, std::i64::MAX],
  );
  let b = Node::new(
    1,
    [1.0, 0.0, 0.0],
    None,
    false,
    true,
    [-std::i64::MAX, std::i64::MAX],
  );
  let c = Node::new(
    2,
    [2.0, 0.0, 0.0],
    None,
    false,
    true,
    [-std::i64::MAX, std::i64::MAX],
  );
  let d = Node::new(
    3,
    [3.0, 0.0, 0.0],
    None,
    false,
    true,
    [-std::i64::MAX, std::i64::MAX],
  );
  let nodes: Vec<&Node> = vec![&a, &b, &c, &d];

  // `colony` is defining the ants colony parameters applying to all ants
  // exploring the world.
  let mut colony = Colony::new(1.0, 1.0, 0.1, 0.05, 1.0);
  dbg!(&colony);

  let mut edges = Edge::build_from_nodes(&nodes);

  let mut ants = Colony::get_ants(0, 10);

  // edge.add_pheromone(1.0);
  for _ in 0..100 {
    colony.explore(&mut ants, &mut edges);
  }

  dbg!(&ants[0].explored_nodes);
  for edge in edges {
    println!(
      "{:?} - d={:?} - from={:?} -> to={:?}: {:?}",
      edge.id, edge.distance, edge.from.coordinates, edge.to.coordinates, edge.tau
    );
  }
}
