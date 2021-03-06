extern crate ants;

use ants::colony::Colony;
use ants::edge::Edge;
use ants::node::Node;

#[test]
fn solve_simple_line() {
  // `from` is a departure:
  let from = Node::new(
    1,
    [0.0, 0.0, 0.0],
    None,
    true,
    false,
    [-std::i64::MAX, std::i64::MAX],
  );
  dbg!(&from);

  // `to` is a destination:
  let to = Node::new(
    2,
    [1.0, 0.0, 0.0],
    None,
    false,
    true,
    [-std::i64::MAX, std::i64::MAX],
  );
  dbg!(&to);

  let mut nodes: Vec<&Node> = Vec::new();
  nodes.push(&from);
  nodes.push(&to);

  // `colony` is defining the ants colony parameters applying to all ants
  // exploring the world.
  let mut colony = Colony::new(1.0, 1.0, 0.1, 0.1, 1.0);
  dbg!(&colony);

  let mut edges = Edge::build_from_nodes(&nodes);

  let mut ants = Colony::get_ants(0, 1);

  // edge.add_pheromone(1.0);
  for _ in 0..3 {
    colony.explore(&mut ants, &mut edges);
  }

  dbg!(&edges);

  assert_eq!(edges[0].tau, 1.8, "Single edge tau must be 2.0");
}
