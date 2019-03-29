extern crate ants;

use ants::ant::Ant;

fn main() {
  println!("Waking ants up...");

  let _ant = Ant::new(1);
}

#[cfg(test)]
mod main_tests {
  use super::*;

  /**
   * This test does not test a lot of things!
   */
  #[test]
  fn main_returns_ant() {
    main();
  }
}
