#[derive(Debug, PartialEq)]
pub struct Colony {
  pub alpha: f64,
  pub beta: f64,
  pub gamma: f64,
  pub rho: f64,
  pub q: f64,

  iteration: i64,
}

impl Colony {
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
