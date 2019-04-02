use super::ant::Ant;
use super::edge::Edge;

#[derive(Debug, PartialEq)]
pub struct Colony {
  pub alpha: f64,
  pub beta: f64,
  pub gamma: f64,
  pub rho: f64,
  pub q: f64,

  iteration: i64,
}

impl<'a> Colony {
  pub fn new(alpha: f64, beta: f64, gamma: f64, rho: f64, q: f64) -> Self {
    Self {
      alpha,
      beta,
      gamma,
      rho,
      q,

      iteration: 0,
    }
  }

  fn get_ants(iteration: i64, n_ants: i64) -> Vec<Ant<'a>> {
    let mut ants: Vec<Ant> = Vec::new();

    for i in 0..n_ants {
      let ant = Ant::new(iteration * n_ants + i);

      ants.push(ant);
    }

    ants
  }

  fn compute_probabilities(&mut self, edges: &mut Vec<Edge<'a>>) {
    for edge in edges {
      let num = self.gamma + edge.tau.powf(self.alpha) * edge.eta.powf(self.beta);

      edge.set_num(Some(num));
    }
  }

  fn increment_iteration(&mut self) {
    self.iteration += 1;
  }

  pub fn explore(&mut self, edges: &mut Vec<Edge<'a>>, n_ants: i64) {
    self.compute_probabilities(edges);

    let ants = Colony::get_ants(self.iteration, n_ants);

    for mut ant in ants {
      ant.explore(edges);

      dbg!(&ant);

      if ant.finished == true {
        for edge_id in ant.explored_edges {
          match edges.iter().position(|edge| edge.id == edge_id) {
            Some(index) => {
              let mut edge = edges[index];
              let tau = self.q / ant.distance;
              dbg!(tau);

              edge.add_pheromone(tau);
            }
            _ => {}
          }
        }
      }
    }

    self.increment_iteration();
  }
}

#[cfg(test)]
mod edge_tests {
  use super::*;

  #[test]
  fn colony_new() {
    let colony = Colony::new(1.0, 2.0, 3.0, 4.0, 5.0);

    assert_eq!(
      colony,
      Colony {
        alpha: 1.0,
        beta: 2.0,
        gamma: 3.0,
        rho: 4.0,
        q: 5.0,

        iteration: 0
      }
    );
  }
}
