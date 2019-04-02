extern crate rand;

use rand::Rng;

use super::edge::Edge;
use super::node::Node;

#[derive(Debug, PartialEq)]
pub struct Ant<'a> {
  id: i64,

  // Explored nodes:
  explored_nodes: Vec<i64>,
  pub explored_edges: Vec<[i64; 2]>,

  // Current node and edge id:
  current_node: Option<&'a Node>,
  current_edge: Option<Edge<'a>>,

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

impl<'a> Ant<'a> {
  pub fn new(id: i64) -> Self {
    Self {
      id,

      explored_nodes: Vec::new(),
      explored_edges: Vec::new(),

      current_node: None,
      current_edge: None,

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

  pub fn start(&mut self, edges: &mut Vec<Edge<'a>>) {
    let mut i: i64 = 0;

    if self.current_node.is_some() {
      return;
    }

    loop {
      let num: usize = rand::thread_rng().gen_range(0, edges.len()) as usize;

      let edge = edges[num];

      if edge.from.is_departure == true {
        self.explored_nodes.push(edge.from.id);
        self.current_node = Some(edge.from);

        break;
      }

      i += 1;
      if i > 10000 {
        panic!("Failed to find a departure point");
      }
    }
  }

  fn move_along_current_edge(&mut self) {
    self.edge_time += 1;

    if self.edge_time > self.next_edge_duration_to_reach {
      return self.reach();
    }
  }

  fn reach(&mut self) {
    match self.current_edge {
      Some(current_edge) => {
        self.explored_nodes.push(current_edge.to.id);
        self.explored_edges.push(current_edge.id);

        self.distance += current_edge.distance;

        self.current_node = Some(current_edge.to.clone());
        self.current_edge = None;
      }
      _ => {
        panic!("A node is reached only if the ant is on an edge");
      }
    }
  }

  /**
   * The ant is exploring the edges given in parameters
   */
  pub fn explore(&mut self, edges: &mut Vec<Edge<'a>>) {
    self.exploration_time += 1;

    self.start(edges);

    if self.current_edge.is_some() {
      return self.move_along_current_edge();
    }

    let mut choices: Vec<Edge<'a>> = Vec::new();
    for edge in edges {
      if !self.explored_nodes.contains(&edge.to.id) {
        choices.push(edge.clone());
      }
    }

    dbg!(&choices);

    let next_id = match self.current_node {
      Some(node) => node.next_id,
      _ => None,
    };

    if next_id.is_some() {
      let next_id = next_id.unwrap();

      for choice in choices.iter() {
        if choice.from.id == next_id {
          return self.choose(*choice);
        }
      }
    }

    let mut available_choices: Vec<Edge<'a>> = Vec::new();
    for choice in choices.iter() {
      if choice.to.time_window[0] <= self.exploration_time
        && self.exploration_time <= choice.to.time_window[1]
      {
        available_choices.push(choice.clone());
      }
    }

    if available_choices.len() == 0 {
      self.finished = true;

      return;
    }

    let total = available_choices.iter().fold(0f64, |acc, value| {
      acc
        + match value.num {
          Some(num) => num,
          _ => 0.0,
        }
    });

    let probabilities = available_choices
      .iter()
      .map(|choice| match choice.num {
        Some(value) => value,
        _ => 0.0
      } / total)
      .collect::<Vec<f64>>();

    let choice = available_choices[self.choose_from_probabilities(probabilities)];

    return self.choose(choice);
  }

  fn choose_from_probabilities(&mut self, probabilities: Vec<f64>) -> usize {
    let n = rand::thread_rng().gen_range(0.0, 1.0);

    let mut i: usize = 0;
    let mut p = probabilities[i];

    loop {
      if (p == 1.0) | (n < p) {
        break;
      }

      i += 1;
      p += probabilities[i];
    }

    i
  }

  /**
   * The ant is choosing one the available nodes in front of it.
   */
  pub fn choose(&mut self, choice: Edge<'a>) {
    // debug!("[{}] Choosing edge {}", self.id, choice);
    self.current_edge = Some(choice);

    self.edge_time = 0;
    self.next_edge_duration_to_reach = (choice.distance / choice.velocity) as i64;
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
