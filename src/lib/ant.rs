pub struct Ant {
  id: i64,
}

impl Ant {
  pub fn new(id: i64) -> Self {
    Self { id }
  }

  /**
   * Returns the Ant id
   */
  pub fn get_id(self) -> i64 {
    self.id
  }

  /**
   * The ant is exploring the edges given in parameters
   */
  pub fn explore(self) {}

  /**
   * The ant is choosing one the available nodes in front of it.
   */
  pub fn choose(self, choice: i64) {
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
