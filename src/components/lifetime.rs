use specs::{Component, VecStorage};

use std::time::Duration;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct LifetimeComponent {
  pub duration: Duration,
}
