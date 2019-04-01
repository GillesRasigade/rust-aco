extern crate rand;

use rand::Rng;

use super::edge::Edge;

#[derive(Debug, PartialEq)]
pub struct Ant {
  id: i64,

  // Explored nodes:
  explored_node_ids: Vec<i64>,

  // Current node and edge id:
  current_node_id: Option<i64>,
  current_edge_id: Option<[i64; 2]>,

  // Next edge id:
  next_edge_id: Option<i64>,

  // Total explored distance:
  pub distance: f64,

  // Does the ant finished its exploration:
  pub finished: bool,

  // Exploration time:
  exploration_time: i64,

  // Edge exploration time:
  edge_time: i64,
  next_edge_duration_to_reach: i64,
}

impl Ant {
  pub fn new(id: i64) -> Self {
    Self {
      id,

      explored_node_ids: Vec::new(),

      current_node_id: None,
      current_edge_id: None,
      next_edge_id: None,

      distance: 0.0,
      finished: false,
      exploration_time: 0,

      edge_time: 0,
      next_edge_duration_to_reach: 0,
    }
  }

  /**
   * Returns the Ant id
   */
  pub fn get_id(self) -> i64 {
    self.id
  }

  pub fn start(&mut self, edges: &mut Vec<Edge>) {
    let mut i: i64 = 0;

    if self.current_node_id.is_some() {
      return;
    }

    // loop {
    //   let num: usize = rand::thread_rng().gen_range(0, edges.len()) as usize;

    //   let edge = edges[num];

    //   if edge.from.is_departure == true {
    //     self.explored_node_ids.push(edge.id[0]);
    //     self.current_node = Some(edge.from.clone());

    //     break;
    //   }

    //   i += 1;
    //   if i > 10000 {
    //     panic!("Failed to find a departure point");
    //   }
    // }
  }

  /**
   * The ant is exploring the edges given in parameters
   */
  pub fn explore(&mut self, nodes: &Vec<&Node>, edges: &mut Vec<Edge>) {
    self.exploration_time += 1;
  }

  /**
   * The ant is choosing one the available nodes in front of it.
   */
  pub fn choose(self) {
    // debug!("[{}] Choosing edge {}", self.id, choice);
  }
}

#[cfg(test)]
mod ant_tests {
  use super::*;

  #[test]
  fn ant_new_id() {
    let ant = Ant::new(1);
    let ant_id = ant.get_id();

    assert_eq!(ant_id, 1, "Ant id must be 1. 1 != {}", ant_id);
  }
}

#[cfg(test)]
mod choose_tests {
  use super::*;

  #[test]
  fn ant_choose() {}
}
