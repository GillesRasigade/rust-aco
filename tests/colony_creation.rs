extern crate ants;

use ants::colony::Colony;

#[test]
fn colony_creation() {
  let colony = Colony::new(1.0, 2.0, 3.0, 4.0, 5.0);

  assert_eq!(colony.alpha, 1.0, "Colony instanciation must be valid");
}